mod mino;
mod game;
mod block;
mod play;
mod ai;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[arg(value_enum, default_value_t = Mode::Normal)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Run normal play
    Normal,
    /// Run auto play
    Auto,
}

fn main() {
    // コマンドライン引数の解析
    let cli = Cli::parse();
    match cli.mode {
        Mode::Normal => {
            // 通常プレイ
            play::normal();
        }
        Mode::Auto => {
            // オートプレイ
            play::auto();
        }
    }
}
