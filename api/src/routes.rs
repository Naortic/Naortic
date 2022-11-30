#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

pub mod login {
    use crate::database::search_account;
    use crate::util::Error;
    use anyhow::anyhow;

    #[get("/?<email>&<password>")]
    pub async fn index(email: &str, password: &str) -> Result<String, Error> {
        let account = search_account(email, password).await;

        if account.is_err() {
            Err(Error(anyhow!("account not found")))
        } else {
            let account = account.unwrap();
            Ok(account[0].get(0))
        }
    }
}

pub mod signup {
    use base64ct::{Base64, Encoding};
    use sha2::{Digest, Sha256};

    use crate::database::create_account;

    #[get("/?<email>&<name>&<password>")]
    pub async fn index(email: &str, name: &str, password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(name);
        hasher.update(password);

        let hash = hasher.finalize();
        let token = &Base64::encode_string(&hash);

        let account = create_account(name, password, email, token).await;

        account.get(0).unwrap().get(0)
    }
}

pub mod users {
    use crate::util::Error;
    use crate::{account::Account, database::find_account};
    use anyhow::anyhow;
    use rocket::serde::json::Json;

    #[get("/<id>")]
    pub async fn index(id: &str) -> Result<Json<Account>, Error> {
        let account = find_account(id).await;

        if account.is_err() {
            return Err(Error(anyhow!("account not found")));
        }

        let account = account.unwrap();

        let row = &account[0];

        let account = Account {
            snowflake: row.get(0),
            name: row.get(1),
            friends: row.get(2),
            avatar: row.get(3),
            email: None,
            password: None,
            token: None,
        };

        Ok(Json(account))
    }
}

pub mod me {
    use crate::{account::Account, database::read_account};
    use rocket::serde::json::Json;

    #[get("/?<token>")]
    pub async fn index(token: &str) -> Json<Account> {
        let row = &read_account(token).await[0];

        let account = Account {
            snowflake: row.get(0),
            name: row.get(1),
            email: row.get(2),
            friends: row.get(3),
            token: Some(token.to_string()),
            password: None,
            avatar: None,
        };

        Json(account)
    }

    pub mod friends {

        use crate::database::{
            read_account, update_account_friend_add, update_account_friend_remove,
        };
        use rocket::serde::json::Json;

        #[get("/friends?<token>")]
        pub async fn index(token: &str) -> Json<Vec<String>> {
            let friends = read_account(token).await[0].get(0);

            Json(friends)
        }

        #[get("/friends/add?<token>&<friend>")]
        pub async fn add(token: &str, friend: &str) -> String {
            update_account_friend_add(token, friend).await;

            friend.to_string()
        }

        #[get("/friends/remove?<token>&<friend>")]
        pub async fn remove(token: &str, friend: &str) -> String {
            update_account_friend_remove(token, friend).await;

            friend.to_string()
        }
    }
}
