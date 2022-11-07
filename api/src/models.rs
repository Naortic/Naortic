use crate::schema::accounts;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Clone, Serialize, Debug)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub token: String,
    pub email: String,
}

#[derive(Insertable, Queryable, Debug)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub password: &'a str,
    pub token: &'a str,
    pub email: &'a str,
}
