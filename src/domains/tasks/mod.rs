/// The Tasks Aggregate
pub mod aggregate;

/// Task Commands
pub mod commands;

/// Task Events
pub mod events;

/// Task input types
pub mod inputs;

/// The default Task View
pub mod view;

/// The default Task CqrsFramework
pub mod cqrs;

pub use aggregate::{Services, Task, AGGREGATE_TYPE};
pub use commands::Command;
pub use events::Event;
pub use view::{Query, View};
