use rawsql::Loader;
use std::env::var;
use tokio_postgres::{connect, Client, Error, Row};

use crate::util::snowflake;

fn query(name: &str) -> String {
    Loader::read_queries_from("src/postgre.sql")
        .unwrap()
        .get(name)
        .unwrap()
        .clone()
}

async fn connection() -> Client {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs") {
        roots.add(&rustls::Certificate(cert.0)).unwrap();
    }

    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let tls = tokio_postgres_rustls::MakeRustlsConnect::new(config);

    let url = var("DATABASE_URL").expect("DATABASE_URL not found");
    let (client, connection) = connect(&url, tls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            println!("connection error: {}", e);
        }
    });

    client
}

pub async fn create_table() {
    connection()
        .await
        .execute(&query("create-table"), &[])
        .await
        .unwrap();
}

pub async fn create_account(name: &str, password: &str, email: &str, token: &str) -> Vec<Row> {
    connection()
        .await
        .query(
            &query("create-account"),
            &[&snowflake(), &name, &password, &token, &email],
        )
        .await
        .unwrap()
}

pub async fn read_account(token: &str) -> Vec<Row> {
    connection()
        .await
        .query(&query("read-account"), &[&token])
        .await
        .unwrap()
}

pub async fn find_account(id: &str) -> Result<Vec<Row>, Error> {
    connection()
        .await
        .query(&query("find-account"), &[&id])
        .await
}

pub async fn search_account(email: &str, password: &str) -> Result<Vec<Row>, Error> {
    connection()
        .await
        .query(&query("search-account"), &[&email, &password])
        .await
}

pub async fn update_account_name(token: &str, name: &str) -> Vec<Row> {
    connection()
        .await
        .query(&query("update-account-name"), &[&name, &token])
        .await
        .unwrap()
}

pub async fn update_account_password(token: &str, password: &str) -> Vec<Row> {
    connection()
        .await
        .query(&query("update-account-password"), &[&password, &token])
        .await
        .unwrap()
}

pub async fn update_account_email(token: &str, email: &str) -> Vec<Row> {
    connection()
        .await
        .query(&query("update-account-email"), &[&email, &token])
        .await
        .unwrap()
}

pub async fn update_account_token(token: &str, new_token: &str) -> Vec<Row> {
    connection()
        .await
        .query(&query("update-account-token"), &[&new_token, &token])
        .await
        .unwrap()
}

pub async fn update_account_friend_add(token: &str, friend: &str) -> Vec<Row> {
    connection()
        .await
        .query(&query("update-account-friend-add"), &[&friend, &token])
        .await
        .unwrap()
}

pub async fn update_account_friend_remove(token: &str, friend: &str) -> Vec<Row> {
    connection()
        .await
        .query(&query("update-account-friend-remove"), &[&friend, &token])
        .await
        .unwrap()
}

pub async fn delete_account(token: &str) {
    connection()
        .await
        .execute(&query("delete-account"), &[&token])
        .await
        .unwrap();
}
