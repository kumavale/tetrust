use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng,
    Rng,
};
use crate::block::block_kind::{I, O, S, Z, J, L, T};

// テトリミノの種類
const MINO_KIND_MAX: usize = 7;
#[derive(Clone, Copy)]
pub enum MinoKind {
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

pub type MinoShape = [[usize; 4]; 4];

// テトリミノの形状
pub const MINOS: [MinoShape; MINO_KIND_MAX] = [
    // Iミノ
    [
        [0,0,0,0],
        [0,0,0,0],
        [I,I,I,I],
        [0,0,0,0],
    ],
    // Oミノ
    [
        [0,0,0,0],
        [0,O,O,0],
        [0,O,O,0],
        [0,0,0,0],
    ],
    // Sミノ
    [
        [0,0,0,0],
        [0,S,S,0],
        [S,S,0,0],
        [0,0,0,0],
    ],
    // Zミノ
    [
        [0,0,0,0],
        [Z,Z,0,0],
        [0,Z,Z,0],
        [0,0,0,0],
    ],
    // Jミノ
    [
        [0,0,0,0],
        [J,0,0,0],
        [J,J,J,0],
        [0,0,0,0],
    ],
    // Lミノ
    [
        [0,0,0,0],
        [0,0,L,0],
        [L,L,L,0],
        [0,0,0,0],
    ],
    // Tミノ
    [
        [0,0,0,0],
        [0,T,0,0],
        [T,T,T,0],
        [0,0,0,0],
    ],
];

// シャッフルされた7種のテトリミノを生成
pub fn gen_mino_7() -> [MinoShape; MINO_KIND_MAX] {
    let mut rng = thread_rng();
    let mut que = [
        MinoKind::I,
        MinoKind::O,
        MinoKind::S,
        MinoKind::Z,
        MinoKind::J,
        MinoKind::L,
        MinoKind::T,
    ];
    que.shuffle(&mut rng);
    que.map(|mino| MINOS[mino as usize])
}
