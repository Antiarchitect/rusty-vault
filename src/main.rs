extern crate docopt;
extern crate rustc_serialize;
extern crate rusty_vault;

use docopt::Docopt;
use rusty_vault as vault;

const USAGE: &'static str = "
Rusty Vault.

Usage:
  rusty_vault <external-id> <data-string>
  rusty_vault <external-id>
  rusty_vault --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_external_id: String,
    arg_data_string: Option<String>
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then( |d| d.decode() )
                            .unwrap_or_else( |e| e.exit() );

    let external_id = args.arg_external_id;
    let data_string = args.arg_data_string;

    match data_string {
        Some(value) => dump(external_id, value.into_bytes()),
        None => load(external_id)
    }
}

fn dump(external_id: String, data: Vec<u8>) {
    match vault::dump(&external_id, data) {
        Ok(_) => println!("Object is successfully stored!"),
        Err(error) => println!("An error has occurred: {}", error)
    }
}

fn load(external_id: String) {
    match vault::load(&external_id)  {
        Ok(plaintext) => println!("Data: {}", String::from_utf8(plaintext).unwrap()),
        Err(error) => println!("An error has occurred: {:?}", error)
    }
}
