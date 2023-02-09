use crate::game::*;
use crate::block::block_kind;
use crate::ga::{GenomeKind, GenoSeq};

// 評価して、一番優秀な個体を返す
pub fn eval(game: &Game, weight: &GenoSeq) -> Game {
    // エリートブロック (Game, score)
    let mut elite = (game.clone(), 0f64);

    // ホールド有無
    for do_hold in [true, false] {
        let mut game = game.clone();
        if do_hold {
            hold(&mut game);
        }
        // 全回転
        for rotate_count in 0..=3 {
            let mut game = game.clone();
            for _ in 0..=rotate_count {
                // 回転処理
                rotate_right(&mut game);
            }
            // 全横移動
            for dx in -4..=5 {
                let mut game = game.clone();
                // 移動処理
                let new_pos = Position {
                    x: match game.pos.x as isize + dx {
                        (..=0) => 0,
                        x => x as usize,
                    },
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                hard_drop(&mut game);
                fix_block(&mut game);

                // インプット情報の取得
                let line        = erase_line_count(&game.field);  // 消せるライン数
                let height_max  = field_height_max(&game.field);  // フィールドの高さ
                let height_diff = diff_in_height(&game.field);    // 高低差
                let dead_space  = dead_space_count(&game.field);  // デッドスペース数

                // 正規化
                let mut line        =       normalization(line as f64, 0.0, 4.0);
                let mut height_max  = 1.0 - normalization(height_max as f64, 0.0, 20.0);
                let mut height_diff = 1.0 - normalization(height_diff as f64, 0.0, 200.0);
                let mut dead_space  = 1.0 - normalization(dead_space as f64, 0.0, 200.0);

                // インプット情報に重み付け
                line        *= weight[GenomeKind::Line] as f64;
                height_max  *= weight[GenomeKind::HeightMax] as f64;
                height_diff *= weight[GenomeKind::HeightDiff] as f64;
                dead_space  *= weight[GenomeKind::DeadSpace] as f64;

                // インプット情報を評価
                let score = line + height_max + height_diff + dead_space;
                if elite.1 < score {
                    // 一番良い個体を記録
                    elite.0 = game;
                    elite.1 = score;
                }
            }
        }
    }
    elite.0
}

// 正規化(Min-Max normalization)する
fn normalization(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min)
}

// 消せるライン数を返す
#[allow(clippy::needless_range_loop)]
fn erase_line_count(field: &Field) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT-2 {
        let mut can_erase = true;
        for x in 2..FIELD_WIDTH-2 {
            if field[y][x] == block_kind::NONE {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            count += 1;
        }
    }
    count
}

// フィールドの一番高いブロックの高さを返す
// ブロックが何もないときは「0」
// ブロックが積みあがっていくにつれ、数値は増える
#[allow(clippy::needless_range_loop)]
fn field_height_max(field: &Field) -> usize {
    for y in 1..FIELD_HEIGHT-2 {
        for x in 2..FIELD_WIDTH-2 {
            if field[y][x] != block_kind::NONE {
                return FIELD_HEIGHT - y - 1;
            }
        }
    }
    unreachable!()
}

// フィールドの高低差の合計を返す
#[allow(clippy::needless_range_loop)]
pub fn diff_in_height(field: &Field) -> usize {
    let mut diff = 0;
    let mut top = [0; FIELD_WIDTH-4];
    // 各列の一番上の高さを求める
    for x in 2..FIELD_WIDTH-2 {
        for y in 1..FIELD_HEIGHT-2 {
            if field[y][x] != block_kind::NONE {
                top[x-2] = FIELD_HEIGHT - y - 1;
                break;
            }
        }
    }
    // 右隣との差を合計する
    for i in 0..FIELD_WIDTH-4-1 {
        diff += top[i].abs_diff(top[i+1]);
    }
    diff
}

// デッドスペース数を返す
pub fn dead_space_count(field: &Field) -> usize {
    let mut count = 0;
    for y in (1..FIELD_HEIGHT-2).rev() {
        for x in 2..FIELD_WIDTH-2 {
            // 各列の一番下の何もない座標
            if field[y][x] == block_kind::NONE {
                for y2 in (2..y).rev() {
                    // 上にブロックがあるならデッドスペース
                    if field[y2][x] != block_kind::NONE {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    count
}
