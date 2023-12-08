pub type BaseResult<T> = Result<T, BaseError>;

#[derive(thiserror::Error, Debug)]
pub enum BaseError {
    #[error("could not set config")]
    ConfigSetError,

    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    ClientError(#[from] crate::api::error::ClientError),

    #[error(transparent)]
    SyncError(#[from] crate::sync::error::SyncError),

    #[error(transparent)]
    DbError(#[from] crate::db::error::DbError),

    #[error(transparent)]
    WebError(#[from] crate::web::error::WebError),
}
