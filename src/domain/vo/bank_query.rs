use serde::Serialize;
use crate::domain::table::bank_card_issue::BankCardIssue;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankQueryRes {
    pub card_bin: String,
    pub bank: String,
    pub card_bin_length: i8,
    pub card_name: String,
    pub card_type: String,
    pub issue_name: String,
    pub issue_no: String,
}

impl From<BankCardIssue> for BankQueryRes {
    fn from(v: BankCardIssue) -> Self {
        Self {
            card_bin: v.card_bin,
            bank: v.bank,
            card_bin_length: v.card_bin_length,
            card_name: v.card_name,
            card_type: v.card_type,
            issue_name: v.issue_name,
            issue_no: v.issue_no,
        }
    }
}

impl From<&BankCardIssue> for BankQueryRes {
    fn from(v: &BankCardIssue) -> Self {
        Self {
            card_bin: v.card_bin.clone(),
            bank: v.bank.clone(),
            card_bin_length: v.card_bin_length,
            card_name: v.card_name.clone(),
            card_type: v.card_type.clone(),
            issue_name: v.issue_name.clone(),
            issue_no: v.issue_no.clone(),
        }
    }
}
