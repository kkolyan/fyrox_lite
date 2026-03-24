use super::script_object_residence::ScriptResidence;
use crate::fyrox_lua_plugin::PluginsRefMut_Ext;
use crate::lua_lifecycle::{invoke_callback_node, lua_vm};
use crate::user_script_impl::LuaUserScriptMessageEnvelope;
use fyrox::core::reflect::prelude::*;
use fyrox::core::type_traits::prelude::*;
use fyrox::core::visitor::prelude::*;
use fyrox::core::Downcast;
use fyrox::script::BaseScript;
use fyrox::script::ScriptContext;
use fyrox::script::ScriptTrait;
use lite_runtime::{reflect_base, reflect_base_lite, wrapper_reflect};
use mlua::{IntoLua, Value};
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone, ComponentProvider)]
pub struct ExternalScriptProxy {
    pub name: String,
    pub data: ScriptResidence,
}

impl ScriptTrait for ExternalScriptProxy {
    fn on_init(&mut self, ctx: &mut ScriptContext) {
        self.data
            .ensure_unpacked(&mut ctx.plugins.lua_mut().failed, ctx.handle);
        invoke_callback_node(&mut self.data, ctx, "on_init", || Ok(()));
    }

    fn on_start(&mut self, ctx: &mut ScriptContext) {
        self.data
            .ensure_unpacked(&mut ctx.plugins.lua_mut().failed, ctx.handle);
        invoke_callback_node(&mut self.data, ctx, "on_start", || Ok(()));
    }

    fn on_deinit(&mut self, ctx: &mut fyrox::script::ScriptDeinitContext) {
        invoke_callback_node(&mut self.data, ctx, "on_deinit", || Ok(()));
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.data
            .ensure_unpacked(&mut ctx.plugins.lua_mut().failed, ctx.handle);
        let dt = ctx.dt;
        invoke_callback_node(&mut self.data, ctx, "on_update", || Ok(dt));
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        if let Some(lua_message) = message.downcast_ref::<LuaUserScriptMessageEnvelope>() {
            self.data
                .ensure_unpacked(&mut ctx.plugins.lua_mut().failed, ctx.handle);
            invoke_callback_node(&mut self.data, ctx, "on_message", || {
                Ok((
                    lua_message.class.clone().into_lua(lua_vm()).unwrap(),
                    Value::clone(&lua_message.message),
                ))
            });
        } else {
            panic!("non-lua messages not supported by lua scripts")
        }
    }
}

impl BaseScript for ExternalScriptProxy {
    fn clone_box(&self) -> Box<dyn ScriptTrait> {
        Box::new(self.clone())
    }
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
    fn as_any_ref_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn id(&self) -> Uuid {
        self.data.id()
    }
}

impl Visit for ExternalScriptProxy {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.data.visit(name, visitor)
    }
}

impl Reflect for ExternalScriptProxy {
    reflect_base!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        self.data.with_script_object(|it| it.fields_info(func))
    }

    fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
        self.data.with_script_object(|it| it.fields(func))
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
        self.data.with_script_object_mut(|it| it.fields_mut(func))
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        self.data.with_script_object(|it| it.field(name, func))
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        self.data
            .with_script_object_mut(|it| it.field_mut(name, func))
    }
}
