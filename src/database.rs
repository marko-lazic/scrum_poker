use deadpool::async_trait;
use deadpool::managed;
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
        // TODO: Replace unrwarp with retry task
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
