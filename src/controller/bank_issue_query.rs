use axum::extract::Query;
use crate::context::CTX;
use crate::domain::{
    dto::bank_card_issue::Params,
    vo::{
        bank_query::BankQueryRes,
        response::ApiResponse,
    },
};

pub async fn bank_issue_query(Query(Params { card_no }): Query<Params>) -> ApiResponse<BankQueryRes> {
    CTX.bank_issue_srv.query(card_no).await.into()
}