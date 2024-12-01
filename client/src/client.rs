use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::input::{Account, TransRecord};

const SERVER_BASE_URL: &str = "http://localhost:8080";

#[derive(Serialize, Deserialize)]
pub struct ServerAccount {
    pub account_id: i64,
    pub user_id: i64,
    pub account_name: String,
    pub account_type: String,
    pub account_limit: f64,
}

impl ServerAccount {
    fn to_account(&self) -> Account {
        Account::new(
            &format!("{}", self.account_id),
            &self.account_name,
            &self.account_type,
            &format!("{}", self.user_id),
            self.account_limit,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerTransaction {
    pub transaction_id: i64,
    pub transaction_date: String,
    pub transaction_type: String,
    pub category: String,
    pub transaction_memo: String,
    pub amount: f64,
}

impl ServerTransaction {
    fn to_transaction(&self) -> TransRecord {
        TransRecord::new(
            // &format!("{}", self.account_id),
            // &self.account_name,
            // &self.account_type,
            // &format!("{}", self.user_id),
            // self.account_limit,
            &format!("{}", self.transaction_id),
            // &self.transaction_id,
            &self.transaction_date,
            &self.transaction_type,
            &self.category,
            &self.transaction_memo,
            self.amount,
            // pub timestamp: String,
            // pub trans_type: String, // expense or income
            // pub category: String,
            // pub description: String,
            // pub amount: f64,
        )
    }
}

// Example usage:
// let accounts = crate::client::query_or_create_user("sophie").await;
// for account in accounts.iter() {
//     println!("Got account_id {} account_name {} user_id {}", account.acct_id, account.acct_name, account.user_id);
// }
pub async fn query_or_create_user(username: &str) -> Vec<Account> {
    // TODO: need to convert card type number into text
    let url = format!("{SERVER_BASE_URL}/query_or_create_user");
    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(format!("username={username}"))
        .send()
        .await
        .unwrap();
    if !resp.status().is_success() {
        // Just panic for now
        panic!("Error: Reqwest failed");
    }

    let body = resp.text().await.unwrap();
    let accounts: Vec<ServerAccount> = serde_json::from_str(&body).unwrap();
    let accounts = accounts
        .iter()
        .map(|a| a.to_account())
        .collect::<Vec<Account>>();

    accounts
}

// Example usage:
// let acct_id_str = crate::client::create_or_update_account(Some("2".to_string()), "sophie", "account2", "Credit", 2000.0).await;
pub async fn create_or_update_account(acct_id: Option<String>,
                                      username: &str,
                                      acct_name: &str,
                                      acct_type: &str,
                                      card_limit: f64) -> String {
    let url = format!("{SERVER_BASE_URL}/create_or_update_account");
    let mut post_body = if let Some(aid) = acct_id {
        format!("account_id={}&", aid)
    } else {
        "".to_string()
    };
    post_body.push_str(&format!("username={}&", username));
    post_body.push_str(&format!("account_name={}&", acct_name));
    post_body.push_str(&format!("account_type={}&", acct_type));
    post_body.push_str(&format!("account_limit={}", card_limit));

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(post_body)
        .send()
        .await
        .unwrap();
    if !resp.status().is_success() {
        // Just panic for now
        panic!("Error: Reqwest failed");
    }

    resp.text().await.unwrap()
}

// Example usage:
// let trans_id_str = crate::client::create_or_update_transaction(Some("1".to_string()), "2024-11-11", "Expenses", "Meal", 13.3, "Sushi Burrito", "1").await;
pub async fn create_or_update_transaction(trans_id: Option<String>,
                                          timestamp: &str,
                                          trans_type: &str,
                                          category: &str,
                                          amt: f64,
                                          descrip: &str,
                                          acct_id: &str) -> String {
    let url = format!("{SERVER_BASE_URL}/create_or_update_transaction");
    let mut post_body = if let Some(tid) = trans_id {
        format!("transaction_id={}&", tid)
    } else {
        "".to_string()
    };
    post_body.push_str(&format!("transaction_date={}&", timestamp));
    post_body.push_str(&format!("transaction_type={}&", trans_type));
    post_body.push_str(&format!("category={}&", category));
    post_body.push_str(&format!("amount={}&", amt));
    post_body.push_str(&format!("transaction_memo={}&", descrip));
    post_body.push_str(&format!("account_id={}", acct_id));

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(post_body)
        .send()
        .await
        .unwrap();
    if !resp.status().is_success() {
        // Just panic for now
        panic!("Error: Reqwest failed");
    }

    resp.text().await.unwrap()
}

pub async fn delete_user(username: &str) -> bool {
    let url = format!("{SERVER_BASE_URL}/delete_user");
    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(format!("username={}", username))
        .send()
        .await;
    match resp {
        Ok(response) => response.status().is_success(),
        Err(e) => {
            eprintln!("Error in delete_user: {}", e);
            false
        }
    }
}

pub async fn delete_account(account_id: i64) -> bool {
    let url = format!("{SERVER_BASE_URL}/delete_account/{account_id}");
    let resp = reqwest::get(&url).await;
    match resp {
        Ok(response) => response.status().is_success(),
        Err(e) => {
            eprintln!("Error in delete_account: {}", e);
            false
        }
    }
}

pub async fn delete_transaction(transaction_id: i64) -> bool {
    let url = format!("{SERVER_BASE_URL}/delete_transaction/{transaction_id}");
    let resp = reqwest::get(&url).await;
    match resp {
        Ok(response) => response.status().is_success(),
        Err(e) => {
            eprintln!("Error in delete_transaction: {}", e);
            false
        }
    }
}

// not working
pub async fn query_account(account_id: i64) -> Vec<TransRecord> {
    let url = format!("{SERVER_BASE_URL}/query_account/{}", account_id);
    let resp = reqwest::get(&url).await;
    let resp = match resp {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error in query_account: {}", e);
            panic!("Failed to query account for account_id={}", account_id);
        }
    };

    if !resp.status().is_success() {
        eprintln!("Error: Server responded with status {} for account_id={}", resp.status(), account_id);
        panic!("Reqwest failed for account_id={} resp={}", account_id, resp.status());
    }

    let body = resp.text().await.unwrap();
    let transactions: Vec<ServerTransaction> = serde_json::from_str(&body).unwrap();
    transactions
        .iter()
        .map(|t| t.to_transaction())
        .collect::<Vec<TransRecord>>()
}
