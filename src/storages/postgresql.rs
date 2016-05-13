use std::error::Error;

use rustc_serialize::json;
use rustc_serialize::{Decodable, Encodable};

extern crate postgres;

use self::postgres::{Connection, SslMode};

pub struct Storage {
    pub connection_url: &'static str,
    pub table_name: &'static str
}

impl Storage {

    fn ensure_connection(&self) -> Result<Connection, Box<Error>> {
        let connection = Connection::connect(self.connection_url, SslMode::None).unwrap();
        connection.execute("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\" WITH SCHEMA public", &[]).unwrap();
        connection.execute(&format!("CREATE TABLE IF NOT EXISTS {} (
            id uuid DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
            data jsonb NOT NULL)", &self.table_name), &[]).unwrap();
        Ok(connection)
    }

    pub fn dump<T: Encodable>(&self, id: &String, storable: T) -> Result<(), Box<Error>> {
        self.ensure_connection().ok().unwrap().execute(&format!("INSERT INTO {} (id, data) VALUES ($1, $2)", &self.table_name), &[id, &json::encode(&storable).unwrap()]).unwrap();
        Ok(())
    }

    pub fn delete(&self, id: &String) -> Result<Option<()>, Box<Error>> {
        self.ensure_connection().ok().unwrap().execute(&format!("DELETE FROM {} WHERE id = $1", &self.table_name), &[id]).unwrap();
        Ok(Some(()))
    }

    pub fn load<T: Decodable>(&self, id: &String) -> Result<Option<T>, Box<Error>> {
        let connection = self.ensure_connection().ok().unwrap();
        for row in &connection.query(&format!("SELECT id, data FROM {} WHERE id = $1", &self.table_name), &[id]).unwrap() {
            let data: String = row.get(1);
            let json = try!(json::decode(&data));
            return Ok(Some(json))
        }
        Ok(None)
    }
}
