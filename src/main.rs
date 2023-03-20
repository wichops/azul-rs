use bevy::app::App;
use bevy::prelude::{info, IntoSystemConfig};

use bevy::prelude::{DefaultPlugins, Input, KeyCode, OnUpdate, Res, ResMut, Resource, States};
use rand::seq::SliceRandom;
use rand::thread_rng;

const COLOR_COUNT: usize = 5;
const TILES_PER_COLOR: usize = 20;
const TILES_COUNT: usize = TILES_PER_COLOR * COLOR_COUNT;
const FACTORY_TILES: usize = 4;

#[derive(Debug, Hash, Clone, Eq, PartialEq, Default, States)]
enum Phase {
    #[default]
    Picking,
    Tiling,
}

#[derive(Debug, Clone)]
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
    phase: Phase,
    players: Vec<Player>,
    factories: Vec<Factory>,
    bag: Bag,
    turn_count: usize,
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
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
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

fn input(keyboard_input: Res<Input<KeyCode>>, game: ResMut<Game>) {
    if keyboard_input.pressed(KeyCode::Key1) {
        info!("{:?}", game.factories[0]);
    }
    if keyboard_input.pressed(KeyCode::Key2) {
        info!("{:?}", game.factories[1]);
    }
    if keyboard_input.pressed(KeyCode::Key3) {
        info!("{:?}", game.factories[2]);
    }
    if keyboard_input.pressed(KeyCode::Key4) {
        info!("{:?}", game.factories[3]);
    }

    if keyboard_input.pressed(KeyCode::Key5) {
        info!("{:?}", game.factories[4]);
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
        players.push(Player::new(&p.to_string()));
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<Phase>()
        .init_resource::<Game>()
        .add_startup_system(setup)
        .add_system(input)
        .run();
}
