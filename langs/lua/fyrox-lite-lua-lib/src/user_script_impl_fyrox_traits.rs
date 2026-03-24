use crate::user_script_impl::UserScriptProxy;
use fyrox::core::reflect::{FieldInfo, Reflect};
use lite_runtime::{reflect_base, reflect_base_lite};

impl Reflect for UserScriptProxy {
    reflect_base!();

    reflect_base_lite!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        match self {
            UserScriptProxy::Global(it) => it.fields_info(func),
            UserScriptProxy::Node(it) => it.fields_info(func),
        }
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        match self {
            UserScriptProxy::Global(it) => it.field(name, func),
            UserScriptProxy::Node(it) => it.field(name, func),
        }
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        match self {
            UserScriptProxy::Global(it) => it.field_mut(name, func),
            UserScriptProxy::Node(it) => it.field_mut(name, func),
        }
    }
}
