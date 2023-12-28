use std::fmt::Debug;

#[derive(thiserror::Error, Debug)]
pub enum ScError {
    #[error("failed to retrieve from database")]
    DatabaseError(#[from] surrealdb::Error),
}
