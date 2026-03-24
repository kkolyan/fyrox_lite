use crate::arena::Arena;
use crate::errors::ResultTcrateLangSpecificErrorExt;
use crate::external_global_script_proxy::ExternalGlobalScriptProxy;
use crate::external_script_proxy::invoke_callback;
use crate::external_script_proxy::ExternalScriptProxy;
use crate::scripted_app::{GlobalHasCallback, APP};
use crate::LangSpecificError;
use fyrox::core::log::Log;
use fyrox::core::notify::EventKind;
use fyrox::core::reflect::prelude::*;
use fyrox::core::reflect::Reflect;
use fyrox::core::visitor::prelude::*;
use fyrox::core::visitor::Visit;
use fyrox::core::watcher::FileSystemWatcher;
use fyrox::event::Event;
use fyrox::plugin::DynamicPlugin;
use fyrox::plugin::Plugin;
use fyrox::plugin::PluginContext;
use fyrox::plugin::PluginRegistrationContext;
use fyrox::script::constructor::ScriptConstructor;
use fyrox::script::Script;
use fyrox_lite::lite_input::Input;
use itertools::Itertools;
use lite_runtime::global_script_object::ScriptObject;
use lite_runtime::global_script_object_residence::GlobalScriptResidence;
use lite_runtime::script_failure::ScriptFailureHandler;
use lite_runtime::script_metadata::ScriptDefinition;
use lite_runtime::script_object::NodeScriptObject;
use lite_runtime::script_object_residence::ScriptResidence;
use lite_runtime::wrapper_reflect;
use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

#[derive(Visit, Reflect)]
pub struct CPlugin {
    #[visit(skip)]
    #[reflect(hidden)]
    pub failed: bool,

    #[visit(skip)]
    #[reflect(hidden)]
    pub need_reload: bool,

    #[visit(skip)]
    #[reflect(hidden)]
    pub script_failure_handler: ScriptFailureHandler,

    #[visit(skip)]
    #[reflect(hidden)]
    pub scripts: RefCell<GlobalScriptList>,

    #[visit(skip)]
    #[reflect(hidden)]
    pub hot_reload: HotReload,
}

pub enum HotReload {
    Disabled,
    Enabled { watcher: FileSystemWatcher },
}

impl Debug for CPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalScriptPlugin")
            .field("failed", &self.failed)
            .field("scripts", &self.scripts)
            .finish()
    }
}

impl CPlugin {
    pub fn new(
        reloadable_assembly_path: Option<PathBuf>,
        script_failure_handler: ScriptFailureHandler,
    ) -> Self {
        Self {
            failed: false,
            need_reload: Default::default(),
            script_failure_handler,
            scripts: RefCell::new(Default::default()),
            hot_reload: reloadable_assembly_path
                .map(|path| {
                    println!(
                        "trying to initialize watcher for file: {}",
                        path.to_str().unwrap()
                    );
                    HotReload::Enabled {
                        watcher: FileSystemWatcher::new(&path, Duration::from_millis(500)).unwrap(),
                    }
                })
                .unwrap_or(HotReload::Disabled),
        }
    }

    pub fn is_candidate_for_reload(&self) -> bool {
        let HotReload::Enabled { watcher } = &self.hot_reload else {
            return false;
        };
        let mut reload = false;
        while let Some(event) = watcher.try_get_event() {
            Log::info(format!("FS watcher event: {event:?}"));
            match &event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    println!("assembly file change detected");
                    reload = true;
                }
                _ => {}
            }
        }
        reload
    }

    pub fn check_for_script_changes(&mut self) {
        self.need_reload = self.need_reload || self.is_candidate_for_reload();
    }
}

impl Default for CPlugin {
    fn default() -> Self {
        todo!()
    }
}

