use std::time::Duration;

use bevy::{color::palettes::css::*, prelude::*};

use crate::{despawn_screen, tetrust::*, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::new())
            .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(1000)))
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                OnExit(GameState::Game),
                (despawn_screen::<GameScreen>, despawn_screen::<Block>),
            )
            .add_systems(FixedUpdate, drop.run_if(in_state(GameState::Game)))
            .add_systems(
                Update,
                (draw, key_input, score_update).run_if(in_state(GameState::Game)),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            GameScreen,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Score,
                Node {
                    left: Val::Percent(2.),
                    top: Val::Percent(2.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                Text::new("score: 0"),
                TextFont {
                    font_size: 30.,
                    ..default()
                },
            ));
            parent.spawn((
                Node {
                    right: Val::Percent(27.),
                    top: Val::Percent(0.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                Text::new("Hold"),
            ));
            parent.spawn((
                Node {
                    right: Val::Percent(27.),
                    top: Val::Percent(20.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                Text::new("Next"),
            ));
        });
}

#[derive(Component)]
struct GameScreen;

#[derive(Component)]
struct Score;

#[derive(Component)]
struct Block;

#[derive(Bundle)]
struct BlockBundle {
    block: Block,
    sprite: Sprite,
    transform: Transform,
}

impl BlockBundle {
    fn new(color: Srgba, x: f32, y: f32) -> BlockBundle {
        let scale = Vec3::splat(25.);
        BlockBundle {
            block: Block,
            sprite: Sprite::from_color(color, Vec2::new(1., 1.)),
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale,
                ..default()
            },
        }
    }
}

fn spawn_block<const Y: usize, const X: usize>(
    commands: &mut Commands,
    field: [[BlockColor; X]; Y],
    x: f32,
    y: f32,
) {
    for (y_i, y_field) in field.iter().enumerate() {
        for (x_i, block) in y_field.iter().enumerate() {
            if let Some(color) = block_color(*block) {
                let x = (x_i as f32 * 25.) + x;
                let y = (y_i as f32 * 25.) + y;

                commands.spawn(BlockBundle::new(color, x, y));
            }
        }
    }
}

fn draw(mut commands: Commands, game: Res<Game>, query: Query<Entity, With<Sprite>>) {
    query.iter().for_each(|e| commands.entity(e).despawn());

    let Game {
        field,
        pos,
        block,
        hold,
        next,
        ..
    } = game.as_ref();

    if let Some(block) = hold {
        spawn_block(&mut commands, *block, 275., 250.);
    }

    for (i, next) in next.iter().take(NEXT_LENGTH).rev().enumerate() {
        spawn_block(&mut commands, *next, 275., (75. * i as f32) - 50.);
    }

    let mut field_buf = *field;
    // 描画用フィールドにゴーストブロックを書き込む
    let ghost_pos = ghost_pos(field, pos, block);
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field_buf[y + ghost_pos.y][x + ghost_pos.x] = block_kind::GHOST;
            }
        }
    }
    // 描画用フィールドにブロックの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field_buf[y + pos.y][x + pos.x] = block[y][x];
            }
        }
    }

    for (y_i, y) in field_buf.iter().enumerate() {
        // println!("{:?}", y);
        for (x_i, block) in y.iter().enumerate() {
            if let Some(color) = block_color(*block) {
                let x = (x_i * 25) as f32 - 100.;
                let y = (y_i * 25) as f32 - 200.;

                commands.spawn(BlockBundle::new(color, x, -y));
            }
        }
    }
}

fn key_input(mut game: ResMut<Game>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::ArrowLeft) {
        let new_pos = Position {
            x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
            y: game.pos.y,
        };
        move_block(&mut game, new_pos);
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        let new_pos = Position {
            x: game.pos.x,
            y: game.pos.y + 1,
        };
        move_block(&mut game, new_pos);
    }
    if input.just_pressed(KeyCode::ArrowRight) {
        let new_pos = Position {
            x: game.pos.x + 1,
            y: game.pos.y,
        };
        move_block(&mut game, new_pos);
    }
    if input.just_pressed(KeyCode::KeyZ) {
        // 左回転
        rotate_left(&mut game);
    }
    if input.just_pressed(KeyCode::KeyX) {
        // 右回転
        rotate_right(&mut game);
    }
    // up
    if input.just_pressed(KeyCode::ArrowUp) {
        // ハードドロップ
        hard_drop(&mut game);
        if landing(&mut game).is_err() {
            // ブロックを生成できないならゲームオーバー
            gameover(&game);
        }
    }
    if input.just_pressed(KeyCode::Space) {
        // ホールド
        hold(&mut game);
    }
}

// 自然落下
fn drop(
    mut game: ResMut<Game>,
    mut time: ResMut<Time<Fixed>>,
    mut state: ResMut<NextState<GameState>>,
) {
    // nミリ秒間スリーブする
    let sleep_msec = match 1000u64.saturating_sub((game.line as u64 / 10) * 100) {
        0 => 100,
        msec => msec,
    };
    // drop関数を呼び出す間隔を変更
    time.set_timestep(Duration::from_millis(sleep_msec));

    let new_pos = Position {
        x: game.pos.x,
        y: game.pos.y + 1,
    };
    if !is_collision(&game.field, &new_pos, &game.block) {
        // posの座標を更新
        game.pos = new_pos;
    } else {
        // ブロック落下後の処理
        if landing(&mut game).is_err() {
            // ブロックを生成できないならゲームオーバー
            gameover(&game);

            state.set(GameState::GameOver);
        }
    }
}

// スコアの更新
fn score_update(mut query: Query<&mut Text, With<Score>>, game: Res<Game>) {
    query.single_mut().0 = format!("score: {}", game.score);
}

const fn block_color(block_color: BlockColor) -> Option<Srgba> {
    match block_color {
        0 => None,
        1 => Some(GRAY),
        2 => Some(WHITE),
        3 => Some(BLUE),
        4 => Some(GREEN),
        5 => Some(RED),
        6 => Some(SKY_BLUE),
        7 => Some(PINK),
        8 => Some(ORANGE),
        9 => Some(YELLOW),
        _ => panic!(),
    }
}
