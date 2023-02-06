use std::ops::Index;
use std::thread;
use crate::game::*;
use crate::nn::eval;
use getch_rs::{Getch, Key};
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

// 遺伝子集合の個体数
const POPULATION: usize = 10;
// 最大世代数
const GENERATION_MAX: usize = 10;
// nライン消したら終了(1個体の実行終了条件)
const LINE_COUNT_MAX: usize = 256;
// 交叉率
const CROSSOVER_RATE: usize = 70;
const CROSSOVER_LEN: usize = (POPULATION as f64 * (CROSSOVER_RATE as f64 / 100.)) as usize;
// 突然変異率
const MUTATION_RATE: usize = 10;
const MUTATION_LEN: usize = (POPULATION as f64 * (MUTATION_RATE  as f64 / 100.)) as usize;
// 再生確率
const SELECTION_RATE: usize = 20;
const SELECTION_LEN: usize = (POPULATION as f64 * (SELECTION_RATE as f64 / 100.)) as usize;
// 確率の割り振りが正しいことをコンパイル時に保証する
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(CROSSOVER_RATE + MUTATION_RATE + SELECTION_RATE == 100);
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(CROSSOVER_LEN  + MUTATION_LEN  + SELECTION_LEN  == POPULATION);

// 遺伝子の種類
pub enum GenomeKind {
    Line,
    HeightMax,
    HeightDiff,
    DeadSpace,
}

// 遺伝子配列
pub type GenoSeq = [u8; 4];
impl Index<GenomeKind> for GenoSeq {
    type Output = u8;
    fn index(&self, kind: GenomeKind) -> &Self::Output {
        &self[kind as usize]
    }
}

// 個体
#[derive(Clone, Copy)]
struct Individual {
    geno: GenoSeq,
    score: usize,
}

impl Distribution<Individual> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Individual {
        Individual {
            geno: rand::random::<GenoSeq>(),
            score: 0,
        }
    }
}

// 学習
#[allow(clippy::needless_range_loop)]
pub fn learning() -> ! {
    let _ = thread::spawn(|| {
        // 世代を初期生成
        let mut inds = rand::random::<[Individual; POPULATION]>();
        for gen in 1..=GENERATION_MAX {
            let mut children = vec![];
            println!("{gen}世代目:");
            for i in 0..POPULATION {
                // 新しいスレッドを起動
                children.push(thread::spawn(move || {
                    let mut game = Game::new();
                    // nライン消したら終了
                    while game.line < LINE_COUNT_MAX {
                        // 指定した遺伝子で評価後のエリート個体を取得
                        let elite = eval(&game, &inds[i].geno);
                        game = elite;
                        // エリート個体のテトリミノを落下
                        if landing(&mut game).is_err() {
                            break;
                        }
                    }
                    // 個体の最終スコアを記録
                    inds[i].score = game.score;
                    println!("{i}: {:?} => {}", inds[i].geno, game.score);
                }));
            }
            for child in children {
                // 子スレッドが終了するのを待つ
                let _ = child.join();
            }
            // 次世代の生成
            let next_genos = gen_next_generation(&inds);
            // 世代交代
            inds.iter_mut()
                .map(|i| &mut i.geno)
                .zip(next_genos)
                .for_each(|(now, next)| *now = next);
        }
        // 学習終了
        quit();
    });

    // キー入力処理
    let g = Getch::new();
    loop {
        // `q`キーで終了
        if let Ok(Key::Char('q')) = g.getch() {
            quit();
        }
    }
}

// 次世代を生成
fn gen_next_generation(inds: &[Individual]) -> [GenoSeq; POPULATION] {
    let mut rng = rand::thread_rng();
    let mut genos = vec![];
    genos.extend_from_slice(&crossover(inds));  // 交叉
    genos.extend_from_slice(&mutation(inds));   // 突然変異
    genos.extend_from_slice(&selection(inds));  // 選択
    genos.shuffle(&mut rng);
    genos.try_into().unwrap()
}

// 交叉(二点交叉)
fn crossover(inds: &[Individual]) -> [GenoSeq; CROSSOVER_LEN] {
    let mut genos = inds.iter().map(|i|i.geno).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    for i in (0..genos.len()-1).step_by(2) {
        let mut geno1 = genos[i];
        let mut geno2 = genos[i+1];
        let point1 = rng.gen_range(0..4);
        let point2 = rng.gen_range(point1..4);
        mem_swap_range(&mut geno1, &mut geno2, point1..=point2);
        genos[i]   = geno1;
        genos[i+1] = geno2;
    }
    genos.shuffle(&mut rng);
    genos[..CROSSOVER_LEN].try_into().unwrap()
}

// 突然変異
fn mutation(inds: &[Individual]) -> [GenoSeq; MUTATION_LEN] {
    let mut genos = inds.iter().map(|i|i.geno).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    genos.shuffle(&mut rng);
    for geno in genos.iter_mut().take(MUTATION_LEN) {
        geno[rng.gen_range(0..4)] = rand::random();
    }
    genos[..MUTATION_LEN].try_into().unwrap()
}

// 選択
// スコアの高い順にして返す
fn selection(inds: &[Individual]) -> [GenoSeq; SELECTION_LEN] {
    let mut new_inds = inds.to_vec();
    new_inds.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    new_inds.iter()
        .map(|i| i.geno)
        .collect::<Vec<_>>()[..SELECTION_LEN]
        .try_into()
        .unwrap()
}

// 指定した範囲のデータを交換する
fn mem_swap_range<T>(x: &mut [T], y: &mut [T], range: std::ops::RangeInclusive<usize>) {
    for i in range {
        std::mem::swap(&mut x[i], &mut y[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_swap_range() {
        let tests = [
            (0..=0, [[5,2,3,4], [1,6,7,8]]),
            (0..=1, [[5,6,3,4], [1,2,7,8]]),
            (1..=1, [[1,6,3,4], [5,2,7,8]]),
            (1..=2, [[1,6,7,4], [5,2,3,8]]),
            (1..=3, [[1,6,7,8], [5,2,3,4]]),
            (0..=3, [[5,6,7,8], [1,2,3,4]]),
        ];
        for (range, [geno1_expect, geno2_expect]) in tests {
            let mut geno1 = [1,2,3,4];
            let mut geno2 = [5,6,7,8];
            mem_swap_range(&mut geno1, &mut geno2, range);
            assert_eq!(geno1, geno1_expect);
            assert_eq!(geno2, geno2_expect);
        }
    }
}
