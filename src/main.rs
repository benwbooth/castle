use bevy::{prelude::*, window::WindowResized};
use bevy_ascii_terminal::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use linked_hash_map::LinkedHashMap;

pub const BOARD_WIDTH: usize = 40;
pub const BOARD_HEIGHT: usize = 25;

#[derive(serde::Deserialize)]
enum Effect {
    None,
}

#[derive(serde::Deserialize)]
struct Item {
    name: String,
    description: String,
    glyph: String,
    effect: Effect,
    x: u16,
    y: u16,
}

#[derive(serde::Deserialize)]
struct Monster {
    name: String,
    description: String,
    glyph: String,
    hp: u16,
    att: u16,
    def: u16,
}

#[derive(serde::Deserialize)]
struct Room {
    description: String,
    board: String,
    north: Option<String>,
    south: Option<String>,
    east: Option<String>,
    west: Option<String>,
    items: Vec<Item>,
    monsters: Vec<Monster>,
    commands: LinkedHashMap<String,String>,
}


#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "9cdb0933-7a78-46aa-9075-538223c0882d"]
struct Castle {
    rooms: LinkedHashMap<String,Room>,
}

#[derive(Component)]
pub struct Location{
    x: u16, 
    y: u16,
}

fn setup(mut commands: Commands) {
    // Create the terminal
    let mut terminal = Terminal::new([BOARD_WIDTH,BOARD_HEIGHT]);
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

fn player_move(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<Input<KeyCode>>) 
{
    if input.just_pressed(KeyCode::Up) || input.just_pressed(KeyCode::W) {

    }
    else if input.just_pressed(KeyCode::Down) || input.just_pressed(KeyCode::S) {

    }
    else if input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::A) {

    }
    else if input.just_pressed(KeyCode::Right) || input.just_pressed(KeyCode::D) {

    }
}

fn main () {
    App::new()
    .add_plugin(TomlAssetPlugin::<Castle>::new(&["castle.toml"]))
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
    .insert_resource(ClearColor(Color::BLACK))
    .add_system(window_resize_system)
    .add_system(player_move)
    .add_system(enter_room)
    .add_system(monster_move)
    .add_system(render)
    .run();
}
