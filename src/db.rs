use dioxus::prelude::*;
use surrealdb::engine::any::{connect, Any};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub type Db = Surreal<Any>;

pub async fn init_db_connection() -> Db {
    println!("Init db connection");
    let db = connect("ws://localhost:8000").await.unwrap();

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    db
}

pub fn use_db(cx: &ScopeState) -> &UseSharedState<Db> {
    use_shared_state::<Db>(cx).expect("Db not provided")
}
