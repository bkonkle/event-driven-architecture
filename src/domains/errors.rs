/// CQRS Domain errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Not found error
    #[error("{entity} not found")]
    NotFound {
        /// The entity type tht was not found
        entity: String,
    },

    /// Forbidden error
    #[error("Forbidden")]
    Forbidden,

    /// Unauthorized error
    #[error("Unauthorized")]
    #[allow(dead_code)]
    Unauthorized(#[source] Option<anyhow::Error>),

    /// A uniquness conflict
    #[error("The field `{field}` must be unique")]
    Uniqueness {
        /// The field that failed a uniqueness check
        field: String,
    },
}
