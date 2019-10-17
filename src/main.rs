#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};
use rocket_contrib::json::{Json, JsonValue};

use rocket_contrib::databases::diesel;
#[database("app_db")]
struct DbConnection(diesel::PgConnection);

#[get("/")]
fn index(db: DbConnection) -> &'static str {
    "Hello, world!"
}

#[post("/", format = "application/json", data = "<record>")]
fn insert_record(db: DbConnection, record: Json<JsonValue>) {
    // TODO: insert something I guess
//    json! { "name": "wow" }
}

fn db_conf() -> Config {
    let app_env = std::env::var("APP_ENV").unwrap_or("dev".to_string());
    let db_user = std::env::var("DATABASE_USER").unwrap_or("app".to_string());
    let db_password = std::env::var("DATABASE_PASSWORD").unwrap_or("P@assw0rd".to_string());
    let db_name = std::env::var("DATABASE_NAME").unwrap_or("app".to_string());
    let host = if app_env == "dev".to_string() { "localhost" } else { "postgres" };

    let connection_str = format!("postgres://{}:{}@{}/{}", db_user, db_password, host, db_name);

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("DATABASE_URL", Value::from(connection_str));
    databases.insert("app_db", Value::from(database_config));

    Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

fn main() {
    dotenv::dotenv().ok();
    rocket::custom(db_conf())
        .attach(DbConnection::fairing())
        .mount("/", routes![index, insert_record])
        .launch();
}