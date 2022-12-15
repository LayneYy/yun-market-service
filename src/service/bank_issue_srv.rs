use super::market::MarketSrv;
use anyhow::anyhow;
use tokio::try_join;
use tracing::info;
use crate::domain::table::bank_card_issue::BankCardIssue;
use crate::domain::vo::bank_query::BankQueryRes;
use crate::pool;

pub struct BankIssueSrv;

impl BankIssueSrv {
    pub async fn query(&self, card_no: String) -> anyhow::Result<BankQueryRes> {
        // 1.查询本地数据库,若有匹配记录直接返回
        let issue = {
            BankCardIssue::select_by_card_no(pool!(), card_no.as_str()).await?
        };

        if let Some(bci) = issue {
            info!("{} return from local", card_no);
            Ok(bci.into())
        } else {
            //2.云市场查询,结果落库,返回结果
            info!("{} return from market", card_no);
            let issue = {
                let l = tokio::spawn(MarketSrv.lianzhuo(card_no.clone()));
                let s = tokio::spawn(MarketSrv.shumai(card_no));
                let (l, s) = try_join!(l ,s)?;
                let err = || anyhow!("market no result");
                let (l, s) = (l?.data.ok_or_else(err)?, s?.data.ok_or_else(err)?);
                BankCardIssue {
                    card_bin: s.card_bin,
                    bank: s.abbreviation,
                    card_bin_length: s.bin_digits,
                    card_name: s.card_name,
                    card_type: s.card_type,
                    issue_name: l.bank_name,
                    issue_no: l.bankno,
                }
            };

            BankCardIssue::insert(pool!(), &issue).await?;

            Ok(issue.into())
        }
    }
}
