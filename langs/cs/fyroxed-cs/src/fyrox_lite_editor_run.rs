//! Editor with your game connected to it as a plugin.

use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyrox_build_tools::{BuildProfile, CommandDescriptor};
use fyrox_lite_cs_lib::fyrox_c_plugin::CPlugin;
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::plugin::EditorPlugin;
use fyroxed_base::settings::Settings;
use fyroxed_base::Editor;
use fyroxed_base::StartupData;
use lite_runtime::script_failure::ScriptFailureHandler;
use std::env;
use std::ffi::{c_char, c_int, CStr};
use std::path::Path;

#[no_mangle]
extern "C" fn fyrox_lite_editor_run(
    working_dir: *const c_char,
    assembly_path: *const c_char,
    is_cli: c_int,
) {
    Log::set_verbosity(MessageKind::Warning);
    let working_dir = unsafe { CStr::from_ptr(working_dir) }
        .to_str()
        .expect("failed to parse working directory argument");
    let working_dir = dunce::canonicalize(working_dir).unwrap();
    let event_loop = EventLoop::new().unwrap();
    println!("Using working dir: {}", working_dir.display());
    // Fyrox has some places that rely on `env::current_dir == working_dir`. To avoid potential
    // issues just make them the same
    env::set_current_dir(&working_dir).unwrap();
    let settings = ensure_cs_profiles(&working_dir);

    let mut editor = Editor::new_with_settings(
        Some(StartupData {
            working_directory: working_dir.clone(),
            scenes: vec!["data/scene.rgs".into()],
        }),
        settings,
    );

    let assembly_path = unsafe { CStr::from_ptr(assembly_path) }.to_str().unwrap();
    let plugin = CPlugin::new(
        Some(assembly_path.into()),
        ScriptFailureHandler::new_for_editor(match is_cli {
            1 => true,
            0 => false,
            _ => panic!("cannot convert is_cli to bool: {is_cli}"),
        }),
    );
    if let Err(err) = editor.add_dynamic_plugin_custom(plugin) {
        Log::err(err);
    }

    editor.user_project_icon = Some(include_bytes!("../../../../target/fyrox_cs_001.ico").to_vec());
    editor.user_project_name = "/C#".to_string();
    editor.user_project_version = "/0.1".to_string();

    editor.add_editor_plugin(CSharpPluginRefreshOnUpdate);

    editor.run(event_loop)
}

fn ensure_cs_profiles(_working_dir: &Path) -> Settings {
    let (mut settings, loaded) = match Settings::load() {
        Ok(it) => (it, true),
        Err(_) => (Settings::default(), false),
    };
    settings.build.profiles.clear();
    settings.build.profiles.push(BuildProfile {
        name: "C# Project".to_string(),
        build_commands: vec![CommandDescriptor {
            command: "dotnet".to_string(),
            args: vec!["build".to_string()],
            environment_variables: vec![],
            skip_passthrough_marker: false,
        }],
        run_command: CommandDescriptor {
            command: "dotnet".to_string(),
            args: vec!["run".to_string()],
            environment_variables: vec![],
            skip_passthrough_marker: false,
        },
    });
    settings.force_save();
    settings
}

struct CSharpPluginRefreshOnUpdate;

impl EditorPlugin for CSharpPluginRefreshOnUpdate {
    fn on_update(&mut self, editor: &mut Editor) {
        for it in editor.engine.plugins_mut() {
            if let Some(it) = it.cast_mut::<CPlugin>() {
                it.check_for_script_changes();
            }
        }
    }
}
