use crate::prelude::*;

#[derive(Component, Debug, PartialEq, Clone)]
pub enum TileColor {
    Black,
    White,

    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name }
    }
}
