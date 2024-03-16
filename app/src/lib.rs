#![feature(box_patterns)]

use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;
pub mod auth;
use auth::*;
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ory-auth-example.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path=ids::REGISTER_ROUTE view=RegistrationPage/>
                    <Route path=ids::VERIFY_EMAIL_ROUTE view=||view!{<div id=ids::VERIFY_EMAIL_DIV_ID>"Check Email for Verification"</div>}/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <a href="/register" id=ids::REGISTER_BUTTON_ID>Register</a>
    }
}
