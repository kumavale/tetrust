use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng,
    Rng,
};
use block_kind::{I, O, S, Z, J, L, T};

pub type BlockColor = usize;

pub mod block_kind {
    pub const NONE:  super::BlockColor = 0;
    pub const WALL:  super::BlockColor = 1;
    pub const GHOST: super::BlockColor = 2;
    pub const I:     super::BlockColor = 3;
    pub const O:     super::BlockColor = 4;
    pub const S:     super::BlockColor = 5;
    pub const Z:     super::BlockColor = 6;
    pub const J:     super::BlockColor = 7;
    pub const L:     super::BlockColor = 8;
    pub const T:     super::BlockColor = 9;
}

// ブロックの種類
const BLOCK_KIND_MAX: usize = 7;
#[derive(Clone, Copy)]
pub enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

impl Distribution<BlockKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockKind {
        match rng.gen_range(0..=6) {
            0 => BlockKind::I,
            1 => BlockKind::O,
            2 => BlockKind::S,
            3 => BlockKind::Z,
            4 => BlockKind::J,
            5 => BlockKind::L,
            _ => BlockKind::T,
        }
    }
}

// ブロックの形状
pub type BlockShape = [[usize; 4]; 4];
pub const BLOCKS: [BlockShape; BLOCK_KIND_MAX] = [
    // Iブロック
    [
        [0,0,0,0],
        [0,0,0,0],
        [I,I,I,I],
        [0,0,0,0],
    ],
    // Oブロック
    [
        [0,0,0,0],
        [0,O,O,0],
        [0,O,O,0],
        [0,0,0,0],
    ],
    // Sブロック
    [
        [0,0,0,0],
        [0,S,S,0],
        [S,S,0,0],
        [0,0,0,0],
    ],
    // Zブロック
    [
        [0,0,0,0],
        [Z,Z,0,0],
        [0,Z,Z,0],
        [0,0,0,0],
    ],
    // Jブロック
    [
        [0,0,0,0],
        [J,0,0,0],
        [J,J,J,0],
        [0,0,0,0],
    ],
    // Lブロック
    [
        [0,0,0,0],
        [0,0,L,0],
        [L,L,L,0],
        [0,0,0,0],
    ],
    // Tブロック
    [
        [0,0,0,0],
        [0,T,0,0],
        [T,T,T,0],
        [0,0,0,0],
    ],
];

// シャッフルされた7種のブロックを生成
pub fn gen_block_7() -> [BlockShape; BLOCK_KIND_MAX] {
    let mut rng = thread_rng();
    let mut que = [
        BlockKind::I,
        BlockKind::O,
        BlockKind::S,
        BlockKind::Z,
        BlockKind::J,
        BlockKind::L,
        BlockKind::T,
    ];
    que.shuffle(&mut rng);
    que.map(|block| BLOCKS[block as usize])
}
