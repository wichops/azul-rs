use bevy::app::App;
use bevy::sprite::MaterialMesh2dBundle;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

const TILES_PER_COLOR: usize = 20;
const FACTORY_TILES: usize = 4;

#[derive(Debug, Hash, Clone, Eq, PartialEq, Default, States)]
enum GameState {
    #[default]
    PickingFactory,
    PickingColor,
    Tiling,
}

#[derive(Debug, PartialEq, Clone)]
enum TileColor {
    Black,
    White,

    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
struct Tile {
    color: TileColor,
}

#[derive(Debug, Default)]
struct Bag {
    tiles: Vec<Tile>,
}

#[derive(Component, Debug, Default, Clone)]
struct Factory {
    tiles: Vec<Tile>,
    entity: Option<Entity>,
}

#[derive(Debug)]
struct Row {
    color: Option<TileColor>,
    filled: usize,
    size: usize,
}

#[derive(Debug)]
struct Board {
    rows: [Row; 5],
}

#[derive(Debug)]
struct Player {
    name: String,
    board: Board,
}

#[derive(Resource, Debug, Default)]
struct Game {
    players: Vec<Player>,
    factories: Vec<Factory>,
    center: Vec<Tile>,
    bag: Bag,
    selected_factory: usize,
    player_index: usize,
}

impl Row {
    fn new(size: usize) -> Self {
        Self {
            color: None,
            filled: 0,
            size,
        }
    }
}

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            board: Board {
                rows: [
                    Row::new(1),
                    Row::new(2),
                    Row::new(3),
                    Row::new(4),
                    Row::new(5),
                ],
            },
        }
    }
}

impl Tile {
    fn new(color: TileColor) -> Self {
        Self { color }
    }
}

fn select_factory(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.clear_just_pressed(KeyCode::Key1) {
        game.selected_factory = 0;
        next_state.set(GameState::PickingColor);
        info!("{:?}", game.factories[0]);
    }
    if keyboard_input.clear_just_pressed(KeyCode::Key2) {
        game.selected_factory = 1;
        next_state.set(GameState::PickingColor);
        info!("{:?}", game.factories[1]);
    }
    if keyboard_input.clear_just_pressed(KeyCode::Key3) {
        game.selected_factory = 2;
        next_state.set(GameState::PickingColor);
        info!("{:?}", game.factories[2]);
    }
    if keyboard_input.clear_just_pressed(KeyCode::Key4) {
        game.selected_factory = 3;
        next_state.set(GameState::PickingColor);
        info!("{:?}", game.factories[3]);
    }

    if keyboard_input.clear_just_pressed(KeyCode::Key5) {
        game.selected_factory = 4;
        next_state.set(GameState::PickingColor);
        info!("{:?}", game.factories[4]);
    }
}

