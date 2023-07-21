use std::io;

use cli::{Cli, Commands};

mod cli;
mod subcommands;

use clap::Parser;
use feature_flags::db::get_db_rc;

fn main() {
    let cli_app = Cli::parse();

    let db = get_db_rc();
    let stdout = io::stdout();
    let writer = stdout.lock();

    match cli_app.command {
        Commands::Get(args) => {
            if let Some(name) = args.name {
                subcommands::get_flags::get_flag(db, name, writer);
            } else if args.all {
                subcommands::all_flags::all_flags(db, writer);
            }
        }
        Commands::Create(args) => {
            // All new flags are true
            subcommands::create_flags::create_flag(
                db,
                args.key,
                args.name,
                "{\"data_type\": \"boolean\", \"value\": true}".to_string(),
                writer,
            );
        }
        Commands::Update(args) => {
            let name = args.name;

            subcommands::update_flags::update_flag(db, args.key, name, args.value.unwrap(), writer);
        }
        Commands::Delete(args) => {
            subcommands::delete_flags::delete_flag(db, args.key, args.name, writer);
        }
    };
}

#[cfg(test)]
mod tests {}
