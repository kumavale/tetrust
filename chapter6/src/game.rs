use crate::mino::{MinoKind, MINOS};

pub const FIELD_WIDTH:  usize = 12;
pub const FIELD_HEIGHT: usize = 22;

pub type FieldSize = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn init() -> Position {
        Position {
            x: 4,
            y: 0,
        }
    }
}

pub struct Game {
    pub field: FieldSize,
    pub pos: Position,
    pub mino: MinoKind,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: [
                [1,1,1,0,0,0,0,0,0,1,1,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,1],
                [1,1,1,1,1,1,1,1,1,1,1,1],
            ],
            pos: Position::init(),
            mino: rand::random::<MinoKind>(),
        }
    }
}

// フィールドを描画する
#[allow(clippy::needless_range_loop)]
pub fn draw(Game { field, pos, mino }: &Game) {
    // 描画用フィールドの生成
    let mut field_buf = *field;
    // 描画用フィールドにテトリミノの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            field_buf[y+pos.y][x+pos.x] |= MINOS[*mino as usize][y][x];
        }
    }
    // フィールドを描画
    println!("\x1b[H");  // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

// テトリミノがフィールドに衝突する場合は`ture`を返す
pub fn is_collision(field: &FieldSize, pos: &Position, mino: MinoKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y+pos.y >= FIELD_HEIGHT || x+pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y+pos.y][x+pos.x] & MINOS[mino as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

// テトリミノをフィールドに固定する
pub fn fix_mino(Game {field, pos, mino }: &mut Game) {
    for y in 0..4 {
        for x in 0..4 {
            field[y+pos.y][x+pos.x] |= MINOS[*mino as usize][y][x];
        }
    }
}

// 消せるラインがあるなら削除し、段を下げる
pub fn erase_line(field: &mut FieldSize) {
    for y in 1..FIELD_HEIGHT-1 {
        let mut can_erase = true;
        for x in 0..FIELD_WIDTH {
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

// テトリミノを指定した座標へ移動できるなら移動する
pub fn move_mino(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, game.mino) {
        // posの座標を更新
        game.pos = new_pos;
    }
}
