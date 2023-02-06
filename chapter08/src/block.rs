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

// 表示ブロックの色/文字定義
pub const COLOR_TABLE: [&str; 10] = [
    "\x1b[48;2;000;000;000m  ",  // 何もなし
    "\x1b[48;2;127;127;127m__",  // 壁
    "\x1b[48;2;000;000;000m[]",  // ゴースト
    "\x1b[48;2;000;255;255m__",  // I
    "\x1b[48;2;255;255;000m__",  // O
    "\x1b[48;2;000;255;000m__",  // S
    "\x1b[48;2;255;000;000m__",  // Z
    "\x1b[48;2;000;000;255m__",  // J
    "\x1b[48;2;255;127;000m__",  // L
    "\x1b[48;2;255;000;255m__",  // T
];
