use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod walkdirs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    // name: Option<String>,

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

    // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {}", name);
    // }

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
                        println!("Recursively walk {:?}...", pb);
                        println!("\tas_os_str: {:?}", pb.as_os_str());

                        walkdirs::walk(pb, u16::MAX);
                    } else {
                        println!("Only walk {:?}...", pb);
                        println!("\tas_os_str: {:?}", pb.as_os_str());

                        walkdirs::walk(pb, 1);
                    }

                },

                Err(e) => println!("Failed to canonicalize the path {}; {}", path, e),
            }
            
        },

        None => {}
    }

}