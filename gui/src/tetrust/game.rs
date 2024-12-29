use bevy::prelude::Resource;

use super::block::{
    block_kind, block_kind::WALL as W, gen_block_7, BlockColor, BlockKind, BlockShape, BLOCKS,
};
use std::collections::VecDeque;

// 得点表
pub const SCORE_TABLE: [usize; 5] = [
    0,   // 0段消し
    1,   // 1段消し
    5,   // 2段消し
    25,  // 3段消し
    100, // 4段消し
];

// フィールドサイズ
pub const FIELD_WIDTH: usize = 11 + 2 + 2; // フィールド＋壁＋番兵
pub const FIELD_HEIGHT: usize = 20 + 1 + 1; // フィールド＋底＋番兵
pub type Field = [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT];

// ネクストブロックを3つ表示
pub const NEXT_LENGTH: usize = 3;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub const fn init() -> Position {
        Position { x: 5, y: 0 }
    }
}

#[derive(Resource)]
pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockShape,
    pub hold: Option<BlockShape>,
    pub holded: bool,
    pub next: VecDeque<BlockShape>,
    pub next_buf: VecDeque<BlockShape>,
    pub score: usize,
    pub line: usize,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            field: [
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, W, W, W, W, W, W, W, W, W, W, W, W, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            pos: Position::init(),
            block: BLOCKS[rand::random::<BlockKind>() as usize],
            hold: None,
            holded: false,
            next: gen_block_7().into(),
            next_buf: gen_block_7().into(),
            score: 0,
            line: 0,
        };
        // 初期ブロックを供給
        spawn_block(&mut game).ok();
        game
    }
}

// ゴーストの座標を返す
pub fn ghost_pos(field: &Field, pos: &Position, block: &BlockShape) -> Position {
    let mut ghost_pos = *pos;
    while {
        let new_pos = Position {
            x: ghost_pos.x,
            y: ghost_pos.y + 1,
        };
        !is_collision(field, &new_pos, block)
    } {
        ghost_pos.y += 1;
    }
    ghost_pos
}

// ブロックがフィールドに衝突する場合は`true`を返す
pub fn is_collision(field: &Field, pos: &Position, block: &BlockShape) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if block[y][x] != block_kind::NONE && field[y + pos.y][x + pos.x] != block_kind::NONE {
                // ブロックとフィールドのどちらも何かしらのブロックがある場合は衝突している
                return true;
            }
        }
    }
    false
}

// ブロックをフィールドに固定する
pub fn fix_block(
    Game {
        field, pos, block, ..
    }: &mut Game,
) {
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field[y + pos.y][x + pos.x] = block[y][x];
            }
        }
    }
}

// 消せるラインがあるなら削除し、段を下げる
// 消したライン数を返す
pub fn erase_line(field: &mut Field) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT - 2 {
        let mut can_erase = true;
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] == 0 {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            count += 1;
            for y2 in (2..=y).rev() {
                field[y2] = field[y2 - 1];
            }
        }
    }
    count
}

// ブロックを指定した座標へ移動できるなら移動する
pub fn move_block(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, &game.block) {
        // posの座標を更新
        game.pos = new_pos;
    }
}

// スーパーローテーション処理
// スーパーローテーションできるなら、その座標を返す
fn super_rotation(field: &Field, pos: &Position, block: &BlockShape) -> Result<Position, ()> {
    // 1マスずらした座標
    let diff_pos = [
        // 上
        Position {
            x: pos.x,
            y: pos.y.checked_sub(1).unwrap_or(pos.y),
        },
        // 右
        Position {
            x: pos.x + 1,
            y: pos.y,
        },
        // 下
        Position {
            x: pos.x,
            y: pos.y + 1,
        },
        // 左
        Position {
            x: pos.x.checked_sub(1).unwrap_or(pos.x),
            y: pos.y,
        },
    ];
    for pos in diff_pos {
        if !is_collision(field, &pos, block) {
            return Ok(pos);
        }
    }
    Err(())
}

// 左に90度回転する
#[allow(clippy::needless_range_loop)]
pub fn rotate_left(game: &mut Game) {
    let mut new_shape: BlockShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[4 - 1 - x][y] = game.block[y][x];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos = new_pos;
        game.block = new_shape;
    }
}

// 右に90度回転する
#[allow(clippy::needless_range_loop)]
pub fn rotate_right(game: &mut Game) {
    let mut new_shape: BlockShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[y][x] = game.block[4 - 1 - x][y];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos = new_pos;
        game.block = new_shape;
    }
}

// ハードドロップする
pub fn hard_drop(game: &mut Game) {
    while {
        let new_pos = Position {
            x: game.pos.x,
            y: game.pos.y + 1,
        };
        !is_collision(&game.field, &new_pos, &game.block)
    } {
        game.pos.y += 1;
    }
    let new_pos = game.pos;
    move_block(game, new_pos);
}

// ホールド処理
// - 1回目のホールドは現在のブロックをホールド
// - 2回目以降のホールドは現在のブロックとホールドを交換
// - 現在のブロックに対して既にホールドしている場合は何もしない
pub fn hold(game: &mut Game) {
    if game.holded {
        // 現在のブロックに対して既にホールドしている場合は早期リターン
        return;
    }
    if let Some(mut hold) = game.hold {
        // ホールドの交換
        std::mem::swap(&mut hold, &mut game.block);
        game.hold = Some(hold);
        game.pos = Position::init();
    } else {
        // ホールドして、新たなブロックを生成
        game.hold = Some(game.block);
        spawn_block(game).ok();
    }
    // ホールド済のフラグを立てる
    game.holded = true;
}

// ブロック落下後の処理
pub fn landing(game: &mut Game) -> Result<(), ()> {
    // ブロックをフィールドに固定
    fix_block(game);
    // ラインの削除処理
    let line = erase_line(&mut game.field);
    // 消した段数によって得点を加算
    game.score += SCORE_TABLE[line];
    // 消した段数の合計を加算
    game.line += line;
    // ブロックの生成
    spawn_block(game)?;
    // 再ホールド可能にする
    game.holded = false;
    Ok(())
}

// ブロックを生成する
// 生成に失敗した場合は`Err(())`を返す
pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
    // posの座標を初期値へ
    game.pos = Position::init();
    // ネクストキューから次のブロックを取り出す
    game.block = game.next.pop_front().unwrap();
    if let Some(next) = game.next_buf.pop_front() {
        // バフからネクストキューに供給
        game.next.push_back(next);
    } else {
        // バフを生成
        game.next_buf = gen_block_7().into();
        // バフからネクストキューに供給
        game.next.push_back(game.next_buf.pop_front().unwrap());
    }
    // 衝突チェック
    if is_collision(&game.field, &game.pos, &game.block) {
        Err(())
    } else {
        Ok(())
    }
}
