use std::time::SystemTime;
use super::super::schema::ereader_t_author;

#[derive(Queryable,Debug,Serialize,Deserialize)]
pub struct Author{
    pub id_author : i32,
    pub f_id_author : i32,
    pub f_name : String,
    pub f_country : String
}