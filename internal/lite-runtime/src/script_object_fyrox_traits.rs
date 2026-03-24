use crate::global_script_object::ScriptObject;
use crate::{
    reflect_base,
    script_metadata::ScriptFieldValueType,
    script_object::{Lang, NodeScriptObject, ScriptFieldValue},
};
use fyrox::asset::Resource;
use fyrox::core::algebra::{UnitQuaternion, Vector2, Vector3};
use fyrox::core::pool::Handle;
use fyrox::core::reflect::*;
use fyrox::gui::UiNode;
use fyrox::resource::model::Model;
use fyrox::scene::node::Node;
use std::any::TypeId;
use std::process::exit;

impl<T: Lang> Reflect for NodeScriptObject<T> {
    crate::wrapper_reflect!(obj);
}

impl<T: Lang> Reflect for ScriptObject<T> {
    reflect_base!();

    crate::reflect_base_lite!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        let def = self.def.clone();
        let fields = def
            .metadata
            .fields
            .iter()
            .filter(|it| it.ty != ScriptFieldValueType::RuntimePin)
            .enumerate()
            .filter(|(_i, it)| !it.private)
            .map(|(i, it)| FieldInfo {
                owner_type_id: TypeId::of::<ScriptObject<T>>(),
                // owner_type_id: TypeId::of::<NodeScriptObject<T>>(),
                name: it.name.as_str(),
                display_name: it.title.as_str(),
                description: it.name.as_str(),
                tag: "",
                type_name: match it.ty {
                    ScriptFieldValueType::bool => std::any::type_name::<bool>(),
                    ScriptFieldValueType::f32 => std::any::type_name::<f32>(),
                    ScriptFieldValueType::f64 => std::any::type_name::<f64>(),
                    ScriptFieldValueType::i16 => std::any::type_name::<i16>(),
                    ScriptFieldValueType::i32 => std::any::type_name::<i32>(),
                    ScriptFieldValueType::i64 => std::any::type_name::<i64>(),
                    ScriptFieldValueType::String => std::any::type_name::<T::String<'_>>(),
                    ScriptFieldValueType::Node => std::any::type_name::<Handle<Node>>(),
                    ScriptFieldValueType::UiNode => std::any::type_name::<Handle<UiNode>>(),
                    ScriptFieldValueType::Prefab => {
                        std::any::type_name::<Option<Resource<Model>>>()
                    }
                    ScriptFieldValueType::Vector3 => std::any::type_name::<Vector3<f32>>(),
                    ScriptFieldValueType::Vector2 => std::any::type_name::<Vector2<f32>>(),
                    ScriptFieldValueType::Vector2I => std::any::type_name::<Vector2<i32>>(),
                    ScriptFieldValueType::Quaternion => {
                        std::any::type_name::<UnitQuaternion<f32>>()
                    }
                    ScriptFieldValueType::RuntimePin => panic!("WTF, it's excluded above"),
                },
                doc: it.description.unwrap_or(""),
                value: match self.values.get(i).unwrap() {
                    ScriptFieldValue::String(it) => it,
                    ScriptFieldValue::Node(it) => it,
                    ScriptFieldValue::UiNode(it) => it,
                    ScriptFieldValue::Prefab(it) => it,
                    ScriptFieldValue::Vector3(it) => it,
                    ScriptFieldValue::Vector2(it) => it,
                    ScriptFieldValue::Vector2I(it) => it,
                    ScriptFieldValue::Quaternion(it) => it,
                    ScriptFieldValue::RuntimePin(_it) => panic!("WTF, it's excluded above"),
                    ScriptFieldValue::bool(it) => it,
                    ScriptFieldValue::f32(it) => it,
                    ScriptFieldValue::f64(it) => it,
                    ScriptFieldValue::i16(it) => it,
                    ScriptFieldValue::i32(it) => it,
                    ScriptFieldValue::i64(it) => it,
                },
                reflect_value: self.values.get(i).unwrap().as_reflect(),
                read_only: false,
                immutable_collection: false,
                min_value: None,
                max_value: None,
                step: None,
                precision: None,
            })
            .collect::<Vec<_>>();
        func(&fields)
    }

    fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
        let fields = self
            .values
            .iter()
            .enumerate()
            .filter(|(i, _it)| !self.def.metadata.fields[*i].private)
            .map(|(_i, it)| {
                let it: &dyn Reflect = it.as_reflect();
                it
            })
            .collect::<Vec<_>>();
        func(&fields)
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
        let mut fields = self
            .values
            .iter_mut()
            .enumerate()
            .filter(|(i, _it)| !self.def.metadata.fields[*i].private)
            .map(|(_i, it)| {
                let it: &mut dyn Reflect = it.as_reflect_mut();
                it
            })
            .collect::<Vec<_>>();
        func(&mut fields)
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        let def = self.def.clone();
        let value = self.values.get(def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &dyn Reflect = it.as_reflect();
            x
        }))
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        let def = self.def.clone();
        let value = self.values.get_mut(def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &mut dyn Reflect = it.as_reflect_mut();
            x
        }))
    }
}
