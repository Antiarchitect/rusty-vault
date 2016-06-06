extern crate docopt;
extern crate rustc_serialize;
extern crate rusty_vault;

use docopt::Docopt;
use rusty_vault::Vault;

use rusty_vault::config::Config;

const USAGE: &'static str = "
Rusty Vault.

Usage:
  rusty_vault --config=<config> <external-id> <data-string>
  rusty_vault --config=<config> <external-id> [--delete]
  rusty_vault --version

Options:
  -h --help         Show this screen.
  --config=<config> Path to the config file.
  --version         Show version.
  --delete -d       Delete object.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_external_id: String,
    arg_data_string: Option<String>,
    flag_delete: bool,
    flag_config: String
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then( |d| d.decode() ).unwrap_or_else( |e| e.exit() );
    let config = Config::from_yaml_file(args.flag_config);
    let vault = Vault::from_config(&config);

//    let vault = Vault::new(
//        fs_storage::Storage { path: "/home/andrey/Documents/storages/keys" },
//        pg_storage::Storage { connection_url: "postgresql://medm:password@localhost/rusty_vault_data", table_name: "data" },
//        pg_storage::Storage { connection_url: "postgresql://medm:password@localhost/rusty_vault_maps", table_name: "maps" }
//    );

    let external_id = args.arg_external_id;
    match args.arg_data_string {
        Some(data) => match vault.dump(&external_id, data.into_bytes()) {
            Ok(_) => println!("Object is successfully stored!"),
            Err(error) => println!("An error has occurred: {}", error)
        },
        None =>
            match args.flag_delete {
                false => match vault.load(&external_id) {
                    Ok(Some(plaintext)) => println!("Data: {}", String::from_utf8(plaintext).unwrap()),
                    Ok(None) => println!("Object {} was not found.", external_id),
                    Err(error) => println!("An error has occurred: {}", error)
                },
                true => match vault.delete(&external_id) {
                    Ok(Some(())) => println!("Object {} is successfully deleted!", external_id),
                    Ok(None) => println!("Object {} was not found.", external_id),
                    Err(error) => println!("An error has occurred: {}", error)
                }
            }
    }
}
