use super::error_template::ErrorTemplate;
use leptos::*;
use leptos_router::*;
pub mod kratos_html;
use kratos_html::kratos_html;
pub mod registration;
pub use registration::RegistrationPage;
pub mod verification;
use serde::{Deserialize, Serialize};
pub use verification::VerificationPage;
pub mod login;
pub use login::LoginPage;
pub mod session;
pub use session::HasSession;
#[cfg(feature = "ssr")]
pub mod extractors;
pub mod kratos_error;
pub use kratos_error::KratosErrorPage;
pub mod logout;
pub use logout::LogoutButton;
#[cfg(feature = "ssr")]
pub mod keto_utils;
