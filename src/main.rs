mod components;
mod spawner;

mod prelude {

    pub use bevy::app::App;
    pub use bevy::core_pipeline::clear_color::ClearColorConfig;
    pub use bevy::prelude::*;
    pub use rand::seq::SliceRandom;
    pub use rand::thread_rng;

    pub const TILES_PER_COLOR: usize = 20;
    pub const FACTORY_TILES: usize = 4;

    #[derive(Debug, Hash, Clone, Eq, PartialEq, Default, States)]
    pub enum GameState {
        #[default]
        PickingFactory,
        PickingColor,
        Tiling,
    }

    #[derive(Resource, Debug, Default)]
    pub struct Game {
        pub players: Vec<Player>,
        pub factories: Vec<Entity>,
        pub center: Vec<Tile>,
        pub bag: Bag,
        pub selected_factory: usize,
        pub player_index: usize,
    }

    pub use crate::components::*;
    pub use crate::spawner::*;
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<Game>()
        .add_startup_systems((setup, spawn_factories))
        // .add_system(select_factory)
        .add_system(select_factory.in_set(OnUpdate(GameState::PickingFactory)))
        // .add_system(color_instructions.in_schedule(OnEnter(GameState::PickingColor)))
        // .add_system(select_color.in_set(OnUpdate(GameState::PickingColor)))
        .add_system(xd.in_schedule(OnExit(GameState::PickingColor)))
        .run();
}

// fn select_color(
//     mut commands: Commands,
//     mut keyboard_input: ResMut<Input<KeyCode>>,
//     mut game: ResMut<Game>,
//     mut next_state: ResMut<NextState<GameState>>,
// ) {
//     let mut color = None;
//     if keyboard_input.clear_just_pressed(KeyCode::Key1) {
//         color = Some(TileColor::Black);
//     }
//     if keyboard_input.clear_just_pressed(KeyCode::Key2) {
//         color = Some(TileColor::White);
//     }
//     if keyboard_input.clear_just_pressed(KeyCode::Key3) {
//         color = Some(TileColor::Red);
//     }
//     if keyboard_input.clear_just_pressed(KeyCode::Key4) {
//         color = Some(TileColor::Green);
//     }

//     if keyboard_input.clear_just_pressed(KeyCode::Key5) {
//         color = Some(TileColor::Blue);
//     }

//     let mut removed_tiles = Vec::new();
//     if let Some(color) = color {
//         let selected_factory = game.selected_factory;

//         if let Some(factory) = game.factories.get_mut(selected_factory) {
//             let mut moved_tiles = 0;
//             let mut i = 0;
//             while i < factory.bag.tiles.len() {
//                 if factory.bag.tiles[i].color == color {
//                     factory.bag.tiles.remove(i);
//                     moved_tiles += 1;
//                 } else {
//                     i += 1;
//                 }
//             }

//             if let Some(entity) = factory.entity {
//                 commands.entity(entity).despawn_descendants();
//             }
//             removed_tiles = factory.tiles.drain(..).collect();

//             let player_index = game.player_index;
//             let player = game.players.get_mut(player_index).unwrap();
//             let rows = &mut player.board.rows;
//             // TODO: This 4 is hardcoded get input for board row
//             let row = &mut rows[4];

//             row.color = Some(color);
//             row.filled = std::cmp::min(row.size, moved_tiles);
//         }

//         game.center.append(&mut removed_tiles);
//         game.player_index += 1;

//         if game.player_index >= game.players.len() {
//             game.player_index = 0;
//         }

//         next_state.set(GameState::PickingFactory);
//     }
// }

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    let players_count = 2;

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

    game.bag = Bag::default();
    game.bag.tiles = tiles;

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BEIGE),
        },
        ..default()
    });
}

fn xd(game: Res<Game>) {
    dbg!("{}", game);
}

fn select_factory(
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut factories: Query<(&Transform, &Factory, &mut Sprite)>,
    window: Query<&Window>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for (t, f, mut s) in factories.iter_mut() {
            let sprite_size = Vec2::splat(140.);
            let sprite_position = t.translation;

            let sprite_rect = Rect::from_center_size(sprite_position.truncate(), sprite_size);

            if sprite_rect.contains(position) && mouse_button_input.just_pressed(MouseButton::Left)
            {
                s.color = Color::hex("#B99B6B").unwrap();
                println!("Selected factory {:?}", f.0 + 1);
            }
        }
    }
}
