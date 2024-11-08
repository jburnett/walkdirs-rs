use std::path::PathBuf;
use clap::{Parser, Subcommand};
// use walkdir::DirEntry;
use walkdirs::WalkResults;

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
                    let wr: WalkResults;
                    if *recurse {
                        wr = walkdirs::walkdirs(pb, usize::MAX);
                    } else {
                        wr = walkdirs::walkdirs(pb, 1);
                        // dump_results(wr);
                    }
                    dump_results(&wr);
                    show_largest(&wr);
                },

                Err(e) => println!("Failed to canonicalize the path {}; {}", path, e),
            }
            
        },

        None => {}
    }

}


fn dump_results(wr: &walkdirs::WalkResults) {
    let items_found = wr.items.len();
    let errs_found = wr.errs.len();

    let walker = wr.items.iter();
    for item in walker {
        println!("{:?}", item.path().display());
    }

    println!("\n> Found {} items and encountered {} errors <", items_found, errs_found);
}

fn show_largest(wr: &walkdirs::WalkResults) {
    // let mut idx : usize = 0;
    let mut largest: u64 = 0;

    for item in wr.items.iter() {
        if item.file_type().is_file() && item.metadata().unwrap().len() > largest {
            largest = item.metadata().unwrap().len();
        }
    }

    println!("Largest found: {:?}", largest);
}