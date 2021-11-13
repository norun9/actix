use crate::pkg;
use crate::schema::posts as posts_schema;
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
#[table_name = "posts_schema"]
pub struct NewPost {
    title: String,
    body: String,
}

impl Post {
    pub fn index() -> Result<Vec<Self>, pkg::InternalError> {
        let cnn = pkg::db_connection();
        let posts = posts_schema::dsl::posts
            .load::<Post>(&cnn)
            .expect("Error loading posts");
        Ok(posts)
    }

    pub fn find(id: i32) -> Result<Self, pkg::InternalError> {
        let cnn = pkg::db_connection();
        let post = posts_schema::dsl::posts
            .filter(posts_schema::id.eq(id))
            .first::<Post>(&cnn)
            .expect("Error loading posts");
        Ok(post)
    }

    pub fn create(new_post: NewPost) -> Result<Self, pkg::InternalError> {
        let cnn = pkg::db_connection();
        let _ = diesel::insert_into(posts_schema::dsl::posts)
            .values(&new_post)
            .execute(&cnn)
            .expect("Error saving posts");

        let new_post = posts_schema::dsl::posts
            .order(posts_schema::id.desc())
            .first::<Post>(&cnn)
            .expect("Error loading posts");
        Ok(new_post)
    }

    pub fn update(id: i32, title: String, body: String) -> Result<Self, pkg::InternalError> {
        let cnn = pkg::db_connection();
        let _ = diesel::update(posts_schema::dsl::posts.find(id))
            .set((posts_schema::title.eq(title), posts_schema::body.eq(body)))
            .execute(&cnn)
            .expect("Error updating users");
        Ok(Post::find(id).unwrap())
    }
}
