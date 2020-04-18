///This library contains common command and response types for client and daemon for their communication.
///Depending on which command the client enters into the console, its type will be determined, the command will be serialized and then sent to the daemon.
///The response from the daemon comes in a serialized form and deserialize, it is reduced to the type of response to which the request was sent from the client.
///Next, information is output to the client in the console in accordance with the type of response.

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
