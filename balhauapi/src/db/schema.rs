use super::super::confs::load_app_configuration;
use super::super::confs::AppConfig;

fn get_db_url(conf_app : &AppConfig) -> String  {
    let hostname : & String = &conf_app.configs.database.hostname;
    let password : & String = &conf_app.configs.database.pass;
    let schema :  & String = &conf_app.configs.database.schema;
    let user : & String = &conf_app.configs.database.user;

    String::from("postgres://")+user+":"+password+"@"+hostname+"/"+schema

}


//TODO: This kinds of sucks. As far as I know infer_schema! resolve this names into a set of compile time
// steps which is dependent on two strategies, one this is an hard coded string with the database connection
// second it is a variable presented in the env that is loaded with the dotenv library. All this is expanded
// with macros at compile time and I don't know yet how to expand db_url into the string at the same compile time
// I need to fix this as soon as possible to avoid duplication and scattering of configurations.
// :/
infer_schema!("dotenv:DATABASE_URL");
