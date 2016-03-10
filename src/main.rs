extern crate docopt;
extern crate rustc_serialize;
extern crate rusty_vault;

use docopt::Docopt;
use rusty_vault as vault;

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
    let args: Args = Docopt::new(USAGE)
                            .and_then( |d| d.decode() )
                            .unwrap_or_else( |e| e.exit() );

    match args.arg_data_string {
        Some(data) => dump(args.arg_external_id, data.into_bytes()),
        None =>
            match args.flag_delete {
                false => load(args.arg_external_id),
                true => delete(args.arg_external_id)
            }
    }
}

fn dump(external_id: String, data: Vec<u8>) {
    match vault::dump(&external_id, data) {
        Ok(_) => println!("Object is successfully stored!"),
        Err(error) => println!("An error has occurred: {}", error)
    }
}

fn load(external_id: String) {
    match vault::load(&external_id) {
        Ok(plaintext) => println!("Data: {}", String::from_utf8(plaintext).unwrap()),
        Err(error) => println!("An error has occurred: {}", error)
    }
}

fn delete(external_id: String) {
    match vault::delete(&external_id) {
        Ok(Some(())) => println!("Object {} is successfully deleted!", external_id),
        Ok(None) => println!("Object {} was not found.", external_id),
        Err(error) => println!("An error has occurred: {}", error)
    }
}