impl Clone for CPlugin {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Plugin for CPlugin {
    fn register(&self, context: PluginRegistrationContext) {
        APP.with_borrow_mut(|app| {
            let app = app.as_mut().unwrap();
            let initialization = (app.functions.init_scripts_metadata)().into_result();
            if let Err(e) = initialization {
                self.script_failure_handler
                    .handle_script_loading_failure(&e);
            } else {
                Log::info("C# script metadata loaded successfully");
            }
            app.load_scripts_metadata();
        });
        APP.with_borrow(|app| {
            let app = app.as_ref().unwrap();
            for md in app.scripts_metadata.as_ref().unwrap().node_scripts.values() {
                let def = Arc::new(ScriptDefinition {
                    metadata: md.md.clone(),
                    assembly_name: self.assembly_name(),
                });
                let name = def.metadata.class.clone();
                let class = md.id;

                let has_callback = md.has_callback;

                println!("adding script constructor {}: {}", &name, md.md.uuid);
                context
                    .serialization_context
                    .script_constructors
                    .add_custom(
                        md.md.uuid,
                        ScriptConstructor {
                            name: name.to_string(),
                            source_path: "",
                            assembly_name: self.assembly_name(),
                            constructor: Box::new(move || {
                                Script::new(ExternalScriptProxy {
                                    class,
                                    data: ScriptResidence::Packed(NodeScriptObject::new(&def)),
                                    has_callback,
                                })
                            }),
                        },
                    )
                    .unwrap();
            }
            for md in app
                .scripts_metadata
                .as_ref()
                .unwrap()
                .global_scripts
                .values()
            {
                let mut plugin_scripts = self.scripts.borrow_mut();
                let def = Arc::new(ScriptDefinition {
                    metadata: md.md.clone(),
                    assembly_name: self.assembly_name(),
                });
                let name = def.metadata.class.clone();
                let class = md.id;
                plugin_scripts.inner_mut().push(ExternalGlobalScriptProxy {
                    name: name.to_string(),
                    class,
                    has_callback: md.has_callback,
                    data: GlobalScriptResidence::Packed(ScriptObject::new(&def)),
                });
            }
        });

        self.script_failure_handler.initially_loaded.set(true);
    }

    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) {
        Input::init_thread_local_state();
        for script in self.scripts.borrow_mut().0.iter_mut() {
            script.data.ensure_unpacked(&mut self.failed);
            if !script.has_callback.contains(GlobalHasCallback::ON_INIT) {
                continue;
            }
            invoke_callback(&mut context, |app| {
                let scene_path = scene_path.map(|it| it.to_string()).into();
                let id = script.data.inner_unpacked().unwrap().instance.inner();
                let result = (app.functions.on_game_init)(id, scene_path);
                result.into_result().handle_scripting_error();
            });
        }
    }

    fn update(&mut self, context: &mut PluginContext) {
        for script in self.scripts.borrow_mut().0.iter_mut() {
            script.data.ensure_unpacked(&mut self.failed);
            if !script.has_callback.contains(GlobalHasCallback::ON_UPDATE) {
                continue;
            }
            invoke_callback(context, |app| {
                (app.functions.on_game_update)(
                    script.data.inner_unpacked().unwrap().instance.inner(),
                )
                .into_result()
                .handle_scripting_error();
            });
        }
    }

    fn on_os_event(&mut self, event: &Event<()>, _context: PluginContext) {
        Input::on_os_event(event);
    }

    fn post_update(&mut self, _context: &mut PluginContext) {
        Input::post_fixed_update();
        Arena::free();
    }
}

#[derive(Debug, Default, Clone)]
pub struct GlobalScriptList(Vec<ExternalGlobalScriptProxy>);

impl GlobalScriptList {
    pub fn inner(&self) -> &Vec<ExternalGlobalScriptProxy> {
        &self.0
    }
    pub fn inner_mut(&mut self) -> &mut Vec<ExternalGlobalScriptProxy> {
        &mut self.0
    }
}

impl Visit for GlobalScriptList {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut guard = visitor.enter_region(name)?;

        if APP.with_borrow(|app| app.as_ref().unwrap().is_editor) {
            // prevent visitor errors outside the opened scene
            return Ok(());
        }

        for item in self.inner_mut().iter_mut() {
            item.data.visit(item.name.as_str(), &mut guard)?;
        }
        Ok(())
    }
}

impl DynamicPlugin for CPlugin {
    fn display_name(&self) -> String {
        "C# Plugin".to_string()
    }

    fn is_reload_needed_now(&self) -> bool {
        self.need_reload
    }

    fn as_loaded_ref(&self) -> &dyn Plugin {
        self
    }

    fn as_loaded_mut(&mut self) -> &mut dyn Plugin {
        self
    }

    fn is_loaded(&self) -> bool {
        true
    }

    fn prepare_to_reload(&mut self) {
        println!("prepare_to_reload");
        self.need_reload = false;
    }

    fn reload(
        &mut self,
        fill_and_register: &mut dyn FnMut(&mut dyn Plugin) -> Result<(), String>,
    ) -> Result<(), String> {
        println!("Reloading C# scripts");
        self.scripts.borrow_mut().inner_mut().clear();
        fill_and_register(self)
    }
}
