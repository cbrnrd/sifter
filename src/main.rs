use clap::{Parser, ColorChoice};
mod generate;

#[derive(Parser)]
#[clap(
    name = "sifter",
    version = "0.1.0",
    about = "A tool for generating password lists from a configuration file",
    author = "Carter Brainerd",
    color = ColorChoice::Always
)]
struct Cli {

    #[command(subcommand)]
    generate: Subcommands,
}

#[derive(clap::Subcommand)]
enum Subcommands {
    /// generate a password list from a configuration file
    #[clap(name = "generate")]
    Generate {
        /// The configuration file to use for generation
        #[clap(short, long, default_value = "config.yaml")]
        config: Option<String>, // The config file to use

        /// The output file to use
        #[clap(short, long)]
        output: Option<String>, // The output file to use

    },
    /// Combine 2 wordlists
    #[clap(name = "combine")]
    Combo {
        /// The first wordlist to use
        #[clap(long)]
        wordlist1: String,

        /// The second wordlist to use
        #[clap(long)]
        wordlist2: String,
    }
}


fn main() {
    // Parse command line arguments using clap
    let args = Cli::parse();

    // Match the subcommand
    match args.generate {
        Subcommands::Generate { config, output } => {
            // Generate a password list from a configuration file
            generate::generate(config, output);
        },
        Subcommands::Combo { wordlist1, wordlist2 } => {
            // Combine 2 wordlists
            println!("Combining 2 wordlists");
            println!("Wordlist 1: {}", wordlist1);
            println!("Wordlist 2: {}", wordlist2);
        }
    }
}
