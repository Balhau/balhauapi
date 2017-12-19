use std::time::SystemTime;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub post_created: SystemTime,
    pub post_updated: SystemTime,
    pub published: bool,
}

use super::super::schema::blog_posts;

#[derive(Insertable)]
#[table_name = "blog_posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub post_created: &'a SystemTime,
    pub post_updated: &'a SystemTime,
}
