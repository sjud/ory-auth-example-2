use std::collections::HashMap;

use super::*;
use ory_kratos_client::models::{SettingsFlow, UiContainer, UiText};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ViewableSettingsFlow(SettingsFlow);

impl IntoView for ViewableSettingsFlow {
    fn into_view(self) -> View {
        format!("{self:#?}").into_view()
    }
}

#[tracing::instrument(ret)]
#[server]
pub async fn init_settings_flow(
    flow_id: Option<String>,
) -> Result<ViewableSettingsFlow, ServerFnError> {
    use reqwest::StatusCode;
    let cookie_jar = leptos_axum::extract::<axum_extra::extract::CookieJar>().await?;
    let cookies = cookie_jar
        .iter()
        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
        .collect::<Vec<String>>()
        .join(";");

    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let opts = expect_context::<leptos_axum::ResponseOptions>();
    
    opts.insert_header(
            axum::http::HeaderName::from_static("cache-control"),
            axum::http::HeaderValue::from_str("private, no-cache, no-store, must-revalidate")?,
    );
    if let Some(flow_id) = flow_id {
        // use flow id to get pre-existing session flow

        let resp = client
            .get("http://127.0.0.1:4433/self-service/settings/flows")
            .query(&[("id", flow_id)])
            .header("accept", "application/json")
            .header("cookie", cookies)
            .send()
            .await?;
        if resp.headers().get_all("set-cookie").iter().count() == 0 {
            tracing::error!("set-cookie is empty");
        }
        for value in resp.headers().get_all("set-cookie").iter() {
            tracing::error!("init set cookie {value:#?}");
            opts.append_header(
                axum::http::HeaderName::from_static("set-cookie"),
                axum::http::HeaderValue::from_str(value.to_str()?)?,
            );
        }
        // expecting 200:settingsflow ok 401,403,404,410:errorGeneric
        let status = resp.status();
        if status == StatusCode::OK {
            let flow = resp.json::<SettingsFlow>().await?;
            Ok(ViewableSettingsFlow(flow))
        } else if status == StatusCode::UNAUTHORIZED
            || status == StatusCode::FORBIDDEN
            || status == StatusCode::NOT_FOUND
            || status == StatusCode::GONE
        {
            // 401 should really redirect to login form...


            let err = resp
                .json::<ory_kratos_client::models::ErrorGeneric>()
                .await?;
            Err(ServerFnError::new(format!("{err:#?}")))
        } else {
            tracing::error!("UHHANDLED STATUS : {status}");
            Err(ServerFnError::new("This is a helpful error message."))
        }
    } else {
        // create a new flow

        let resp = client
            .get("http://127.0.0.1:4433/self-service/settings/browser")
            .header("accept", "application/json")
            .header("cookie", cookies)
            .send()
            .await?;
        if resp.headers().get_all("set-cookie").iter().count() == 0 {
            tracing::error!("set-cookie is empty");
        }
        for value in resp.headers().get_all("set-cookie").iter() {
            tracing::error!("init set cookie {value:#?}");
            opts.append_header(
                axum::http::HeaderName::from_static("set-cookie"),
                axum::http::HeaderValue::from_str(value.to_str()?)?,
            );
        }
        // expecting 200:settingsflow ok 400,401,403:errorGeneric
        let status = resp.status();
        if status == StatusCode::OK {
            let flow = resp.json::<SettingsFlow>().await?;
            Ok(ViewableSettingsFlow(flow))
        } else if status == StatusCode::BAD_REQUEST
            || status == StatusCode::UNAUTHORIZED
            || status == StatusCode::FORBIDDEN
        {
            let err = resp
                .json::<ory_kratos_client::models::ErrorGeneric>()
                .await?;
            Err(ServerFnError::new("{err:#?}"))
        } else {
            tracing::error!("UHHANDLED STATUS : {status}");
            Err(ServerFnError::new("This is a helpful error message."))
        }
    }
}

