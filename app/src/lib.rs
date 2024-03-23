#![feature(box_patterns)]

use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod auth;
#[cfg(feature = "ssr")]
pub mod business_logic;
pub mod error_template;
use auth::*;
use leptos_use::utils::FromToStringCodec;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct IsLoggedIn(RwSignal<bool>);
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
                    <Route path=ids::VERIFICATION_ROUTE view=VerificationPage/>
                    <Route path=ids::LOGIN_ROUTE view=LoginPage/>
                    <Route path=ids::KRATOS_ERROR_ROUTE view=KratosErrorPage/>
                    <Route path="logout" view=LogoutPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
<<<<<<< HEAD
    let clear_cookies = Action::<ClearCookies, _>::server();
=======
    let clear_cookies = Action::<ClearCookies,_>::server();
>>>>>>> e1b880d (idk)
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <div>
            <a href="/register" id=ids::REGISTER_BUTTON_ID>Register</a>
        </div>
        <div>
<<<<<<< HEAD
            <a href="/login" id=ids::LOGIN_BUTTON_ID>"Login"</a>
        </div>
        <div>
            <LogoutButton/>
        </div>
        <div>
            <button on:click=move|_|clear_cookies.dispatch(ClearCookies{})>Clear cookies </button>
        </div>
        <div>
            <HasSession/>
=======
        <a href="/login" id=ids::LOGIN_BUTTON_ID>"Login"</a>
        </div>
        <div>
        <LogoutButton/>
        </div>
        <div>
        <button on:click=move|_|clear_cookies.dispatch(ClearCookies{})>Clear cookies </button>
>>>>>>> e1b880d (idk)
        </div>
    }
}

<<<<<<< HEAD
#[cfg(feature = "ssr")]
pub async fn clear_cookies_inner() -> Result<(), ServerFnError> {
=======
#[cfg(feature="ssr")]
pub async fn clear_cookies_inner() -> Result<(),ServerFnError> {
>>>>>>> e1b880d (idk)
    let opts = expect_context::<leptos_axum::ResponseOptions>();

    let cookie_jar = leptos_axum::extract::<axum_extra::extract::CookieJar>().await?;
    for cookie in cookie_jar.iter() {
        let mut cookie = cookie.clone();
<<<<<<< HEAD
        cookie.set_expires(
            time::OffsetDateTime::now_utc()
                .checked_sub(time::Duration::hours(24 * 356 * 10))
                .unwrap(),
        );
=======
        cookie.set_expires(time::OffsetDateTime::now_utc().checked_sub(time::Duration::hours(24*356*10)).unwrap());
>>>>>>> e1b880d (idk)
        cookie.set_max_age(time::Duration::seconds(0));
        cookie.set_path("/");
        // To clear an http only cookie, one must set an http only cookie.
        cookie.set_http_only(true);
        cookie.set_secure(true);
        let cookie = cookie.to_string();
        opts.append_header(
            axum::http::HeaderName::from_static("set-cookie"),
            axum::http::HeaderValue::from_str(&cookie)?,
        );
    }
    Ok(())
}

#[tracing::instrument]
#[server]
<<<<<<< HEAD
pub async fn clear_cookies() -> Result<(), ServerFnError> {
    clear_cookies_inner().await?;
    Ok(())
}
=======
pub async fn clear_cookies() -> Result<(),ServerFnError> {
    clear_cookies_inner().await?;
    Ok(())
}

>>>>>>> e1b880d (idk)
