use std::error::Error;

use rustc_serialize::json;
use rustc_serialize::{Decodable, Encodable};

extern crate postgres;

use self::postgres::{Connection, SslMode};

use uuid::Uuid;

pub struct Storage {
    pub connection_url: &'static str,
    pub table_name: &'static str
}

impl super::MapsStorage for Storage {}
impl super::KeysStorage for Storage {}
impl super::DataStorage for Storage {}

impl Storage {

    fn ensure_connection(&self) -> Result<Connection, Box<Error>> {
        let connection = try!(Connection::connect(self.connection_url, SslMode::None));
        try!(connection.execute("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\" WITH SCHEMA public", &[]));
        try!(connection.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
               id uuid DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
               data jsonb NOT NULL
            )", &self.table_name), &[]));
        Ok(connection)
    }

}

impl super::BaseStorage for Storage {

    fn dump<T: Encodable>(&self, id: &String, storable: T) -> Result<(), Box<Error>> {
        let json: json::Json = try!(json::Json::from_str(&try!(json::encode(&storable))));
        try!(self.ensure_connection().ok().unwrap().execute(&format!("INSERT INTO {} (id, data) VALUES ($1, $2)", &self.table_name), &[&try!(Uuid::parse_str(id)), &json]));
        Ok(())
    }

    fn delete(&self, id: &String) -> Result<Option<()>, Box<Error>> {
        try!(self.ensure_connection().ok().unwrap().execute(&format!("DELETE FROM {} WHERE id = $1", &self.table_name), &[&try!(Uuid::parse_str(id))]));
        Ok(Some(()))
    }

    fn load<T: Decodable>(&self, id: &String) -> Result<Option<T>, Box<Error>> {
        let connection = self.ensure_connection().ok().unwrap();
        for row in &try!(connection.query(&format!("SELECT id, data FROM {} WHERE id = $1", &self.table_name), &[&try!(Uuid::parse_str(id))])) {
            let json: json::Json = row.get(1);
            let data: T = try!(json::decode(&json.to_string()));
            return Ok(Some(data))
        }
        Ok(None)
    }

}