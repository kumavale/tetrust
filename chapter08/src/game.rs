use std::collections::VecDeque;
use crate::mino::{MinoKind, MinoShape, MINOS, gen_mino_7};
use crate::block::{BlockColor, block_kind, COLOR_TABLE,
    block_kind::WALL as W,
};

pub const FIELD_WIDTH:  usize = 12 + 2;
pub const FIELD_HEIGHT: usize = 22 + 1;

// 得点表
pub const SCORE_TABLE: [usize; 5] = [
    0,    // 0段消し
    1,    // 1段消し
    5,    // 2段消し
    25,   // 3段消し
    100,  // 4段消し
];

pub type FieldSize = [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT];

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
    pub field: FieldSize,
    pub pos: Position,
    pub mino: MinoShape,
    pub hold: Option<MinoShape>,
    pub holded: bool,
    pub next:     VecDeque<MinoShape>,
    pub next_buf: VecDeque<MinoShape>,
    pub score: usize,
    pub line: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: [
                [0,W,W,W,0,0,0,0,0,0,W,W,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,0,0,0,0,0,0,0,0,0,0,W,0],
                [0,W,W,W,W,W,W,W,W,W,W,W,W,0],
                [0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            ],
            pos: Position::init(),
            mino: MINOS[rand::random::<MinoKind>() as usize],
            hold: None,
            holded: false,
            next:     gen_mino_7().into(),
            next_buf: gen_mino_7().into(),
            score: 0,
            line: 0,
        }
    }
}

// ゴーストの座標を返す
fn ghost_pos(field: &FieldSize, pos: &Position, mino: &MinoShape) -> Position {
    let mut ghost_pos = *pos;
    while {
        let new_pos = Position {
            x: ghost_pos.x,
            y: ghost_pos.y + 1,
        };
        !is_collision(field, &new_pos, mino)
    }{
        ghost_pos.y += 1;
    }
    ghost_pos
}

// フィールドを描画する
#[allow(clippy::needless_range_loop)]
pub fn draw(Game { field, pos, mino, hold, holded: _, next, next_buf: _, score, .. }: &Game) {
    // 描画用フィールドの生成
    let mut field_buf = *field;
    // 描画用フィールドにゴーストブロックを書き込む
    let ghost_pos = ghost_pos(field, pos, mino);
    for y in 0..4 {
        for x in 0..4 {
            if mino[y][x] != block_kind::NONE {
                field_buf[y+ghost_pos.y][x+ghost_pos.x] = block_kind::GHOST;
            }
        }
    }
    // 描画用フィールドにテトリミノの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            if mino[y][x] != block_kind::NONE {
                field_buf[y+pos.y][x+pos.x] = mino[y][x];
            }
        }
    }
    // ホールドを描画
    println!("\x1b[2;26HHOLD");  // カーソルをホールド位置に移動
    if let Some(hold) = hold {
        for y in 0..4 {
            print!("\x1b[{};26H", y+3);  // カーソルを移動
            for x in 0..4 {
                print!("{}", COLOR_TABLE[hold[y][x]]);
            }
            println!();
        }
    }
    // ネクストを描画(3つ)
    println!("\x1b[8;26HNEXT");  // カーソルをネクスト位置に移動
    for (i, next) in next.iter().take(3).enumerate() {
        for y in 0..4 {
            print!("\x1b[{};26H", i*4+y+9);  // カーソルを移動
            for x in 0..4 {
                print!("{}", COLOR_TABLE[next[y][x]]);
            }
            println!();
        }
    }
    // スコアを描画
    println!("\x1b[22;26HSCORE");  // カーソルをスコア位置に移動
    println!("\x1b[23;26H{}", score);
    // フィールドを描画
    println!("\x1b[H");  // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT-1 {
        for x in 1..FIELD_WIDTH-1 {
            print!("{}", COLOR_TABLE[field_buf[y][x]]);
        }
        println!();
    }
    // 色情報をリセット
    println!("\x1b[0m");
}

// テトリミノがフィールドに衝突する場合は`ture`を返す
pub fn is_collision(field: &FieldSize, pos: &Position, mino: &MinoShape) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y+pos.y >= FIELD_HEIGHT || x+pos.x >= FIELD_WIDTH {
                continue;
            }
            if mino[y][x] != block_kind::NONE && field[y+pos.y][x+pos.x] != block_kind::NONE {
                // テトリミノとフィールドのどちらも何かしらのブロックがある場合は衝突している
                return true;
            }
        }
    }
    false
}

