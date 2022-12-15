use rbatis::{crud, impl_select};
use crate::domain::table::bank_card_issue::BankCardIssue;

crud!(BankCardIssue{});

impl_select!(BankCardIssue{
    select_by_card_no(card_no:&str) -> Option =>
    "`where card_bin = left(#{card_no}, card_bin_length) order by card_bin_length desc limit 1`"
});