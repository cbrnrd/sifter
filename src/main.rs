use clap::{Parser, ColorChoice};
mod generate;
mod combine;

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
        /// An array of file paths to wordlists
        #[clap(required = true)]
        wordlists: Vec<String>, 

        /// Whether to remove duplicates
        /// (default: false)
        #[clap(short, long, default_value = "false")]
        remove_duplicates: bool,

        /// Whether to sort the wordlist
        /// (default: false)
        #[clap(short, long, default_value = "false")]
        sort: bool,

        /// The output file to use
        /// (default: stdout)
        #[clap(short, long)]
        output: Option<String>, // The output file to use


        /// The delimiter to split the input files by
        /// (default: \n)
        #[clap(short, long)]
        delimiter: Option<String>,

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
        Subcommands::Combo { wordlists,
                             remove_duplicates,
                             sort,
                             output,
                             delimiter } => {
            combine::combine(wordlists, remove_duplicates, sort, output, delimiter);
        }
    }
}
