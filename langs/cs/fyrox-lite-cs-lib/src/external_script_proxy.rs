use crate::auto_dispose::AutoDispose;
use crate::bindings_manual::{NativeClassId, UserScriptMessage};
use crate::c_lang::CCompatibleLang;
use crate::errors::ResultTcrateLangSpecificErrorExt;
use crate::fyrox_c_plugin::CPlugin;
use crate::scripted_app::APP;
use crate::scripted_app::{HasCallback, ScriptedApp};
use fyrox::core::reflect::prelude::*;
use fyrox::core::type_traits::prelude::*;
use fyrox::core::visitor::prelude::*;
use fyrox::script::BaseScript;
use fyrox::script::ScriptContext;
use fyrox::script::ScriptTrait;
use lite_runtime::script_context::without_script_context;
use lite_runtime::script_context::UnsafeAsUnifiedContext;
use lite_runtime::script_object_residence::ScriptResidence;
use lite_runtime::{reflect_base, reflect_base_lite};
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone, ComponentProvider)]
pub struct ExternalScriptProxy {
    pub class: NativeClassId,
    pub has_callback: HasCallback,
    pub data: ScriptResidence<CCompatibleLang>,
}

impl ScriptTrait for ExternalScriptProxy {
    fn on_init(&mut self, ctx: &mut ScriptContext) {
        self.data
            .ensure_unpacked(&mut ctx.plugins.get_mut::<CPlugin>().failed, ctx.handle);
        if !self.has_callback.contains(HasCallback::ON_INIT) {
            return;
        }
        invoke_callback(ctx, |app| {
            (app.functions.on_init)(self.data.inner_unpacked().unwrap().instance.inner())
                .into_result()
                .handle_scripting_error();
        });
    }

    fn on_start(&mut self, ctx: &mut ScriptContext) {
        self.data
            .ensure_unpacked(&mut ctx.plugins.get_mut::<CPlugin>().failed, ctx.handle);
        if !self.has_callback.contains(HasCallback::ON_START) {
            return;
        }
        invoke_callback(ctx, |app| {
            (app.functions.on_start)(self.data.inner_unpacked().unwrap().instance.inner())
                .into_result()
                .handle_scripting_error();
        });
    }

    fn on_deinit(&mut self, ctx: &mut fyrox::script::ScriptDeinitContext) {
        if !self.has_callback.contains(HasCallback::ON_DEINIT) {
            return;
        }
        invoke_callback(ctx, |app| {
            (app.functions.on_deinit)(self.data.inner_unpacked().unwrap().instance.inner())
                .into_result()
                .handle_scripting_error();
        });
    }

    fn on_os_event(&mut self, _event: &fyrox::event::Event<()>, _ctx: &mut ScriptContext) {}

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.data
            .ensure_unpacked(&mut ctx.plugins.get_mut::<CPlugin>().failed, ctx.handle);
        if !self.has_callback.contains(HasCallback::ON_UPDATE) {
            return;
        }
        let dt = ctx.dt;
        invoke_callback(ctx, |app| {
            (app.functions.on_update)(self.data.inner_unpacked().unwrap().instance.inner(), dt)
                .into_result()
                .handle_scripting_error();
        });
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        let Some(message) = message.downcast_ref::<AutoDispose<UserScriptMessage>>() else {
            return;
        };
        self.data
            .ensure_unpacked(&mut ctx.plugins.get_mut::<CPlugin>().failed, ctx.handle);
        if !self.has_callback.contains(HasCallback::ON_MESSAGE) {
            return;
        }
        invoke_callback(ctx, |app| {
            (app.functions.on_message)(
                self.data.inner_unpacked().unwrap().instance.inner(),
                message.inner(),
            )
            .into_result()
            .handle_scripting_error();
        });
    }
}

pub(crate) fn invoke_callback(
    sc: &mut dyn UnsafeAsUnifiedContext<'_, '_, '_>,
    callback: impl FnOnce(&ScriptedApp),
) {
    APP.with_borrow(|app| {
        without_script_context(sc, || {
            callback(app.as_ref().unwrap());
        });
    });
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
