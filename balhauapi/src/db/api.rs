use super::blog::models::*;
use super::bookmarks::models::*;
use super::ereader::models::*;



use diesel::prelude::*;
use dotenv::dotenv;
use diesel::pg::PgConnection;
use std::env;
use std::time::SystemTime;
use diesel;

const DATABASE_URL: &str = "DATABASE_URL";
const LIMIT_ROWS : i64 = 100;


pub fn create_conn() -> PgConnection {
    dotenv().ok();

    let database_url = env::var(DATABASE_URL)
        .expect("Database URL must be defined");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_all_bookmarks(conn : &PgConnection) -> Bookmarks {
    use db::schema::bookmarks;
    Bookmarks {
        bookmarks : bookmarks::table.load(&*conn).unwrap()
    }
}

pub fn get_bookmarks_paged(conn : &PgConnection, start_id : i64, max : i64 ) -> Bookmarks {
    use db::schema::bookmarks;

    let max_rows = if max > LIMIT_ROWS {LIMIT_ROWS} else {max};

    Bookmarks{
        bookmarks : bookmarks::table
            .limit(max_rows)
            .offset(start_id)
            .load(conn).unwrap()
    }
}

pub fn truncate_ereaderitem_table(conn : &PgConnection) {
    use db::schema::ereaderitem;
    let res = diesel::delete(ereaderitem::table).execute(conn);
}

pub fn save_ereader_item<'a>(conn : &PgConnection, item: &'a Item) -> Item {

    use db::schema::ereaderitem;

    let pubdate = item.f_publication_date.clone().unwrap_or(String::from(""));
    let publisher = item.f_publisher.clone().unwrap_or(String::from(""));
    let title = item.f_title.clone().unwrap_or(String::from(""));
    let description = item.f_description.clone().unwrap_or(String::from(""));

    let new_item = NewItem{
        f_id_item: item.f_author_id.clone().unwrap_or(0),
        f_pages_number: item.f_pages_number.unwrap_or(0),
        f_current_page: item.f_current_page.unwrap_or(0),
        f_last_read: item.f_last_read.unwrap_or(0),
        f_publication_date: &pubdate,
        f_publisher: &publisher,
        f_title: &title,
        f_description: &description,
        f_author_id: item.f_author_id.unwrap_or(0)
    };

    diesel::insert_into(ereaderitem::table)
        .values(&new_item)
        .get_result(conn)
        .expect("Error saving bookmark")
}

// Save bookmark into database
pub fn save_bookmark<'a>(
    conn: &PgConnection,
    title: &'a str,
    b64icon: &'a str,
    url: &'a str,
    created: &SystemTime,
) -> Bookmark {
    use db::schema::bookmarks;

    let new_bookmark = NewBookmark {
        title,
        b64icon,
        url,
        created
    };

    diesel::insert_into(bookmarks::table)
        .values(&new_bookmark)
        .get_result(conn)
        .expect("Error saving bookmark")
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use db::schema::blog_posts;

    let new_post = NewPost {
        title,
        body,
        post_created: &SystemTime::now(),
        post_updated: &SystemTime::now(),
    };

    diesel::insert_into(blog_posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}