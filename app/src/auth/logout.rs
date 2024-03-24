use ory_keto_client::models::ErrorGeneric;

use super::*;
use ory_kratos_client::models::logout_flow::LogoutFlow;

#[tracing::instrument(ret)]
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    tracing::debug!("here?");
    use reqwest::StatusCode;

    let cookie_jar = leptos_axum::extract::<axum_extra::extract::CookieJar>().await?;
    let ory_kratos_session = cookie_jar
            .get("ory_kratos_session")
            .ok_or(ServerFnError::new(
                "No `ory_kratos_session` cookie found. Logout shouldn't be visible.",
            ))?;
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    // get logout url
    let resp = client
        .get("http://127.0.0.1:4433/self-service/logout/browser")
        .header(
            "cookie",
            format!(
                "{}={}",
                ory_kratos_session.name(),
                ory_kratos_session.value()
            ),
        )
        .send()
        .await?;
    tracing::debug!("STATUS: {}", resp.status());
    let status =  resp.status();
    if status == StatusCode::NO_CONTENT || status == StatusCode::OK {
        let LogoutFlow {
            logout_token,
            logout_url,
        } = resp.json::<LogoutFlow>().await?;
        tracing::debug!("logout url : {logout_url}");
        let mut resp = client
            .get(logout_url)
            .query(&[("token", logout_token), ("return_to", "/".to_string())])
            .send()
            .await?;
        crate::clear_cookies_inner().await?;
        Ok(())
    } else {
        let location = resp
        .headers()
        .get("Location")
        .ok_or(ServerFnError::new("expecting location in headers"))?
        .to_str()?;
    // Parses the url and takes first query which will be flow=FLOW_ID and we get FLOW_ID at .1
    let location_url = url::Url::parse(location)?;
    tracing::debug!("{}", location_url);
    let id = location_url
        .query_pairs()
        .next()
        .ok_or(ServerFnError::new(
            "Expecting query in location header value",
        ))?
        .1;
    let kratos_err = kratos_error::fetch_error(id.to_string()).await?;
    //let error = resp.json::<ErrorGeneric>().await?;
    Err(ServerFnError::new(kratos_err.to_err_msg()))
    }
}

#[component]
pub fn LogoutButton() -> impl IntoView {
    let logout = Action::<Logout, _>::server();
    view! {
        <button id=ids::LOGOUT_BUTTON_ID on:click=move|_|logout.dispatch(Logout{})>
            Logout
            <ErrorBoundary fallback=|errors|view!{<ErrorTemplate errors/>}>
            { move || logout.value().get().map(|resp|resp.into_view())}
            </ErrorBoundary>
        </button>
    }
}
#[component]
pub fn LogoutPage() -> impl IntoView {
    create_effect(|_| {
        use_navigate()("/", NavigateOptions::default());
    });
    view! {
        Logging out...
    }
}