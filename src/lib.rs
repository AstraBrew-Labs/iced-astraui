//! Astra UI is a reusable component library and design system for
//! [iced](https://iced.rs/) desktop applications.
//!
//! Components can be imported from the crate root or through the [`ui`]
//! compatibility facade:
//!
//! ```no_run
//! use astra_ui::{Alert, AlertKind, Card};
//! ```

pub mod components;
pub mod fonts;
pub mod icons;
pub mod ui;

pub use components::*;

/// Common Astra UI imports for application view modules.
pub mod prelude {
    pub use crate::components::*;
    pub use crate::{fonts, icons};
}
