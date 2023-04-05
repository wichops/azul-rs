use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TileColor {
    Black,
    White,

    Red,
    Green,
    Blue,
}

#[derive(Component, Debug, Clone)]
pub struct Tile {
    pub color: TileColor,
}

impl Tile {
    pub fn new(color: TileColor) -> Self {
        Self { color }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct Bag {
    pub tiles: Vec<Tile>,
}

#[derive(Component, Debug, Clone)]
pub struct Center;

#[derive(Component)]
pub struct Factory(pub usize);

#[derive(Bundle)]
pub struct FactoryBundle {
    pub factory: Factory,

    #[bundle]
    pub bag: Bag,
}

#[derive(Component, Default, Debug)]
pub struct Board {
    pub rows: [Vec<Option<Tile>>; 5],
    pub floor: [[Option<Tile>; 5]; 5],
}

#[derive(Component, Default, Debug)]
pub struct Player {
    pub name: String,
    pub board: Board,
}

impl Player {
    pub fn new(name: String) -> Self {
        let mut board = Board::default();
        board.rows[0] = vec![None; 1];
        board.rows[1] = vec![None; 2];
        board.rows[2] = vec![None; 3];
        board.rows[3] = vec![None; 4];
        board.rows[4] = vec![None; 5];

        Player { name, board }
    }
}