#[tracing::instrument(ret)]
#[server]
pub async fn update_settings(
    flow_id: String,
    body: HashMap<String, String>,
) -> Result<ViewableSettingsFlow, ServerFnError> {
    use ory_kratos_client::models::{
        ErrorBrowserLocationChangeRequired, ErrorGeneric, GenericError,
    };
    use reqwest::StatusCode;

    let cookie_jar = leptos_axum::extract::<axum_extra::extract::CookieJar>().await?;
    let csrf_token = body.get("csrf_token").ok_or(ServerFnError::new("Expecting csrf token in form"))?;

    let cookies = cookie_jar
        .iter()
        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
        .collect::<Vec<String>>()
        .join(";");
    let csrf_cookie = cookie_jar
        .iter()
        .filter(|cookie| cookie.name().contains("csrf_token"))
        .next()
        .ok_or(ServerFnError::new(
            "Expecting a csrf_token cookie to already be set if fetching a pre-existing flow",
        ))?;

    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let resp = client
        .post("http://127.0.0.1:4433/self-service/settings/flows")
        .query(&[("id", flow_id)])
        .header("accept", "application/json")
        .header(
            "cookie",
            format!("{}={}", csrf_cookie.name(), csrf_token),
        )        
        .header("x-csrf-token", csrf_token)
        .json(&body)
        .send()
        .await?;
    
        let opts = expect_context::<leptos_axum::ResponseOptions>();
    
        opts.insert_header(
                axum::http::HeaderName::from_static("cache-control"),
                axum::http::HeaderValue::from_str("private, no-cache, no-store, must-revalidate")?,
        );
        if resp.headers().get_all("set-cookie").iter().count() == 0 {
            tracing::error!("set-cookie is empty");
        }
        for value in resp.headers().get_all("set-cookie").iter() {
            tracing::error!("init set cookie {value:#?}");
            opts.append_header(
                axum::http::HeaderName::from_static("set-cookie"),
                axum::http::HeaderValue::from_str(value.to_str()?)?,
            );
        }
    // https://www.ory.sh/docs/reference/api#tag/frontend/operation/updateSettingsFlow
    // expecting  400,200:settingsflow ok 401,403,404,410:errorGeneric 422:ErrorBrowserLocationChangeRequired
    let status = resp.status();
    if status == StatusCode::OK || status == StatusCode::BAD_REQUEST {
        let flow = resp.json::<SettingsFlow>().await?;
        Ok(ViewableSettingsFlow(flow))
    } else if status == StatusCode::UNAUTHORIZED
        || status == StatusCode::FORBIDDEN
        || status == StatusCode::NOT_FOUND
        || status == StatusCode::GONE
    {   
        /* 
        let ErrorGeneric {
            error: box GenericError { id, message, .. },
        } = resp.json::<ErrorGeneric>().await?;
        if let Some(id) = id {
            match id.as_str() {
                "session_refresh_required" =>
                    /*
                session_refresh_required: The identity requested to change something that needs a privileged session.
                Redirect the identity to the login init endpoint with
                query parameters ?refresh=true&return_to=<the-current-browser-url>,
                or initiate a refresh login flow otherwise.
                 */
                    {}
                "security_csrf_violation" =>
                    /*
                Unable to fetch the flow because a CSRF violation occurred.
                 */
                    {}
                "session_inactive" =>
                    /*
                No Ory Session was found - sign in a user first.
                 */
                    {}
                "security_identity_mismatch" =>
                    /*
                The flow was interrupted with session_refresh_required
                but apparently some other identity logged in instead.

                or

                 The requested ?return_to address is not allowed to be used.
                 Adjust this in the configuration!

                 ?
                 */
                    {}
                "browser_location_change_required" =>
                    /*
                Usually sent when an AJAX request indicates that the browser
                needs to open a specific URL. Most likely used in Social Sign In flows.
                */
                    {}
                _ => {}
            }
        }
        */
        let err = resp.json::<ErrorGeneric>().await?;
        let err = format!("{err:#?}");
        tracing::error!(err);
        Err(ServerFnError::new(err))
    } else if status == StatusCode::UNPROCESSABLE_ENTITY {
        let body = resp.json::<ErrorBrowserLocationChangeRequired>().await?;
        tracing::error!("{body:#?}");
        Err(ServerFnError::new("Unprocessable."))
    } else {
        tracing::error!("UHHANDLED STATUS : {status}");
        Err(ServerFnError::new("This is a helpful error message."))
    }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    // get flow id from url
    // if flow id doesn't exist we create a settings flow
    // otherwise we fetch the settings flow with the flow id
    // we update the settings page with the ui nodes
    // we handle update settings
    // if we are not logged in we'll be redirect to a login page

    let init_settings_flow_resource = create_local_resource(
        // use untracked here because we don't expect the url to change after resource has been fetched.
        || use_query_map().get_untracked().get("flow").cloned(),
        |flow_id| init_settings_flow(flow_id),
    );
    let update_settings_action = Action::<UpdateSettings, _>::server();
    let flow = Signal::derive(move || {
        if let Some(flow) = update_settings_action.value().get() {
            Some(flow)
        } else {
            init_settings_flow_resource.get()
        }
    });
    let body = create_rw_signal(HashMap::new());
    view! {
    <Suspense fallback=||"loading settings...".into_view()>
        <ErrorBoundary fallback=|errors|view!{<ErrorTemplate errors/>}>
            {
                move || flow.get().map(|resp|
                    match resp {
                        Ok(
                            ViewableSettingsFlow(SettingsFlow{id,ui:box UiContainer{nodes,action,messages,..},..})
                        ) => {
                            let form_inner_html = nodes.into_iter().map(|node|kratos_html(node,body)).collect_view();
                        body.update(move|map|{_=map.insert(String::from("action"),action);});
                        let id = create_rw_signal(id);
                            view!{
                                <form id=ids::SETTINGS_FORM_ID
                                on:submit=move|e|{
                                    e.prevent_default();
                                    e.stop_propagation();
                                    update_settings_action.dispatch(UpdateSettings{flow_id:id.get_untracked(),body:body.get_untracked()});
                                }>
                                {form_inner_html}
                                {messages.map(|messages|{
                                    view!{
                                        <For
                                            each=move || messages.clone().into_iter()
                                            key=|text| text.id
                                            children=move |text: UiText| {
                                              view! {
                                                <p id=text.id>{text.text}</p>
                                              }
                                            }
                                        />
                                    }
                                }).unwrap_or_default()}
                                </form>
                            }.into_view()
                        },
                        err => err.into_view()
                    })
                }
            </ErrorBoundary>
            </Suspense>
        }
}
