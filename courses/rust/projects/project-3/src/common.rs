use serde::{Deserialize, Serialize};

// by deriving the Serialize trait, the struct or enum can be converted into
// a format that can be stored or transmitted, such as JSON or XML

// by deriving the deserialize trait, the struct or enum can be reconstructed from
// the serialized format back into the original data structure.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetResponse {
    Ok(Option<String>),
    Err(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SetResponse {
    Ok(()),
    Err(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoveResponse {
    Ok(()),
    Err(String),
}
