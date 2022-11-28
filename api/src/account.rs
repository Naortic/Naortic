use serde::Serialize;

/// Public data that can be returned safely without authorization
#[derive(Clone, Serialize, Debug)]
pub struct Account {
    pub name: String,
    pub email: String,
    pub friends: Vec<String>,
    pub token: String,
}