fn select_color(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut color = None;
    if keyboard_input.clear_just_pressed(KeyCode::Key1) {
        color = Some(TileColor::Black);
    }
    if keyboard_input.clear_just_pressed(KeyCode::Key2) {
        color = Some(TileColor::White);
    }
    if keyboard_input.clear_just_pressed(KeyCode::Key3) {
        color = Some(TileColor::Red);
    }
    if keyboard_input.clear_just_pressed(KeyCode::Key4) {
        color = Some(TileColor::Green);
    }

    if keyboard_input.clear_just_pressed(KeyCode::Key5) {
        color = Some(TileColor::Blue);
    }

    let mut removed_tiles = Vec::new();
    if let Some(color) = color {
        let selected_factory = game.selected_factory;

        if let Some(factory) = game.factories.get_mut(selected_factory) {
            let mut moved_tiles = 0;
            let mut i = 0;
            while i < factory.tiles.len() {
                if factory.tiles[i].color == color {
                    factory.tiles.remove(i);
                    moved_tiles += 1;
                } else {
                    i += 1;
                }
            }

            if let Some(entity) = factory.entity {
                commands.entity(entity).despawn_descendants();
            }
            removed_tiles = factory.tiles.drain(..).collect();

            let player_index = game.player_index;
            let player = game.players.get_mut(player_index).unwrap();
            let rows = &mut player.board.rows;
            // TODO: This 4 is hardcoded get input for board row
            let row = &mut rows[4];

            row.color = Some(color);
            row.filled = std::cmp::min(row.size, moved_tiles);
        }

        game.center.append(&mut removed_tiles);
        game.player_index += 1;

        if game.player_index >= game.players.len() {
            game.player_index = 0;
        }

        next_state.set(GameState::PickingFactory);
    }
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    let players_count = 2;
    let factories_count = match players_count {
        2 => 5,
        3 => 7,
        4 => 9,
        _ => panic!("invalid number of players"),
    };

    let mut tiles = [
        vec![Tile::new(TileColor::Red); TILES_PER_COLOR],
        vec![Tile::new(TileColor::Green); TILES_PER_COLOR],
        vec![Tile::new(TileColor::Blue); TILES_PER_COLOR],
        vec![Tile::new(TileColor::White); TILES_PER_COLOR],
        vec![Tile::new(TileColor::Black); TILES_PER_COLOR],
    ]
    .concat();

    let mut rng = thread_rng();
    tiles.shuffle(&mut rng);

    let mut players = Vec::with_capacity(players_count as usize);
    for p in 0..players_count {
        players.push(Player::new(format!("Player {}", p + 1)));
    }

    game.players = players;

    let mut factories = vec![Factory::default(); factories_count];
    for f in factories.iter_mut() {
        f.tiles = tiles.drain(0..FACTORY_TILES).collect();
    }

    game.factories = factories;
    game.bag = Bag::default();
    game.bag.tiles = tiles;

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BEIGE),
        },
        ..default()
    });
}

fn draw_factories(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game: ResMut<Game>,
) {
    let step: f32 = 360.0 / game.factories.len() as f32;

    for (i, f) in game.factories[..].iter_mut().enumerate() {
        info!("tiles {:?}", f.tiles);
        let radius: f32 = 280.0;
        let angle = (step * i as f32).to_radians();

        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        let parent = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(80.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GRAY)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..default()
            })
            .id();

        f.entity = Some(parent);

        for (i, t) in f.tiles.iter().enumerate() {
            let color = match t.color {
                TileColor::White => Color::WHITE,
                TileColor::Black => Color::BLACK,
                TileColor::Red => Color::RED,
                TileColor::Green => Color::GREEN,
                TileColor::Blue => Color::BLUE,
            };

            let offset = 20.0;
            let child = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(40.0, 40.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        44.0 * (i / 2) as f32 - offset,
                        44.0 * (i % 2) as f32 - offset,
                        0.,
                    )),
                    ..default()
                })
                .id();

            commands.entity(parent).add_child(child);
        }
    }
}

fn instructions(game: Res<Game>) {
    println!("Player: {}", game.player_index + 1);
    println!("Select Factory 1 - 8");
    for (i, f) in game.factories.iter().enumerate() {
        println!("Factory: {}", i + 1);
        println!("{:?}", f.tiles);
    }
    println!("Center:");
    println!("{:?}", game.center);
}

fn color_instructions(game: Res<Game>) {
    info!("Select Color 1- 5");
    info!("{:?}", game.factories[game.selected_factory].tiles);
}

fn xd(game: Res<Game>) {
    dbg!("{}", game);
}
fn cursor_position(window: Query<&Window>) {
    let window = window.single();

    if let Some(position) = window.cursor_position() {
        // info!("{:?}", position);
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<Game>()
        .add_startup_systems((setup, draw_factories))
        .add_system(cursor_position)
        .add_system(instructions.in_schedule(OnEnter(GameState::PickingFactory)))
        .add_system(select_factory.in_set(OnUpdate(GameState::PickingFactory)))
        .add_system(color_instructions.in_schedule(OnEnter(GameState::PickingColor)))
        .add_system(select_color.in_set(OnUpdate(GameState::PickingColor)))
        .add_system(xd.in_schedule(OnExit(GameState::PickingColor)))
        .run();
}
