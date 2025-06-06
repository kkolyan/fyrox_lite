use crate::debug::override_print;
use crate::debug::var_dump;
use crate::external_script_proxy::ExternalScriptProxy;
use crate::fyrox_lua_plugin::HotReload;
use crate::fyrox_lua_plugin::LuaPlugin;
use crate::fyrox_lua_plugin::PluginScriptList;
use crate::generated::registry::register_classes;
use crate::lua_lang::{LuaLang, UnpackedScriptObjectVisit};
use crate::lua_script_metadata::parse_file;
use crate::lua_utils::log_error;
use crate::script_class::ScriptClass;
use crate::script_metadata::ScriptDefinition;
use crate::script_metadata::ScriptKind;
use crate::script_metadata::ScriptMetadata;
use crate::script_object::NodeScriptObject;
use crate::script_object_residence::ScriptResidence;
use crate::typed_userdata::TypedUserData;
use crate::user_data_plus::Traitor;
use fyrox::core::log::Log;
use fyrox::core::watcher::FileSystemWatcher;
use fyrox::plugin::PluginRegistrationContext;
use fyrox::script::constructor::ScriptConstructor;
use fyrox::script::Script;
use fyrox::walkdir::DirEntry;
use fyrox_lite::script_context::without_script_context;
use fyrox_lite::script_context::UnsafeAsUnifiedContext;
use mlua::{Function, MultiValue, Table};
use mlua::IntoLuaMulti;
use mlua::Lua;
use mlua::UserDataRef;
use mlua::UserDataRefMut;
use mlua::Value;
use send_wrapper::SendWrapper;
use std::cell::RefCell;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use fyrox_lite::global_script_object::ScriptObject;
use fyrox_lite::global_script_object_residence::GlobalScriptResidence;
use crate::global_external_script_proxy::ExternalGlobalScriptProxy;
use crate::user_script_impl::UserScriptProxy;

thread_local! {
    static LOADING_CLASS_NAME: RefCell<Option<String>> = Default::default();
}

thread_local! {
    static LUA: RefCell<Option<&'static mlua::Lua>> = RefCell::new(None);
}

pub(crate) fn lua_vm() -> &'static Lua {
    LUA.with_borrow(|it| it.unwrap())
}

pub(crate) fn load_script(
    context: &PluginRegistrationContext,
    entry: &DirEntry,
    plugin_scripts: &mut PluginScriptList,
    assembly_name: &'static str,
) {
    if !entry.file_type().is_file() {
        return;
    }

    Log::info(format!("loading Lua script {:?}", entry.path()));

    let metadata = parse_file(entry.path());
    let metadata = match metadata {
        Ok(it) => it,
        Err(errs) => {
            for err in errs {
                Log::err(format!(
                    "failed to load script from file {}: {}",
                    &entry.path().to_string_lossy(),
                    err
                ));
            }
            return;
        }
    };

    let class_loading: mlua::Result<()> = LOADING_CLASS_NAME.with(|class_name| {
        *class_name.borrow_mut() = Some(metadata.class.clone());

        lua_vm()
            .load(
                "
                return function(class) 
                    package.loaded[class] = nil
                    require(class)
                end",
            )
            .eval::<Function>()
            .and_then(|it| it.call::<_, ()>(metadata.class.clone()))?;

        *class_name.borrow_mut() = None;
        Ok(())
    });

    match class_loading {
        Ok(_) => {}
        Err(err) => {
            Log::err(format!(
                "Failed to load Lua class {:?}: {}",
                &metadata.class, err
            ));
            return;
        }
    }

    let name = metadata.class.clone();

    let definition = Arc::new(ScriptDefinition {
        metadata,
        assembly_name,
    });

    let class = lua_vm()
        .globals()
        .get::<_, Option<UserDataRefMut<ScriptClass>>>(definition.metadata.class.as_str())
        .unwrap();
    let Some(mut class) = class else {
        Log::err(format!("invalid class file: {:?}", entry.path()));
        return;
    };

    class.def = Some(definition.clone());

    match definition.metadata.kind {
        ScriptKind::Node => {
            let addition_result = context
                .serialization_context
                .script_constructors
                .add_custom(
                    definition.metadata.uuid,
                    ScriptConstructor {
                        constructor: Box::new(move || {
                            Script::new(ExternalScriptProxy {
                                data: ScriptResidence::Packed(NodeScriptObject::new(&definition)),
                                name: definition.metadata.class.to_string(),
                            })
                        }),
                        name: name.to_string(),
                        source_path: entry.path().to_string_lossy().to_string().leak(),
                        assembly_name,
                    },
                );
            if let Err(err) = addition_result {
                Log::err(err.to_string().as_str());
            }
        }
        ScriptKind::Global => {
            plugin_scripts.inner_mut().push(ExternalGlobalScriptProxy {
                name: name.to_string(),
                data: GlobalScriptResidence::Packed(ScriptObject::new(&definition)),
            });
        }
    }

    Log::info(format!(
        "script registered: {}",
        entry.path().to_string_lossy()
    ));
}

