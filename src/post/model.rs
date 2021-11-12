use crate::pkg::db;
use crate::schema::posts;

/// This represents a post pulled from the database, including the auto-generated fields
#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
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
    pub fn index() -> Result<Vec<Self>, InternalError> {
        let conn = db::cnn()?;

        let posts = posts::dsl::posts
            .load::<Post>(&cnn)
            .expect("Error loading posts");
        Ok(posts)
    }
}
