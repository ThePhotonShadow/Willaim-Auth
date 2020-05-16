use std::env;
use sqlx::Connect;
use warp::Filter;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;



//fn checkCredentials (username: &str, password: &str) {
//
//}


#[tokio::main]
async fn main () {
    let dbUname: &str = "acore";
    let dbPwd: &str = "acore";
    let dbpool = MySqlPool::new(format!("{}{}{}{}{}",
        "mysql://", dbUname, ":", dbPwd, "@localhost")).await?;

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