pub(crate) fn create_plugin(scripts_dir: PathBuf, hot_reload_enabled: bool) -> LuaPlugin {
    // mlua has approach with lifetimes that makes very difficult storing Lua types
    // here and there in Rust. But we need a single Lua VM instance for the whole life
    // of game process, so that's ok to make it 'static.
    let vm = Box::leak(Box::new(Lua::new()));

    expose_os_exit(vm);

    LUA.set(Some(vm));
    let lua_version = vm.load("return _VERSION").eval::<mlua::String>().unwrap();
    println!("Lua Version: {}", lua_version.to_str().unwrap_or("unknown"));
    override_print(vm);

    vm.globals()
        .set("PINS", vm.create_table().unwrap())
        .unwrap();

    let scripts_dir_lua_comp = scripts_dir.display().to_string().replace("\\", "/");
    let setting_package_path = format!(
        "package.path = '{}/?.lua;{}/?/init.lua'",
        scripts_dir_lua_comp, scripts_dir_lua_comp
    );
    println!("Lua: {}", &setting_package_path);
    log_error(
        "set 'package.path'",
        vm.load(setting_package_path)
        .eval::<()>(),
    );

    {
        vm.globals()
            .set(
                "script_class",
                vm.create_function(move |_lua, _args: ()| {
                    LOADING_CLASS_NAME.with(|class_name| {
                        let class_name = class_name
                            .borrow()
                            .as_ref()
                            .expect("script_class() called out of permitted context")
                            .clone();

                        Ok(ScriptClass {
                            name: class_name.clone(),
                            table: Default::default(),
                            def: Default::default(),
                        })
                    })
                })
                .unwrap(),
            )
            .unwrap();
    }

    vm.globals()
        .set("var_dump", vm.create_function(var_dump).unwrap())
        .unwrap();

    register_classes(vm);

    println!("creating watcher for {}", scripts_dir.display());

    LuaPlugin {
        failed: false,
        scripts_dir: scripts_dir.display().to_string(),
        hot_reload: match hot_reload_enabled {
            true => HotReload::Enabled {
                watcher: FileSystemWatcher::new(scripts_dir, Duration::from_millis(500)).unwrap(),
            },
            false => HotReload::Disabled,
        },
        need_reload: false,
        scripts: Default::default(),
    }
}

// Luau doesn't define it
fn expose_os_exit(vm: &mut Lua) {
    let lua_os = vm.globals().get::<_, Table>("os").unwrap();
    lua_os
        .set("exit", vm.create_function::<_, (), _>(|_lua, args: MultiValue| {
            Log::info("os.exit() called by script");
            let code = args.get(0)
                .map(|it| it.as_i32().unwrap())
                .unwrap_or(0);
            exit(code);
        }).unwrap())
        .unwrap();
}

pub(crate) fn invoke_callback_global<'a, 'b, 'c, 'lua, A: IntoLuaMulti<'lua>>(
    data: &mut GlobalScriptResidence<LuaLang>,
    ctx: &mut dyn UnsafeAsUnifiedContext<'a, 'b, 'c>,
    callback_name: &str,
    args: impl FnOnce() -> mlua::Result<A>,
) {
    let script_object_ud = TypedUserData::clone(
        &data
            .inner_unpacked()
            .expect("WTF, it's guaranteed to be unpacked here")
            .0,
    );
    invoke_callback_internal(script_object_ud, ctx, callback_name, args)
}

pub(crate) fn invoke_callback_node<'a, 'b, 'c, 'lua, A: IntoLuaMulti<'lua>>(
    data: &mut ScriptResidence,
    ctx: &mut dyn UnsafeAsUnifiedContext<'a, 'b, 'c>,
    callback_name: &str,
    args: impl FnOnce() -> mlua::Result<A>,
) {
    let script_object_ud = TypedUserData::clone(
        &data
            .inner_unpacked()
            .expect("WTF, it's guaranteed to be unpacked here")
            .0,
    );
    invoke_callback_internal(script_object_ud, ctx, callback_name, args)
}

pub(crate) fn invoke_callback_internal<'a, 'b, 'c, 'lua, A: IntoLuaMulti<'lua>>(
    script_object_ud: TypedUserData<'lua, UserScriptProxy>,
    ctx: &mut dyn UnsafeAsUnifiedContext<'a, 'b, 'c>,
    callback_name: &str,
    args: impl FnOnce() -> mlua::Result<A>,
) {
    without_script_context(ctx, || {

        let class_name = script_object_ud.borrow().unwrap().as_script_object().def.metadata.class.clone();
        // TODO optimize me
        let class = LUA
            .with_borrow(|it| it.unwrap())
            .globals()
            .get::<_, UserDataRef<ScriptClass>>(class_name.as_str())
            .unwrap_or_else(|err| panic!("class not found: {}. error: {}", class_name, err));

        let callback = class.table.get(callback_name);

        if let Some(Value::Function(callback)) = callback {
            let args = args().unwrap();
            match callback.call::<_, ()>((script_object_ud, args)) {
                Ok(_) => {}
                Err(err) => {
                    Log::err(format!(
                        "callback \"{}:{}\" failed with Lua error:\n{}",
                        class_name, callback_name, err
                    ));
                    Log::warn("exiting to prevent error spamming (change this behavior in future)");
                    exit(123);
                }
            };
        }
    });
}
