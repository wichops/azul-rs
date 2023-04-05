use crate::prelude::*;

pub fn spawn_factories(mut commands: Commands, mut game: ResMut<Game>) {
    let factories_count = match game.players.len() {
        1 => 3,
        2 => 5,
        3 => 7,
        4 => 9,
        _ => panic!("invalid number of players"),
    };

    let step: f32 = 360.0 / factories_count as f32;

    for i in 0..factories_count {
        let radius: f32 = 140.0;
        let angle = (step * i as f32).to_radians();

        let x = angle.cos() * radius + 260.0;
        let y = angle.sin() * radius;

        // let x = 100.0;
        // let y = 128.0 * i as f32 + game.players.len() as f32 / 2.0 * 10.0;
        let bag = Bag {
            tiles: game.bag.tiles.drain(0..FACTORY_TILES).collect(),
        };

        let parent = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::hex("#826f58").unwrap(),
                        custom_size: Some(Vec2::splat(120.0)),
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

        for (i, tile) in bag.tiles.iter().enumerate() {
            let color = match tile.color {
                TileColor::White => Color::WHITE,
                TileColor::Black => Color::BLACK,
                TileColor::Red => Color::RED,
                TileColor::Green => Color::GREEN,
                TileColor::Blue => Color::BLUE,
            };

            let offset = 20.0;
            let child = commands
                .spawn((
                    SpriteBundle {
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
                    },
                    tile.to_owned(),
                ))
                .id();

            commands.entity(parent).add_child(child);
        }
    }
}

pub fn spawn_board(mut commands: Commands, game: Res<Game>, players: Query<&Player>) {
    let selected_player = game.players[game.player_index];

    let parent = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("#826f58").unwrap(),
                custom_size: Some(Vec2::new(600.0, 400.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-300.0, 0., 0.)),
            ..default()
        })
        .id();

    if let Ok(current_player) = players.get(selected_player) {
        println!("{:?}", current_player.board);
        for (i, row) in current_player.board.rows.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let offset = 8.0;
                let x = (48.0 + offset) * i as f32 - 260.0;
                let y = (48.0 + offset) * j as f32 - 80.0;

                let color = if let Some(tile) = cell {
                    match tile.color {
                        TileColor::White => Color::WHITE,
                        TileColor::Black => Color::BLACK,
                        TileColor::Red => Color::RED,
                        TileColor::Green => Color::GREEN,
                        TileColor::Blue => Color::BLUE,
                    }
                } else {
                    Color::hex("#B99B6B").unwrap()
                };

                let child = commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::splat(48.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                        ..default()
                    })
                    .id();

                commands.entity(parent).add_child(child);
            }
        }

        for (i, row) in current_player.board.floor.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let offset = 8.0;
                let x = (48.0 + offset) * i as f32 + 40.0;
                let y = (48.0 + offset) * j as f32 - 80.0;

                let color = if let Some(tile) = cell {
                    match tile.color {
                        TileColor::White => Color::WHITE,
                        TileColor::Black => Color::BLACK,
                        TileColor::Red => Color::RED,
                        TileColor::Green => Color::GREEN,
                        TileColor::Blue => Color::BLUE,
                    }
                } else {
                    Color::hex("#B99B6B").unwrap()
                };

                let child = commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::splat(48.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                        ..default()
                    })
                    .id();

                commands.entity(parent).add_child(child);
            }
        }
    }
}
