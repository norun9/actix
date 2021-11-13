use crate::pkg;
use crate::schema::posts;
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};

/// This represents a post pulled from the database, including the auto-generated fields
#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

/// This represents a post being inserted into the database, without the auto-generated fields
#[derive(Deserialize, Insertable)]
#[table_name = "posts"]
pub struct InsertPost {
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    title: &'a str,
    body: &'a str,
}

impl Post {
    pub fn index() -> Result<Vec<Self>, pkg::InternalError> {
        let cnn = pkg::db_connection();
        let posts = posts::dsl::posts
            .load::<Post>(&cnn)
            .expect("Error loading posts");
        Ok(posts)
    }
}
