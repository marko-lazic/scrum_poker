use std::sync::Arc;

use deadpool::async_trait;
use deadpool::managed;
use dioxus::core::ScopeState;
use dioxus::hooks::use_shared_state;
use dioxus::hooks::UseSharedState;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Error;
use surrealdb::Surreal;

pub type Pool = managed::Pool<Manager>;

pub struct Manager {}

#[async_trait]
impl managed::Manager for Manager {
    type Type = Surreal<Client>;
    type Error = Error;

    async fn create(&self) -> Result<Surreal<Client>, Error> {
        let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        Ok(db)
    }

    async fn recycle(
        &self,
        _: &mut Surreal<Client>,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        Ok(())
    }
}

pub fn use_pool(cx: &ScopeState) -> &UseSharedState<Arc<Pool>> {
    use_shared_state::<Arc<Pool>>(cx).expect("Pool not provided")
}
