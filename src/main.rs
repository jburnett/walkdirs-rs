use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod walkdirs;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Walk {
        /// walks directory
        #[arg(short, long)]
        recurse: bool,
        #[arg(default_value("."))]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Walk { recurse, path }) => {

            let p = std::path::Path::new(path);

            match p.canonicalize() {
                Ok(pb) => {
                    if *recurse {
                        let wr = walkdirs::walkdirs(pb, u16::MAX);
                        dump_results(wr);
                    } else {
                        let wr = walkdirs::walkdirs(pb, 1);
                        dump_results(wr);
                    }

                },

                Err(e) => println!("Failed to canonicalize the path {}; {}", path, e),
            }
            
        },

        None => {}
    }

}


fn dump_results(wr: walkdirs::WalkResults) {
    let items_found = wr.items.len();
    let errs_found = wr.errs.len();

    for item in wr.items {
        println!("{:?}", item.path());
    }

    println!("\n> Found {} items and encountered {} errors <", items_found, errs_found);
}