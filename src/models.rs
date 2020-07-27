use serde::Serialize;


#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize)]
pub struct Message {
    pub message: String
}

