use super::super::schema::ereaderitem;

#[derive(Queryable,Debug,Serialize,Deserialize)]
pub struct Author{
    pub id_author : i32,
    pub f_id_author : i32,
    pub f_name : String,
    pub f_country : String
}

#[derive(Queryable,Debug,Serialize,Deserialize)]
pub struct Item{
    pub id_item: i32,
    pub f_id_item: Option<i32>,
    pub f_pages_number: Option<i32>,
    pub f_current_page: Option<i32>,
    pub f_last_read: Option<i32>,
    pub f_publication_date: Option<String>,
    pub f_publisher :Option<String>,
    pub f_title : Option<String>,
    pub f_description: Option<String>,
    pub f_author_id: Option<i32>
}

#[derive(Insertable)]
#[table_name = "ereaderitem"]
pub struct NewItem<'a> {
    pub f_id_item: i32,
    pub f_pages_number: i32,
    pub f_current_page: i32,
    pub f_last_read: i32,
    pub f_publication_date: &'a str,
    pub f_publisher :&'a str,
    pub f_title : &'a str,
    pub f_description: &'a str,
    pub f_author_id: i32
}