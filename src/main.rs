use bevy::{prelude::*, window::WindowResized};
use bevy_ascii_terminal::prelude::*;

fn setup(mut commands: Commands) {
    // Create the terminal
    let mut terminal = Terminal::new([40,25]);
    // Draw a blue "Hello world!" to the terminal
    terminal.put_string([0, 24], "Hello world!".fg(Color::BLUE));

    commands.spawn((
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal),
        // Automatically set up the camera to render the terminal
        AutoCamera,
    ));
}

fn window_resize_system(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        println!("width = {} height = {}", e.width, e.height);
    }
}

fn main () {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Castle".into(),
            resolution: (960., 600.).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugin(TerminalPlugin)
    .add_startup_system(setup)
    .add_system(window_resize_system)
    .insert_resource(ClearColor(Color::BLACK))
    .run();
}
