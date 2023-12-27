// use surrealdb::engine::any::Any;
// use surrealdb::opt::auth::Root;
// use surrealdb::Surreal;

// pub static DB: AtomRef<Surreal<Any>> = AtomRef(|_| db_block());

// pub type Db = Surreal<Any>;

// fn db_block() -> Surreal<Any> {
//     tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(db())
// }

// pub async fn db() -> Surreal<Any> {
//     let db = connect("ws://localhost:8000").await.unwrap();

//     db.signin(Root {
//         username: "root",
//         password: "root",
//     })
//     .await
//     .unwrap();
//     db.use_ns("test").use_db("test").await.unwrap();

//     db
// }
