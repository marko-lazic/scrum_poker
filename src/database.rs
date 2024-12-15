use std::env;

use deadpool::managed;
use surrealdb::engine::any;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::Error;
use surrealdb::Surreal;

pub type Pool = managed::Pool<Manager>;

pub struct Manager {}

impl managed::Manager for Manager {
    type Type = Surreal<Any>;
    type Error = Error;

    async fn create(&self) -> Result<Surreal<Any>, Error> {
        let address = env::var("DB_ADDRESS").unwrap_or("ws://localhost:8000".into());
        tracing::info!("Connecting to database address {address}");
        let db = any::connect(address).await.unwrap();

        let username = env::var("DB_USERNAME").unwrap_or("root".into());
        let password = env::var("DB_PASSWORD").unwrap_or("root".into());
        let ns = env::var("DB_NS").unwrap_or("scrumpokerdb".into());
        let db_name = env::var("DB_NAME").unwrap_or("scrumpokerdb".into());
        db.signin(Root {
            username: username.as_str(),
            password: password.as_str(),
        })
        .await
        .unwrap();
        db.use_ns(ns).use_db(db_name).await.unwrap();
        Ok(db)
    }

    async fn recycle(
        &self,
        _: &mut Surreal<Any>,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        Ok(())
    }
}
