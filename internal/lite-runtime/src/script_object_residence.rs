use crate::global_script_object::ScriptObject;
use crate::script_object::Lang;
use crate::script_object::NodeScriptObject;
use crate::script_object::ScriptFieldValue;
use convert_case::Case;
use convert_case::Casing;
use fyrox::core::log::Log;
use fyrox::core::pool::Handle;
use fyrox::core::visitor::Visit;
use fyrox::core::visitor::VisitError;
use fyrox::core::visitor::VisitResult;
use fyrox::core::visitor::Visitor;
use fyrox::core::Uuid;
use fyrox::scene::node::Node;
use std::fmt::Debug;
use std::fmt::Formatter;

/// Initially, when script is loaded from file (scene or save game), it's in "packed" mode.
/// First time this script receives `on_update` callback, it's converted to "unpacked", by
/// transfering state into UserData managed by Lua VM. Thoughm serialization should work fine,
/// because Visit is implemented in both modes.
pub enum ScriptResidence<T: Lang> {
    Packed(NodeScriptObject<T>),
    Unpacked(T::UnpackedScriptObject),
}

impl<T: Lang> ScriptResidence<T> {
    pub fn is_packed(&self) -> bool {
        match self {
            ScriptResidence::Packed(_) => true,
            ScriptResidence::Unpacked(_) => false,
        }
    }

    pub fn inner_unpacked(self: &ScriptResidence<T>) -> Option<&T::UnpackedScriptObject> {
        match self {
            ScriptResidence::Packed(_it) => None,
            ScriptResidence::Unpacked(it) => Some(it),
        }
    }

    pub fn ensure_unpacked(self: &mut ScriptResidence<T>, failed: &mut bool, node: Handle<Node>) {
        if *failed {
            // don't spam logs, though, plugin is completely broken at this point
            return;
        }
        if self.is_packed() {
            // script was just loaded from the scene file or safe game. unpack it!
            let data = match self {
                ScriptResidence::Packed(it) => {
                    it.node = node;
                    let so = T::unpack_node_script(it);
                    match so {
                        Ok(it) => it,
                        Err(err) => {
                            Log::err(format!("failed to unpack node script: {:?}", err));
                            *failed = true;
                            return;
                        }
                    }
                }
                ScriptResidence::Unpacked(_) => panic!("WTF?"),
            };
            *self = Self::Unpacked(data);
        }
    }

    pub fn with_script_object<R>(&self, f: impl FnOnce(&NodeScriptObject<T>) -> R) -> R {
        match self {
            ScriptResidence::Packed(it) => f(it),
            ScriptResidence::Unpacked(_it) => todo!(),
            // ScriptResidence::Unpacked(it) => f(&it.borrow().unwrap()),
        }
    }

    pub fn with_script_object_mut<R>(
        &mut self,
        f: impl FnOnce(&mut NodeScriptObject<T>) -> R,
    ) -> R {
        match self {
            ScriptResidence::Packed(it) => f(it),
            ScriptResidence::Unpacked(_it) => todo!(),
            // ScriptResidence::Unpacked(it) => f(&mut it.borrow_mut().unwrap()),
        }
    }

    pub fn id(&self) -> Uuid {
        match self {
            ScriptResidence::Packed(it) => uuid_of_script(&it.obj),
            ScriptResidence::Unpacked(it) => T::id_of(it),
        }
    }
}

pub fn uuid_of_script<T: Lang>(script: &ScriptObject<T>) -> Uuid {
    script.def.metadata.uuid
}

impl<T: Lang> Debug for ScriptResidence<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptResidence::Packed(it) => it.fmt(f),
            ScriptResidence::Unpacked(it) => write!(f, "Packed( {:?} )", it),
        }
    }
}

impl<T: Lang> Clone for ScriptResidence<T> {
    fn clone(&self) -> Self {
        match self {
            ScriptResidence::Packed(it) => ScriptResidence::Packed(it.clone()),

            // will implement when know when cloning is really needed during game cycle
            ScriptResidence::Unpacked(_) => {
                panic!("cloning for Lua-backed ScriptData is not supported")
            }
        }
    }
}

