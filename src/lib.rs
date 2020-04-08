/// Crate for containing common commands for client and daemon
/// Example:
/// let command = CommandType::Status;
use serde::{Serialize, Deserialize};
use std::net::IpAddr;
use std::collections::HashMap;

pub const PORT: u16 = 3000; // now it connects to 3000
pub type LsResponseType = HashMap<IpAddr, Vec<String>>;
pub type StatusResponseType = (HashMap<String, Vec<IpAddr>>, HashMap<String, Vec<IpAddr>>);

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandType {
    Share(String),
    Scan,
    Ls,
    Download(String, String),
    Status,
}

/* Client gets enum from a daemon what is below.
   Any request from a client could end with access or failure.
   In success a client gets enum with 1 or 2 or 3 field. Depends from what client was requested.
   If error - enum with 4th field.*/

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseType {
    ShareScan,
    Ls(String),
    Download(bool),
    Status(String),
    Error(String),
}
