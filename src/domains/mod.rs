/// The Tasks domain
pub mod tasks;

/// The Domain Event type
pub mod event;

/// Domain Errors
pub mod errors;

pub use errors::Error;

#[allow(unused_imports)]
pub use event::DomainEvent;
