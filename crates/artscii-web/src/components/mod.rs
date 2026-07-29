//! UI components, one per file, composed by [`crate::app::App`].

mod controls;
mod footer;
mod header;
mod hero;
mod preview;
mod sidebar;
mod toast;
mod workspace;

pub use footer::Footer;
pub use header::Header;
pub use hero::Hero;
pub use toast::Toast;
pub use workspace::Workspace;
