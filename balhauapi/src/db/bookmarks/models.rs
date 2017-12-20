use std::time::SystemTime;
use super::super::schema::bookmarks;

#[derive(Queryable,Debug,Serialize,Deserialize)]
pub struct Bookmark {
    pub id: i32,
    pub title: String,
    pub b64icon: Option<String>,
    pub created: SystemTime,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Bookmarks {
    pub bookmarks : Vec<Bookmark>
}


#[derive(Insertable)]
#[table_name = "bookmarks"]
pub struct NewBookmark<'a> {
    pub title: &'a str,
    pub b64icon: &'a str,
    pub created: &'a SystemTime,
    pub url: &'a str,
}
