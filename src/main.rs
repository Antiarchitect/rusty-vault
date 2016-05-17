extern crate docopt;
extern crate rustc_serialize;
extern crate rusty_vault;

use docopt::Docopt;
use rusty_vault::Vault;

use rusty_vault::storages::filesystem as fs_storage;
use rusty_vault::storages::postgresql as pg_storage;

const USAGE: &'static str = "
Rusty Vault.

Usage:
  rusty_vault <external-id> <data-string>
  rusty_vault <external-id> [--delete]
  rusty_vault --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --delete -d   Delete object.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_external_id: String,
    arg_data_string: Option<String>,
    flag_delete: bool
}

fn main() {
    static KEYS_STORAGE: fs_storage::Storage = fs_storage::Storage { path: "/home/andrey/Documents/storages/keys" };
    static DATA_STORAGE: pg_storage::Storage = pg_storage::Storage { connection_url: "postgresql://medm:password@localhost/rusty_vault_data", table_name: "data" };
    static MAPS_STORAGE: pg_storage::Storage = pg_storage::Storage { connection_url: "postgresql://medm:password@localhost/rusty_vault_maps", table_name: "maps" };
    let vault = Vault {
        keys: &KEYS_STORAGE,
        data: &DATA_STORAGE,
        maps: &MAPS_STORAGE
    };
    let args: Args = Docopt::new(USAGE)
                            .and_then( |d| d.decode() )
                            .unwrap_or_else( |e| e.exit() );

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
