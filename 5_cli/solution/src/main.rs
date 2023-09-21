mod chifoumi;
mod greetings;

use chifoumi::{play, Game};
use greetings::greets;

use clap::{Parser, Subcommand};

use crate::chifoumi::random_game;

#[derive(Parser)]
#[clap(
    author = "Julien Rollin",
    version = "1.0.0",
    about = "Crabby cli",
    long_about = None
)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greets with name
    Greets {
        /// Name of the person to greet
        #[clap(short, long, value_parser)]
        name: String,
    },
    /// chifoumi with players
    Chifoumi {
        #[clap(short = 'a', long, value_enum)]
        one: Game,
        /// random game if not provided
        #[clap(short = 'b', long, value_enum)]
        two: Option<Game>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Chifoumi { one, two } => match two {
            // player two provided
            Some(p) => {
                let result = play(one.clone(), p.clone());
                println!("p1: {:?} vs p2: {:?} => {:?}", one, p, result)
            }
            None => {
                let random = random_game();
                let result = play(one.clone(), random.clone());
                println!("p1: {:?} vs p2: {:?} => {:?}", one, random, result)
            }
        },
        Commands::Greets { name } => {
            println!("{}", greets(name));
        }
    }
}
