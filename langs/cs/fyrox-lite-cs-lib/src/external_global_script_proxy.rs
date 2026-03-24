use crate::bindings_manual::NativeClassId;
use crate::c_lang::CCompatibleLang;
use crate::scripted_app::GlobalHasCallback;
use fyrox::core::reflect::{FieldInfo, Reflect};
use fyrox::core::type_traits::prelude::*;
use fyrox::core::visitor::{Visit, VisitResult, Visitor};
use lite_runtime::global_script_object_residence::GlobalScriptResidence;
use lite_runtime::{reflect_base, reflect_base_lite};
use std::fmt::Debug;

#[derive(Debug, Clone, ComponentProvider)]
pub struct ExternalGlobalScriptProxy {
    pub name: String,
    pub class: NativeClassId,
    pub has_callback: GlobalHasCallback,
    pub data: GlobalScriptResidence<CCompatibleLang>,
}

impl Visit for ExternalGlobalScriptProxy {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.data.visit(name, visitor)
    }
}

impl Reflect for ExternalGlobalScriptProxy {
    reflect_base!();

    reflect_base_lite!();

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
