#![feature(box_patterns)]

use crate::{error_template::{AppError, ErrorTemplate}};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;
pub mod auth;
#[cfg(feature="ssr")]
pub mod business_logic;
use auth::*;

#[derive(Clone,Copy,PartialEq,Debug,Default)]
pub struct LoggedIn(bool);
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(create_rw_signal(LoggedIn::default()));
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
                    <Route path=ids::VERIFICATION_ROUTE view=VerificationPage/>
                    <Route path=ids::LOGIN_ROUTE view=LoginPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let logged_in = expect_context::<RwSignal<LoggedIn>>();
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <div>
        <a href="/register" id=ids::REGISTER_BUTTON_ID>Register</a>
        </div>
        <div>
        <Show 
            when=move||!logged_in().0 
            fallback=||view!{<a href="/" id=ids::LOGOUT_BUTTON_ID>"Logout"</a>}
            >
            <a href="/login" id=ids::LOGIN_BUTTON_ID>"Login"</a>
        </Show>
        </div>
    }
}
