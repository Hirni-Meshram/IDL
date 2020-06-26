/* Generated with IDL v0.2.0 */

use crate::serde_derive::*;

// --------------- DECLARATIONS -----------------
#[derive(Serialize, Deserialize)]
pub enum Packet {
    pub request(Request),
    pub response(Response),
}

#[derive(Serialize, Deserialize)]
pub enum LoginError {
    pub UNKNOWN_USER = 0,
    pub WRONG_PASSWORD = 1,
    pub SERVER_DOWN = 2,
}

#[derive(Serialize, Deserialize)]
pub struct PingRequest;

#[derive(Serialize, Deserialize)]
pub struct PongResponse;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub password: [char; 32],
}

#[derive(Serialize, Deserialize)]
pub enum LoginResponse {
    pub session_id(String),
    pub error(LoginError),
}

#[derive(Serialize, Deserialize)]
pub enum Request {
    pub ping(PingRequest),
    pub login(LoginRequest),
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    pub pong(PongResponse),
    pub login(LoginResponse),
}

// --------------- DEFINITIONS ------------------