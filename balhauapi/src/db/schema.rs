use super::super::confs::load_app_configuration;
use super::super::confs::AppConfig;

fn get_db_url(conf_app : &AppConfig) -> String  {
    let hostname : & String = &conf_app.configs.database.hostname;
    let password : & String = &conf_app.configs.database.pass;
    let schema :  & String = &conf_app.configs.database.schema;
    let user : & String = &conf_app.configs.database.user;

    String::from("postgres://")+user+":"+password+"@"+hostname+"/"+schema

}

infer_schema!("dotenv:DATABASE_URL");