impl<T: Lang> Drop for ScriptResidence<T> {
    fn drop(&mut self) {
        match self {
            ScriptResidence::Packed(_it) => {
                // ScriptObject is dropped automatically without delay
            }
            ScriptResidence::Unpacked(it) => {
                T::drop_script_object_to_prevent_delayed_destructor(it);
            }
        }
    }
}

impl<T: Lang> Visit for ScriptResidence<T> {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        match self {
            ScriptResidence::Packed(it) => it.visit(name, visitor),
            ScriptResidence::Unpacked(it) => it.visit(name, visitor),
        }
    }
}

impl<T: Lang> Visit for NodeScriptObject<T> {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.obj.visit(name, visitor)
    }
}

impl<T: Lang> Visit for ScriptObject<T> {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let reading = visitor.is_reading();

        let mut guard = visitor.enter_region(name)?;
        let it = self;
        let def = it.def.clone();

        // let's store all numbers as str, to allow user easily change type.
        macro_rules! visit_as_str {
            ($var:expr, $field_name:expr, $guard:expr, $ty:ty) => {{
                if reading {
                    let def = def.clone();
                    let mut action = || {
                        let mut v = $var.to_string();
                        if let Ok(_) = v.visit($field_name, &mut $guard) {
                            *$var = v
                                .parse::<$ty>()
                                .map_err(|_err| {
                                    Log::warn(format!(
                                        "Failed to initialize {}::{} with {}",
                                        &def.metadata.class, name, v,
                                    ));
                                    VisitError::FieldTypeDoesNotMatch
                                })
                                .unwrap();
                            return Ok(());
                        }
                        let mut v = *$var as f32;
                        if let Ok(_) = v.visit($field_name, &mut $guard) {
                            *$var = v as $ty;
                            return Ok(());
                        }
                        let mut v = *$var as f64;
                        if let Ok(_) = v.visit($field_name, &mut $guard) {
                            *$var = v as $ty;
                            return Ok(());
                        }
                        let mut v = *$var as i16;
                        if let Ok(_) = v.visit($field_name, &mut $guard) {
                            *$var = v as $ty;
                            return Ok(());
                        }
                        let mut v = *$var as i32;
                        if let Ok(_) = v.visit($field_name, &mut $guard) {
                            *$var = v as $ty;
                            return Ok(());
                        }
                        let mut v = *$var as i64;
                        if let Ok(_) = v.visit($field_name, &mut $guard) {
                            *$var = v as $ty;
                            return Ok(());
                        }
                        Log::warn(format!(
                            "failed to initialize {}::{}",
                            &def.metadata.class, name
                        ));
                        Ok(())
                    };
                    action()
                } else {
                    let mut v = $var.to_string();
                    v.visit($field_name, &mut $guard)
                }
            }};
        }

        for (i, field) in def.metadata.fields.iter().enumerate() {
            let field_name = &field.name.to_case(Case::UpperCamel);
            let result = match &mut it.values[i] {
                ScriptFieldValue::String(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::Node(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::UiNode(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::Prefab(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::Vector3(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::Vector2(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::Vector2I(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::Quaternion(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::RuntimePin(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::bool(it) => it.visit(field_name, &mut guard),
                ScriptFieldValue::f32(it) => visit_as_str!(it, field_name, guard, f32),
                ScriptFieldValue::f64(it) => visit_as_str!(it, field_name, guard, f64),
                ScriptFieldValue::i16(it) => visit_as_str!(it, field_name, guard, i16),
                ScriptFieldValue::i32(it) => visit_as_str!(it, field_name, guard, i32),
                ScriptFieldValue::i64(it) => visit_as_str!(it, field_name, guard, i64),
            };
            if let Err(err) = &result {
                Log::warn(
                    format!(
                        "skipping deserialization of field `{}::{}` ({:?}) due to error: {}",
                        it.def.metadata.class, field_name, it.values[i], err
                    )
                    .as_str(),
                );
            }
        }
        Ok(())
    }
}
