mod block;
mod game;
mod play;
mod ai;
mod ga;

use clap::{
    Args, Parser, Subcommand,
    error::{ErrorKind, ContextKind, ContextValue},
};

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
    Auto(Auto),
    /// Learning with GeneticAlgorithm
    Learning,
}

#[derive(Args)]
struct Auto {
    /// Specify gene sequence [default: [100,1,10,100]]
    #[arg(short, long)]
    genome: Option<String>,
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
        Some(Mode::Auto(args)) => {
            // オートプレイ
            let genome = match args.genome {
                Some(genome) => {
                    genome.trim_matches(|c|!char::is_numeric(c))
                        .split(|c|!char::is_numeric(c))
                        .map(|c|c.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>()
                        .try_into()
                        .unwrap_or_else(|_| {
                            let cmd = clap::Command::new("tetrust");
                            let mut err = clap::Error::new(ErrorKind::InvalidValue)
                                .with_cmd(&cmd);
                            err.insert(ContextKind::InvalidArg, ContextValue::String("--genome".to_owned()));
                            err.insert(ContextKind::InvalidValue, ContextValue::String(genome));
                            err.exit();
                        })
                }
                None => [100,1,10,100],
            };
            play::auto(genome);
        }
        Some(Mode::Learning) => {
            // 遺伝的アルゴリズムにて学習
            ga::learning();
        }
    }
}
