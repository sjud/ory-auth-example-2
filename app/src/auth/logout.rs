<<<<<<< HEAD
=======

>>>>>>> e1b880d (idk)
use ory_keto_client::models::ErrorGeneric;

use super::*;
use ory_kratos_client::models::logout_flow::LogoutFlow;

#[tracing::instrument(ret)]
#[server]
<<<<<<< HEAD
pub async fn logout() -> Result<(), ServerFnError> {
=======
pub async fn logout() -> Result<(),ServerFnError> {
>>>>>>> e1b880d (idk)
    tracing::debug!("here?");
    use reqwest::StatusCode;

    let cookie_jar = leptos_axum::extract::<axum_extra::extract::CookieJar>().await?;
<<<<<<< HEAD
    let ory_kratos_session = cookie_jar
            .get("ory_kratos_session")
            .ok_or(ServerFnError::new(
                "No `ory_kratos_session` cookie found. Logout shouldn't be visible.",
            ))?;
=======
    let token = cookie_jar.get("ory_kratos_session").ok_or(ServerFnError::new("No `ory_kratos_session` cookie found. Logout shouldn't be visible."))?;
    let session_token = token.value().to_string();
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        //.redirect(reqwest::redirect::Policy::none())
        .build()?;
    // get logout url
    // post logout url with the ory_session token?
    let resp =  client.get("http://127.0.0.1:4433/self-service/logout/browser")
        .header("ory_kratos_session", session_token)
        .send()
        .await?;
    let _headers = resp.headers();
    tracing::debug!("status: {}",resp.status());
    let LogoutFlow{logout_token,logout_url} = resp
        .json::<LogoutFlow>()
        .await?;
    tracing::debug!("logout url : {logout_url}");
    let mut resp = client.get(logout_url)
        .query(&[("token",logout_token),("return_to","/".to_string())])
        .send()
        .await?;
    if resp.status() != StatusCode::NO_CONTENT {
        tracing::debug!("STATUS: {}",resp.status());
        let error = resp.json::<ErrorGeneric>().await?;
        return Err(ServerFnError::new(format!("{:#?}",error)));
    }
    crate::clear_cookies_inner().await?;
    Ok(())
}
/* 
#[tracing::instrument(ret)]
#[server]
pub async fn logout() -> Result<(),ServerFnError> {
    use reqwest::StatusCode;
    use ory_kratos_client::models::generic_error::GenericError;

    let cookie_jar = leptos_axum::extract::<axum_extra::extract::CookieJar>().await?;
    let token = cookie_jar.get("ory_kratos_session").ok_or(ServerFnError::new("No `ory_kratos_session` cookie found. Logout shouldn't be visible."))?;
    let session_token = token.value().to_string();
>>>>>>> e1b880d (idk)
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
<<<<<<< HEAD
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
=======
    #[derive(Serialize)]
    struct SessionToken{session_token:String}
    let session_token = serde_json::to_string(&SessionToken{session_token})?;
    tracing::debug!("{session_token}");
    let resp = client.delete("http://127.0.0.1:4433/self-service/logout/api")
        .header("content-type", "application/json")
        .body(session_token)
        .send()
        .await?;
    // https://www.ory.sh/docs/reference/api#tag/frontend/operation/performNativeLogout
    match resp.status() {
        StatusCode::NO_CONTENT => {
            crate::clear_cookies_inner().await?;
            Ok(())
        },
        StatusCode::FORBIDDEN | StatusCode::BAD_REQUEST => {
            let err = resp.json::<ErrorGeneric>().await?;
            let err = format!("{:#?}", err);
            tracing::debug!(err);
            Err(ServerFnError::new(err))
        },
        status => {
            tracing::debug!("UNHANDLED: {status}");
            Err(ServerFnError::new("Unhandled status"))
        }
    }
}*/

#[component]
pub fn LogoutButton() -> impl IntoView {
    let logout = Action::<Logout,_>::server();
    create_effect(move|_|
        if let Some(resp) = logout.value().get() {
            match resp {
                Ok(_) => use_navigate()("/logout",NavigateOptions::default()),
                Err(err) => leptos::logging::error!("{err:#?}"),
            }
        }
    );
    view!{
        <button id=ids::LOGOUT_BUTTON_ID on:click=move|_|logout.dispatch(Logout{})>
            Logout
>>>>>>> e1b880d (idk)
        </button>
    }
}
#[component]
pub fn LogoutPage() -> impl IntoView {
<<<<<<< HEAD
    create_effect(|_| {
        use_navigate()("/", NavigateOptions::default());
    });
    view! {
        Logging out...
    }
}
=======
    create_effect(|_|
        {use_navigate()("/",NavigateOptions::default());}
    );
    view!{
        Logging out...
    }
}
>>>>>>> e1b880d (idk)
