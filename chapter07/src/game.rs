use crate::block::{BlockKind, BlockShape, BLOCKS};
use crate::block::{BlockColor, block_kind, COLOR_TABLE,
    block_kind::WALL as W,
};

// フィールドサイズ
pub const FIELD_WIDTH:  usize = 11 + 2 + 2;  // フィールド＋壁＋番兵
pub const FIELD_HEIGHT: usize = 20 + 1 + 1;  // フィールド＋底＋番兵
pub type Field = [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT];

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn init() -> Position {
        Position {
            x: 5,
            y: 0,
        }
    }
}

pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockShape,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: [
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,W,W,W,W,W,W,W,W,W,W,W,W,0],
                [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            ],
            pos: Position::init(),
            block: BLOCKS[rand::random::<BlockKind>() as usize],
        }
    }
}

// ゴーストの座標を返す
fn ghost_pos(field: &Field, pos: &Position, block: &BlockShape) -> Position {
    let mut ghost_pos = *pos;
    while {
        let new_pos = Position {
            x: ghost_pos.x,
            y: ghost_pos.y + 1,
        };
        !is_collision(field, &new_pos, block)
    }{
        ghost_pos.y += 1;
    }
    ghost_pos
}

// フィールドを描画する
#[allow(clippy::needless_range_loop)]
pub fn draw(Game { field, pos, block }: &Game) {
    // 描画用フィールドの生成
    let mut field_buf = *field;
    // 描画用フィールドにゴーストブロックを書き込む
    let ghost_pos = ghost_pos(field, pos, block);
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field_buf[y+ghost_pos.y][x+ghost_pos.x] = block_kind::GHOST;
            }
        }
    }
    // 描画用フィールドにブロックの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field_buf[y+pos.y][x+pos.x] = block[y][x];
            }
        }
    }
    // フィールドを描画
    println!("\x1b[H");  // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT-1 {
        for x in 1..FIELD_WIDTH-1 {
            print!("{}", COLOR_TABLE[field_buf[y][x]]);
        }
        println!();
    }
}

// ブロックがフィールドに衝突する場合は`true`を返す
pub fn is_collision(field: &Field, pos: &Position, block: &BlockShape) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y+pos.y >= FIELD_HEIGHT || x+pos.x >= FIELD_WIDTH {
                continue;
            }
            if block[y][x] != block_kind::NONE && field[y+pos.y][x+pos.x] != block_kind::NONE {
                // ブロックとフィールドのどちらも何かしらのブロックがある場合は衝突している
                return true;
            }
        }
    }
    false
}

// ブロックをフィールドに固定する
pub fn fix_block(Game { field, pos, block }: &mut Game) {
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field[y+pos.y][x+pos.x] = block[y][x];
            }
        }
    }
}

// 消せるラインがあるなら削除し、段を下げる
pub fn erase_line(field: &mut Field) {
    for y in 1..FIELD_HEIGHT-2 {
        let mut can_erase = true;
        for x in 2..FIELD_WIDTH-2 {
            if field[y][x] == 0 {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            for y2 in (2..=y).rev() {
                field[y2] = field[y2-1];
            }
        }
    }
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
            new_shape[4-1-x][y] = game.block[y][x];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos  = new_pos;
        game.block = new_shape;
    }
}

// 右に90度回転する
#[allow(clippy::needless_range_loop)]
pub fn rotate_right(game: &mut Game) {
    let mut new_shape: BlockShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[y][x] = game.block[4-1-x][y];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos  = new_pos;
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
    }{
        game.pos.y += 1;
    }
    let new_pos = game.pos;
    move_block(game, new_pos);
}

// ブロック落下後の処理
pub fn landing(game: &mut Game) -> Result<(), ()> {
    // ブロックをフィールドに固定
    fix_block(game);
    // ラインの削除処理
    erase_line(&mut game.field);
    // ブロックの生成
    spawn_block(game)?;
    Ok(())
}

// ブロックを生成する
// 生成に失敗した場合は`Err(())`を返す
pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
    // posの座標を初期値へ
    game.pos = Position::init();
    // ブロックをランダム生成
    game.block = BLOCKS[rand::random::<BlockKind>() as usize];
    // 衝突チェック
    if is_collision(&game.field, &game.pos, &game.block) {
        Err(())
    } else {
        Ok(())
    }
}

// ゲームオーバー処理
pub fn gameover(game: &Game) -> ! {
    draw(game);
    println!("GAMEOVER");
    quit();
}

// 終了処理
pub fn quit() -> ! {
    // カーソルを再表示
    println!("\x1b[?25h");
    std::process::exit(0);
}
