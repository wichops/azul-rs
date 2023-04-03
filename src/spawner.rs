use crate::prelude::*;

pub fn spawn_factories(mut commands: Commands, mut game: ResMut<Game>) {
    let factories_count = match game.players.len() {
        2 => 5,
        3 => 7,
        4 => 9,
        _ => panic!("invalid number of players"),
    };

    let step: f32 = 360.0 / factories_count as f32;

    for i in 0..factories_count {
        let radius: f32 = 260.0;
        let angle = (step * i as f32).to_radians() + std::f32::consts::FRAC_PI_2;

        println!("{angle}, {i}");
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        let bag = Bag {
            tiles: game.bag.tiles.drain(0..FACTORY_TILES).collect(),
        };

        let parent = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::hex("#826f58").unwrap(),
                        custom_size: Some(Vec2::splat(140.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    ..default()
                },
                FactoryBundle {
                    factory: Factory(i),
                    bag: bag.clone(),
                },
            ))
            .id();

        game.factories.push(parent);

        for (i, t) in bag.tiles.iter().enumerate() {
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
                        custom_size: Some(Vec2::splat(40.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        44.0 * (i / 2) as f32 - offset,
                        44.0 * (i % 2) as f32 - offset,
                        1.,
                    )),
                    ..default()
                })
                .id();

            commands.entity(parent).add_child(child);
        }
    }
}