// テトリミノをフィールドに固定する
pub fn fix_mino(Game {field, pos, mino, .. }: &mut Game) {
    for y in 0..4 {
        for x in 0..4 {
            if mino[y][x] != block_kind::NONE {
                field[y+pos.y][x+pos.x] = mino[y][x];
            }
        }
    }
}

// 消せるラインがあるなら削除し、段を下げる
// 消したライン数を返す
pub fn erase_line(field: &mut FieldSize) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT-2 {
        let mut can_erase = true;
        for x in 1..FIELD_WIDTH-1 {
            if field[y][x] == 0 {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            count += 1;
            for y2 in (2..=y).rev() {
                field[y2] = field[y2-1];
            }
        }
    }
    count
}

// テトリミノを指定した座標へ移動できるなら移動する
pub fn move_mino(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, &game.mino) {
        // posの座標を更新
        game.pos = new_pos;
    }
}

// テトリミノを生成する
// 生成に失敗した場合は`Err(())`を返す
pub fn spawn_mino(game: &mut Game) -> Result<(), ()> {
    // posの座標を初期値へ
    game.pos = Position::init();
    // ネクストキューから次のテトリミノを取り出す
    game.mino = game.next.pop_front().unwrap();
    if let Some(next) = game.next_buf.pop_front() {
        // バフからネクストキューに供給
        game.next.push_back(next);
    } else {
        // バフを生成
        game.next_buf = gen_mino_7().into();
        // バフからネクストキューに供給
        game.next.push_back(game.next_buf.pop_front().unwrap());
    }
    // 衝突チェック
    if is_collision(&game.field, &game.pos, &game.mino) {
        Err(())
    } else {
        Ok(())
    }
}

// スーパーローテーション処理
// スーパーローテーションできるなら、その座標を返す
fn super_rotation(field: &FieldSize, pos: &Position, mino: &MinoShape) -> Result<Position, ()> {
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
        if !is_collision(field, &pos, mino) {
            return Ok(pos);
        }
    }
    Err(())
}

// 左に90度回転する
#[allow(clippy::needless_range_loop)]
pub fn rotate_left(game: &mut Game) {
    let mut new_shape: MinoShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[4-1-x][y] = game.mino[y][x];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.mino = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos  = new_pos;
        game.mino = new_shape;
    }
}

// 右に90度回転する
#[allow(clippy::needless_range_loop)]
pub fn rotate_right(game: &mut Game) {
    let mut new_shape: MinoShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[y][x] = game.mino[4-1-x][y];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.mino = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos  = new_pos;
        game.mino = new_shape;
    }
}

// ハードドロップする
pub fn hard_drop(game: &mut Game) {
    while {
        let new_pos = Position {
            x: game.pos.x,
            y: game.pos.y + 1,
        };
        !is_collision(&game.field, &new_pos, &game.mino)
    }{
        game.pos.y += 1;
    }
    let new_pos = game.pos;
    move_mino(game, new_pos);
}

// ホールド処理
// - 1回目のホールドは現在のテトリミノをホールド
// - 2回目以降のホールドは現在のテトリミノとホールドを交換
// - 現在のテトリミノに対して既にホールドしている場合は何もしない
pub fn hold(game: &mut Game) {
    if game.holded {
        // 現在のテトリミノに対して既にホールドしている場合は早期リターン
        return;
    }
    if let Some(mut hold) = game.hold {
        // ホールドの交換
        std::mem::swap(&mut hold, &mut game.mino);
        game.hold = Some(hold);
        game.pos = Position::init();
    } else {
        // ホールドして、新たなテトリミノを生成
        game.hold = Some(game.mino);
        spawn_mino(game).ok();
    }
    // ホールド済のフラグを立てる
    game.holded = true;
}

// テトリミノ落下後の処理
pub fn landing(game: &mut Game) -> Result<(), ()> {
    // テトリミノをフィールドに固定
    fix_mino(game);
    // ラインの削除処理
    let line = erase_line(&mut game.field);
    // 消した段数によって得点を加算
    game.score += SCORE_TABLE[line];
    // 消した段数の合計を加算
    game.line += line;
    // テトリミノの生成
    spawn_mino(game)?;
    // 再ホールド可能にする
    game.holded = false;
    Ok(())
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
