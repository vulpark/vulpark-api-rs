use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::api::{ServerCalls, ServerContext};
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String
}
impl User {
    pub fn new(id: String, ctx: &mut ServerContext) -> Result<Self, Box<dyn Error>> {
        let res = ctx.make_call(ServerCalls::GetUser(id))?;
        Ok(res.json()?)
    }
}