use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

const COLOR_COUNT: usize = 5;
const TILES_PER_COLOR: usize = 20;
const TILES_COUNT: usize = TILES_PER_COLOR * COLOR_COUNT;

#[derive(Debug, PartialEq, Default)]
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

#[derive(Debug, Default)]
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

impl Bag {
    fn new() -> Self {
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

        Self { tiles }
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

impl Factory {
    fn refill(&mut self, bag: &mut Bag) {
        self.tiles = bag.tiles.drain(0..4).collect();
    }
}

impl Game {
    fn new(players_count: i32) -> Self {
        let factories_count = match players_count {
            2 => 5,
            3 => 7,
            4 => 9,
            _ => panic!("Invalid Number of players"),
        };

        let mut players = Vec::with_capacity(players_count as usize);
        for p in 0..players_count {
            players.push(Player::new(&p.to_string()));
        }

        let mut bag = Bag::new();
        let mut factories = vec![Factory::default(); factories_count];

        for f in factories.iter_mut() {
            f.refill(&mut bag);
        }

        Self {
            bag,
            players,
            factories,
            ..Self::default()
        }
    }
}

fn main() {
    let game = Game::new(4);

    dbg!(game);
    App::new().run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn setup_game() {
        let game = crate::Game::new(2);
        assert_eq!(game.phase, crate::Phase::Picking);
        assert_eq!(game.turn_count, 0);
        assert_eq!(game.player_index, 0);
    }

    #[test]
    fn setup_game_2_players() {
        let game = crate::Game::new(2);

        assert_eq!(game.players.len(), 2);
        assert_eq!(game.factories.len(), 5);
    }

    #[test]
    fn setup_game_3_players() {
        let game = crate::Game::new(3);

        assert_eq!(game.players.len(), 3);
        assert_eq!(game.factories.len(), 7);
    }

    #[test]
    fn setup_game_4_players() {
        let game = crate::Game::new(4);

        assert_eq!(game.players.len(), 4);
        assert_eq!(game.factories.len(), 9);
    }

    #[test]
    fn setup_factories() {
        let game = crate::Game::new(4);

        for f in &game.factories {
            assert_eq!(f.tiles.len(), 4);
        }

        assert_eq!(
            game.factories.len() * 4 + game.bag.tiles.len(),
            crate::TILES_COUNT
        );
    }
}
