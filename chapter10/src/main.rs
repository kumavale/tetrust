mod block;
mod game;
mod play;
mod ai;
mod ga;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand)]
enum Mode {
    /// Run normal play
    Normal,
    /// Run auto play
    Auto,
    /// Learning with GeneticAlgorithm
    Learning,
}

fn main() {
    // コマンドライン引数の解析
    let cli = Cli::parse();
    match cli.mode {
        None |
        Some(Mode::Normal) => {
            // 通常プレイ
            play::normal();
        }
        Some(Mode::Auto) => {
            // オートプレイ
            play::auto();
        }
        Some(Mode::Learning) => {
            // 遺伝的アルゴリズムにて学習
            ga::learning();
        }
    }
}
