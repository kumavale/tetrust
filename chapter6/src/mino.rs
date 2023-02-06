use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// テトリミノの種類
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

// テトリミノの形状
pub const MINOS: [[[usize; 4]; 4]; 7] = [
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
