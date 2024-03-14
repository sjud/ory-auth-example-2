use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod kratos_html;
use kratos_html::kratos_html;
pub mod registration;
pub use registration::RegistrationPage;

use serde::{Serialize,Deserialize};