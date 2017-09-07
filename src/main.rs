extern crate rustc_serialize;

extern crate structopt;
#[macro_use] extern crate structopt_derive;
use structopt::StructOpt;

extern crate rusty_vault;
use rusty_vault::Vault;
use rusty_vault::storages::filesystem as storage_fs;
//use rusty_vault::storages::postgresql as storage_pg;

#[derive(StructOpt, Debug)]
#[structopt(name = "rusty-vault", about = "Rusty Vault CLI usage.")]
struct Cli {
    /// External Identifier of the (future) encrypted object
    #[structopt(help = "External Identifier of the (future) encrypted object")]
    external_id: String,

    /// Data string to be encrypted
    #[structopt(help = "Data string to be encrypted")]
    data_string: Option<String>,

    /// Option to delete object by its external_id
    #[structopt(long = "delete", short = "d")]
    delete: bool
}

fn main() {
    let args = Cli::from_args();

//    let vault = Vault::new(
//        storage_fs::Storage { path: "/home/andrey/Documents/storages/keys" },
//        storage_pg::Storage { connection_url: "postgresql://medm:password@localhost/rusty_vault_data", table_name: "data" },
//        storage_pg::Storage { connection_url: "postgresql://medm:password@localhost/rusty_vault_maps", table_name: "maps" }
//    );

    let vault = Vault::new(
        storage_fs::Storage { path: "~/Documents/storages/keys" },
        storage_fs::Storage { path: "~/Documents/storages/data" },
        storage_fs::Storage { path: "~/Documents/storages/maps" },
    );

    let external_id = args.external_id;
    match args.data_string {
        Some(data) => match vault.dump(&external_id, data.into_bytes()) {
            Ok(_) => println!("Object is successfully stored!"),
            Err(error) => println!("An error has occurred: {}", error)
        },
        None =>
            match args.delete {
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
