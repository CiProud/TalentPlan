use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the value of the key
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Rm {
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut kvs = kvs::KvStore::new();
    match cli.command {
        Commands::Get { key } => {
            kvs.get(key);
        }
        Commands::Set { key, value } => {
            kvs.set(key, value);
        }
        Commands::Rm { key } => {
            kvs.remove(key);
        }
    }
}
