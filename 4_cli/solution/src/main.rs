mod chifoumi;
mod greetings;

use chifoumi::{play, Game};
use greetings::greets;

use clap::{Parser, Subcommand};

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
        #[clap(short = 'b', long, value_enum)]
        two: Game,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Chifoumi { one, two } => {
            let result = play(one.clone(), two.clone());
            println!("p1 vs p2 : {:?}", result);
        }
        Commands::Greets { name } => {
            println!("{}", greets(&name));
        }
    }
}
