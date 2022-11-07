use diesel::result::Error;
use crate::models::*;
use base64ct::{Base64, Encoding};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use sha2::{Digest, Sha256};

pub fn connect() -> MysqlConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create(conn: &mut MysqlConnection, name: &str, password: &str, email: &str) -> Account {
    use crate::schema::accounts;

    let mut hasher = Sha256::new();
    hasher.update(name);
    hasher.update(password);
    
    let hash = hasher.finalize();
    let token = &Base64::encode_string(&hash);

    let new_account = NewAccount {
        email,
        name,
        password,
        token,
    };

    diesel::insert_into(accounts::table)
        .values(&new_account)
        .execute(conn)
        .expect("Error saving new post");

    accounts::table
        .order(accounts::id.desc())
        .first(conn)
        .unwrap()
}

pub fn update(target: String, target_password: String) {
    use crate::schema::accounts::dsl::{accounts, name, password};

    let connection = &mut connect();
    let pattern = format!("%{}%", target);

    diesel::update(accounts.filter(name.like(pattern)))
        .set(password.eq(target_password))
        .execute(connection)
        .unwrap();
}

pub fn get(usr: &str) -> Account {
    use crate::schema::accounts::dsl::*;

    let connection = &mut connect();

    accounts
        .filter(token.eq(usr))
        .limit(1)
        .load::<Account>(connection)
        .expect("Error loading account")[0]
        .clone()
}

pub fn find(usr_email: &str, usr_password: &str) -> (bool, Result<Vec<Account>, Error>) {
    use crate::schema::accounts::dsl::*;

    let connection = &mut connect();

    let account = accounts
        .filter(email.eq(usr_email))
        .filter(password.eq(usr_password))
        .limit(1)
        .load::<Account>(connection);

    (account.is_ok(), account)
}

pub fn delete(target: String) {
    use crate::schema::accounts::dsl::*;

    let pattern = format!("%{}%", target);

    let connection = &mut connect();
    let num_deleted = diesel::delete(accounts.filter(name.like(pattern)))
        .execute(connection)
        .expect("Error deleting accounts");

    println!("Deleted {} accounts", num_deleted);
}
