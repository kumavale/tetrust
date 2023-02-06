mod mino;
mod game;

use std::sync::{Arc, Mutex};
use std::{thread, time};
use getch_rs::{Getch, Key};
use game::*;

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    // フィールドを描画
    draw(&game.lock().unwrap());

    // 自然落下処理
    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || {
            loop {
                // 1秒間スリーブする
                thread::sleep(time::Duration::from_millis(1000));
                // 自然落下
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                if !is_collision(&game.field, &new_pos, game.mino) {
                    // posの座標を更新
                    game.pos = new_pos;
                } else {
                    // テトリミノをフィールドに固定
                    fix_mino(&mut game);
                    // ラインの削除処理
                    erase_line(&mut game.field);
                    // posの座標を初期値へ
                    game.pos = Position { x: 4, y: 0 };
                    // テトリミノをランダム生成
                    game.mino = rand::random();
                }
                // フィールドを描画
                draw(&game);
            }
        });
    }

    // キー入力処理
    let g = Getch::new();
    loop {
        // キー入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                move_mino(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_mino(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_mino(&mut game, new_pos);
                draw(&game);
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
