use std::error::Error;
use reqwest::blocking::Response;

use crate::api::ServerCalls::*;

pub mod message;
pub mod user;

pub enum ServerCalls {
    GetUser(String)
}
pub struct ServerContext {
    pub url_root: String,
}
impl ServerContext {
    pub fn new(url_root: String) -> Self {
        ServerContext {
            url_root
        }
    }
    pub fn make_call(&mut self, call: ServerCalls) -> Result<Response, Box<dyn Error>> {
        let url = match call {
            GetUser(id) => format!("/users/{id}")
        };
        Ok(reqwest::blocking::get(format!("{}/{url}", self.url_root))?)
    }
}
