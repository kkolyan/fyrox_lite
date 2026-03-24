//! Executor with your game connected to it as a plugin.
use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyrox::dpi::LogicalSize;
use fyrox::engine::executor::Executor;
use fyrox::engine::GraphicsContextParams;
use fyrox::event_loop::EventLoop;
use fyrox::window::WindowAttributes;

fn main() {
    #[cfg(feature = "profiling")]
    {
        let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
        let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
        eprintln!("Run this to view profiling data:  puffin_viewer {server_addr}");
        puffin::set_scopes_on(true);
        std::thread::spawn(|| {
            std::process::Command::new("puffin_viewer")
                .args(["--url", "127.0.0.1:8585"])
                .output()
                .unwrap();
        });
    }

    Log::set_verbosity(MessageKind::Warning);
    let mut window_attributes = WindowAttributes::default();
    window_attributes.inner_size = Some(LogicalSize::new(1280.0, 720.0).into());
    let mut executor = Executor::from_params(
        EventLoop::new().unwrap(),
        GraphicsContextParams {
            window_attributes,
            vsync: false,
            msaa_sample_count: None,
            graphics_server_constructor: Default::default(),
        },
    );

    // Dynamic linking with hot reloading.
    #[cfg(feature = "dylib")]
    {
        #[cfg(target_os = "windows")]
        let file_name = "game_dylib.dll";
        #[cfg(target_os = "linux")]
        let file_name = "libgame_dylib.so";
        #[cfg(target_os = "macos")]
        let file_name = "libgame_dylib.dylib";
        executor.add_dynamic_plugin(file_name, true, true).unwrap();
    }

    // Static linking.
    #[cfg(not(feature = "dylib"))]
    {
        use guards_vanilla_game::game::Game;
        executor.add_plugin(Game::default());
    }

    executor.run()
}
