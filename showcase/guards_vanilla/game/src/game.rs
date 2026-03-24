//! Game project.

use crate::beacon::Beacon;
use crate::bullet::Bullet;
use crate::guard::Guard;
use crate::guard_chief::GuardChief;
use crate::player::Player;
use fyrox::core::color::Color;
use fyrox::event::{Event, WindowEvent};
use fyrox::gui::brush::Brush;
use fyrox::gui::text::TextBuilder;
use fyrox::gui::widget::WidgetBuilder;
use fyrox::keyboard::{KeyCode, PhysicalKey};
pub use fyrox::plugin::Plugin;
use fyrox::{
    core::{
        algebra::Vector3, pool::Handle, reflect::prelude::*, visitor::prelude::*, TypeUuidProvider,
    },
    gui::{message::MessageDirection, text::TextMessage, UiNode},
    plugin::{PluginContext, PluginRegistrationContext},
    scene::node::Node,
    script::ScriptTrait,
};
use std::process::exit;

#[derive(Visit, Reflect, Debug, Default, Clone)]
pub struct Game {
    pub player: Handle<Node>,
    pub beacons: Vec<Vector3<f32>>,
    pub frags: usize,
    pub wounds: usize,
    hud: Handle<UiNode>,
}
impl Plugin for Game {
    fn register(&self, ctx: PluginRegistrationContext) {
        register_script::<Player>(&ctx, "Player");
        register_script::<Bullet>(&ctx, "Bullet");
        register_script::<Guard>(&ctx, "Guard");
        register_script::<GuardChief>(&ctx, "GuardChief");
        register_script::<Beacon>(&ctx, "Beacon");
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        self.hud = TextBuilder::new(
            WidgetBuilder::new().with_foreground(Brush::Solid(Color::BLACK).into()),
        )
        .with_font_size(40.0.into())
        .build(&mut context.user_interfaces.first_mut().build_ctx());
    }

    fn on_os_event(&mut self, event: &Event<()>, context: PluginContext) {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::KeyboardInput { event, .. } = event {
                if let PhysicalKey::Code(code) = event.physical_key {
                    if KeyCode::Escape == code {
                        println!("User requested exit");
                        exit(0);
                    }
                }
            }
        }
    }

    fn update(&mut self, #[allow(unused_variables)] context: &mut PluginContext) {
        context
            .user_interfaces
            .first_mut()
            .send_message(TextMessage::text(
                self.hud,
                MessageDirection::ToWidget,
                format!("Wounds: {}\nKilled Guards: {}", self.wounds, self.frags),
            ));
        profiling::finish_frame!();
    }
}

fn register_script<T: TypeUuidProvider + ScriptTrait + Default>(
    ctx: &PluginRegistrationContext,
    name: &str,
) {
    ctx.serialization_context.script_constructors.add::<T>(name);
}
