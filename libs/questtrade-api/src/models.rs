use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    api_server : String,
    access_token: String,
    expires_in: i32,
    refresh_token: String,
}