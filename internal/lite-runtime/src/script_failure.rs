use fyrox::core::log::Log;
use itertools::Itertools;
use native_dialog::{DialogBuilder, MessageLevel};
use std::cell::Cell;
use std::fmt::Display;
use std::process::exit;

pub struct ScriptFailureHandler {
    pub report_initial_failure: fn(&dyn Display),
    pub initially_loaded: Cell<bool>,
}

impl ScriptFailureHandler {
    pub fn new_for_game() -> ScriptFailureHandler {
        Self {
            report_initial_failure: |err| {
                println!("ERROR: Failed to load script metadata: {}", &err);
            },
            initially_loaded: Cell::new(false),
        }
    }

    pub fn new_for_editor(is_cli: bool) -> Self {
        Self {
            report_initial_failure: match is_cli {
                true => |err| {
                    println!("ERROR: {}", err);
                },
                false => |err| {
                    println!("ERROR: {}", &err);
                    DialogBuilder::message()
                        .set_text(err)
                        .set_level(MessageLevel::Error)
                        .alert()
                        .show()
                        .unwrap();
                },
            },
            initially_loaded: Cell::new(false),
        }
    }

    pub fn handle_script_loading_failure(&self, e: &dyn Display) {
        if self.initially_loaded.get() {
            Log::err("Failed to reload script metadata");
            for line in e.to_string().lines() {
                Log::err(line);
            }
        } else {
            (self.report_initial_failure)(&format!(
                "Failed to load script metadata:\n{}",
                e.to_string()
                    .lines()
                    .map(|it| format!("- {}", it))
                    .join("\n")
            ));
            exit(1);
        }
    }
}
