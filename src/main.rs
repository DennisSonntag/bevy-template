use bevy::{prelude::*, window::PresentMode};

pub const WINDOW_SIZE: f32 = 600.;

fn main() {
	#[cfg(not(debug_assertions))]
	std::env::set_var("RUST_LOG", "");

	App::new()
		.insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "game".into(),
				resolution: (WINDOW_SIZE, WINDOW_SIZE + 100.).into(),
				present_mode: PresentMode::AutoVsync,
				fit_canvas_to_parent: true,
				prevent_default_event_handling: false,
				..default()
			}),
			..default()
		}))
		.insert_resource(Msaa::Sample8)
		.add_systems(Startup, setup_camera)
		.run();
}

fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
}
