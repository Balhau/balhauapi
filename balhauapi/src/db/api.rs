use super::blog::models::*;
use super::bookmarks::models::*;
use super::schema::*;

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

// Save bookmark into database
pub fn save_bookmark<'a>(
    conn: &PgConnection,
    title: &'a str,
    b64icon: &'a str,
    url: &'a str,
    created: &SystemTime,
) {
    use db::schema::bookmarks;

    let new_bookmark = NewBookmark{
        title: title,
        b64icon: b64icon,
        url: url,
        created: created
    };

    diesel::insert_into(bookmarks::table)
        .values(&new_bookmark)
        .get_result(conn)
        .expect("Error saving bookmark")
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use db::schema::blog_posts;

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

