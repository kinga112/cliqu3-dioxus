//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

// mod hero;
// pub use hero::Hero;

pub mod dms;
pub mod login;
pub mod server;
pub mod settings;
pub mod sidenav;
pub mod user;

mod modal;
pub use modal::Modal;
