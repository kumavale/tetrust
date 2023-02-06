use std::sync::{Arc, Mutex};
use std::{thread, time};
use getch_rs::{Getch, Key};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

const FIELD_WIDTH:  usize = 12;
const FIELD_HEIGHT: usize = 22;

type FieldSize = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

// テトリミノの種類
#[derive(Clone, Copy)]
enum MinoKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T
}

impl Distribution<MinoKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MinoKind {
        match rng.gen_range(0..=6) {
            0 => MinoKind::I,
            1 => MinoKind::O,
            2 => MinoKind::S,
            3 => MinoKind::Z,
            4 => MinoKind::J,
            5 => MinoKind::L,
            _ => MinoKind::T,
        }
    }
}

// テトリミノの形状
const MINOS: [[[usize; 4]; 4]; 7] = [
    // Iミノ
    [
        [0,0,0,0],
        [0,0,0,0],
        [1,1,1,1],
        [0,0,0,0],
    ],
    // Oミノ
    [
        [0,0,0,0],
        [0,1,1,0],
        [0,1,1,0],
        [0,0,0,0],
    ],
    // Sミノ
    [
        [0,0,0,0],
        [0,1,1,0],
        [1,1,0,0],
        [0,0,0,0],
    ],
    // Zミノ
    [
        [0,0,0,0],
        [1,1,0,0],
        [0,1,1,0],
        [0,0,0,0],
    ],
    // Jミノ
    [
        [0,0,0,0],
        [1,0,0,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
    // Lミノ
    [
        [0,0,0,0],
        [0,0,1,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
    // Tミノ
    [
        [0,0,0,0],
        [0,1,0,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
];

struct Position {
    x: usize,
    y: usize,
}

// テトリミノがフィールドに衝突する場合は`ture`を返す
fn is_collision(field: &FieldSize, pos: &Position, mino: MinoKind) -> bool {
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

// フィールドを描画する
fn draw(field: &FieldSize, pos: &Position, mino: MinoKind) {
    // 描画用フィールドの生成
    let mut field_buf = field.clone();
    // 描画用フィールドにテトリミノの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            field_buf[y+pos.y][x+pos.x] |= MINOS[mino as usize][y][x];
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

fn main() {
    let field = Arc::new(Mutex::new([
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
    ]));
    let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    let mino = Arc::new(Mutex::new(rand::random::<MinoKind>()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    // フィールドを描画
    draw(&field.lock().unwrap(), &pos.lock().unwrap(), *mino.lock().unwrap());

    // 自然落下処理
    {
        let pos = Arc::clone(&pos);
        let field = Arc::clone(&field);
        let mino = Arc::clone(&mino);
        let _ = thread::spawn(move || {
            loop {
                // 1秒間スリーブする
                thread::sleep(time::Duration::from_millis(1000));
                // 自然落下
                let mut pos = pos.lock().unwrap();
                let mut field = field.lock().unwrap();
                let mut mino = mino.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    // posの座標を更新
                    *pos = new_pos;
                } else {
                    // テトリミノをフィールドに固定
                    for y in 0..4 {
                        for x in 0..4 {
                            field[y+pos.y][x+pos.x] |= MINOS[*mino as usize][y][x];
                        }
                    }
                    // ラインの削除処理
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
                    // posの座標を初期値へ
                    *pos = Position { x: 4, y: 0 };
                    *mino = rand::random();
                }
                // フィールドを描画
                draw(&field, &pos, *mino);
            }
        });
    }

    // キー入力処理
    let g = Getch::new();
    loop {
        // キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let mino = mino.lock().unwrap();
                let new_pos = Position {
                    x: pos.x.checked_sub(1).unwrap_or_else(|| pos.x),
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    // posの座標を更新
                    *pos = new_pos;
                }
                // フィールドを描画
                draw(&field, &pos, *mino);
            }
            Ok(Key::Down) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let mino = mino.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    // posの座標を更新
                    *pos = new_pos;
                }
                // フィールドを描画
                draw(&field, &pos, *mino);
            }
            Ok(Key::Right) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let mino = mino.lock().unwrap();
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    // posの座標を更新
                    *pos = new_pos;
                }
                // フィールドを描画
                draw(&field, &pos, *mino);
            }
            Ok(Key::Char('q')) => {
                // カーソルを再表示
                println!("\x1b[?25h");
                return;
            }
            _ => (),  // 何もしない
        }
    }
}
