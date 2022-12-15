use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    pub card_no: String,
}