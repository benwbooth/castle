use bevy::{prelude::*, ecs::query::WorldQuery};
use bevy_ascii_terminal::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use linked_hash_map::LinkedHashMap;

pub const SCREEN_WIDTH: usize = 40;
pub const SCREEN_HEIGHT: usize = 25;

pub const BOARD_WIDTH: usize = 24;
pub const BOARD_HEIGHT: usize = 18;

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
    items: LinkedHashMap<String,Item>,
    monsters: LinkedHashMap<String,Monster>,
    rooms: LinkedHashMap<String,Room>,
}

#[derive(Component)]
pub struct Location{
    x: u16, 
    y: u16,
}

#[derive(Component)]
pub struct Player{
    hp: u16, 
    inventory: Vec<Item>,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    location: Location,
}

fn setup(mut commands: Commands) {
    // Create the terminal
    let mut terminal = Terminal::new([SCREEN_WIDTH, SCREEN_HEIGHT]);
    // Draw a blue "Hello world!" to the terminal
    terminal.put_string([0, 24], "Hello world!".fg(Color::BLUE));

    commands.spawn((
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal),
        // Automatically set up the camera to render the terminal
        AutoCamera,
    ));
    commands.spawn(PlayerBundle {
        player: Player {
            hp: 100,
            inventory: vec![],
        },
        location: Location {
            x: 10,
            y: 10
        },
    });
}

fn player_move(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    query: &mut Query<(&Player, &mut Location)>,) 
{
    for (player, mut location) in query.iter_mut() {
        if input.just_pressed(KeyCode::Up) || input.just_pressed(KeyCode::W) {
            location.y += 1;
        }
        else if input.just_pressed(KeyCode::Down) || input.just_pressed(KeyCode::S) {
            location.y -= 1;
        }
        else if input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::A) {
            location.x -= 1;
        }
        else if input.just_pressed(KeyCode::Right) || input.just_pressed(KeyCode::D) {
            location.x += 1;
        }
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
    .add_system(player_move)
    .add_system(player_prompt)
    .add_system(enter_room)
    .add_system(monster_move)
    .add_system(render)
    .run();
}
