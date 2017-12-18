use super::blog::models::*;
use super::blog::schema::*;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::time::SystemTime;
use diesel;

const DATABASE_URL: &str = "DATABASE_URL";

pub fn create_conn() -> PgConnection {
    dotenv().ok();

    let database_url = env::var(DATABASE_URL)
        .expect("Database URL must be defined");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use db::blog::schema::blog_posts;

    let new_post = NewPost {
        title: title,
        body: body,
        post_created: &SystemTime::now(),
        post_updated: &SystemTime::now(),
    };

    diesel::insert_into(blog_posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

