use bevy::app::App;
use bevy::prelude::{info, IntoSystemConfig};

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
enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
struct Tile {
    color: Color,
}

#[derive(Debug, Default)]
struct Bag {
    tiles: Vec<Tile>,
}

#[derive(Debug, Default, Clone)]
struct Factory {
    tiles: Vec<Tile>,
}

#[derive(Debug)]
struct Row {
    color: Option<Color>,
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
    fn new(color: Color) -> Self {
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
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut color = None;
    if keyboard_input.clear_just_pressed(KeyCode::Key1) {
        color = Some(Color::Black);
    }
    if keyboard_input.clear_just_pressed(KeyCode::Key2) {
        color = Some(Color::White);
    }
    if keyboard_input.clear_just_pressed(KeyCode::Key3) {
        color = Some(Color::Red);
    }
    if keyboard_input.clear_just_pressed(KeyCode::Key4) {
        color = Some(Color::Green);
    }

    if keyboard_input.clear_just_pressed(KeyCode::Key5) {
        color = Some(Color::Blue);
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

        if game.player_index > game.players.len() {
            game.player_index = 0;
        }

        next_state.set(GameState::PickingFactory);
    }
}

fn setup(mut game: ResMut<Game>) {
    let players_count = 2;
    let factories_count = match players_count {
        2 => 5,
        3 => 7,
        4 => 9,
        _ => panic!("invalid number of players"),
    };

    let mut tiles = [
        vec![Tile::new(Color::Red); TILES_PER_COLOR],
        vec![Tile::new(Color::Green); TILES_PER_COLOR],
        vec![Tile::new(Color::Blue); TILES_PER_COLOR],
        vec![Tile::new(Color::White); TILES_PER_COLOR],
        vec![Tile::new(Color::Black); TILES_PER_COLOR],
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
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<Game>()
        .add_system(setup.on_startup())
        .add_system(instructions.in_schedule(OnEnter(GameState::PickingFactory)))
        .add_system(select_factory.in_set(OnUpdate(GameState::PickingFactory)))
        .add_system(color_instructions.in_schedule(OnEnter(GameState::PickingColor)))
        .add_system(select_color.in_set(OnUpdate(GameState::PickingColor)))
        .add_system(xd.in_schedule(OnExit(GameState::PickingColor)))
        .run();
}
