use rustc_serialize::json;
use rustc_serialize::{Decodable, Encodable};

extern crate postgres;

use self::postgres::{Connection, SslMode};

use uuid::Uuid;

use super::StorageResult;
use super::StorageResultOption;

pub struct Storage {
    pub connection_url: &'static str,
    pub table_name: &'static str
}

impl Storage {

    fn ensure_connection(&self) -> StorageResult<Connection> {
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

    fn dump<T: Encodable>(&self, id: &String, storable: T) -> StorageResult<()> {
        let json: json::Json = try!(json::Json::from_str(&try!(json::encode(&storable))));
        try!(self.ensure_connection().ok().unwrap().execute(&format!("INSERT INTO {} (id, data) VALUES ($1, $2)", &self.table_name), &[&try!(Uuid::parse_str(id)), &json]));
        Ok(())
    }

    fn delete(&self, id: &String) -> StorageResultOption<()> {
        try!(self.ensure_connection().ok().unwrap().execute(&format!("DELETE FROM {} WHERE id = $1", &self.table_name), &[&try!(Uuid::parse_str(id))]));
        Ok(Some(()))
    }

    fn load<T: Decodable>(&self, id: &String) -> StorageResultOption<T> {
        let connection = self.ensure_connection().ok().unwrap();
        for row in &try!(connection.query(&format!("SELECT id, data FROM {} WHERE id = $1", &self.table_name), &[&try!(Uuid::parse_str(id))])) {
            let json: json::Json = row.get(1);
            let data: T = try!(json::decode(&json.to_string()));
            return Ok(Some(data))
        }
        Ok(None)
    }

}

impl super::KeysStorage for Storage {}
impl super::DataStorage for Storage {}
impl super::MapsStorage for Storage {}