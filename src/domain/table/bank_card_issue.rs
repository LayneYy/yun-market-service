use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BankCardIssue {
    pub card_bin: String,
    pub bank: String,
    pub card_bin_length: i8,
    pub card_name: String,
    pub card_type: String,
    pub issue_name: String,
    pub issue_no: String,
}