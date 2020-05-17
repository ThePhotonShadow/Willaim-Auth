use std::env;
use sqlx::mysql::MySqlPool;
use warp::Filter;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

//async-std = {version = "1.0.0", features = ["attributes"]}
//Above for async without tokio

// fn checkCredentials (username: &str, password: &str) {
//
// }

async fn connect_db<S: AsRef<str>>(db: S) -> Result<MySqlPool, sqlx::Error> {
    Ok(MySqlPool::builder()
        .max_size(5) // maximum number of connections in the pool
        .build(db.as_ref()).await?)
}

#[tokio::main]
async fn main () -> (){
    let dbpool = connect_db("mysql://acore:acore@localhost");
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
