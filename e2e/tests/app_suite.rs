#![feature(never_type)]
mod fixtures;

use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::Result;
use cucumber::World;
use tracing::instrument;
use anyhow::anyhow;
use tokio_tungstenite::connect_async;
use futures_util::stream::StreamExt;
use uuid::Uuid;
use once_cell::sync::Lazy;
use tokio::{sync::RwLock};
use serde::{Serialize,Deserialize};
use chromiumoxide::{
    page::{ScreenshotParams},
    browser::{Browser, BrowserConfig},
     cdp::browser_protocol::{fetch::RequestId, network::{EventRequestWillBeSent, EventResponseReceived, Request, Response}, page::NavigateParams},
      element::Element, 
      Handler, Page
    };
static EMAIL_ID_MAP: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

#[derive(Clone,Debug,PartialEq)]
pub struct RequestPair{
    req:Option<Request>,
    resp:Option<Response>,
}
impl RequestPair{
    pub fn to_string(&self) -> String {
        let req = 
        if let Some(req) = &self.req {
            format!("
            {}\n{}\n{:#?}\n{}
            ",req.method,req.url,req.headers,req.post_data.clone().unwrap_or_default())
        } else {
            String::new()
        };
        let resp = 
        if let Some(resp) = &self.resp {
            format!("
            {}\n{}\n{:#?}
            ",resp.status,resp.url,resp.headers)
        } else {
            String::new()
        };
        format!("{}\n{}",req,resp)
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    // create a thread and store a
    //  tokio-tungstenite client that connectsto http://127.0.0.1:1080/ws
    // and then stores the recieved messages in a once_cell::Lazy<RwLock<Vec<MailCrabMsg>>>
    // or a custom struct that matches the body or has specific impls for verify codes, links etc.
    let _ = tokio::spawn(async move {
        let (mut socket, _) = connect_async(
            url::Url::parse("ws://127.0.0.1:1080/ws").expect("Can't connect to case count URL"),
        )
        .await
        .unwrap();
    while let Some(msg) = socket.next().await {
        if let tokio_tungstenite::tungstenite::Message::Text(text) = msg.unwrap() {
            let Email{id,to} = serde_json::from_str::<Email>(&text).unwrap();
            let email = to[0].email.clone();
            tracing::info!("Inserting into EMAIL_ID_MAP : {email}, {id}");
            EMAIL_ID_MAP.write().await.insert(email, id.to_string());
        }
    }
    });
    AppWorld::cucumber()
        .init_tracing()
        .fail_on_skipped()
        .max_concurrent_scenarios(1)
        .fail_fast()
        .before(|_feature,_rule,_scenario,world| Box::pin(async move {
            // take the page from world
            // add network event listener, tracking requests and pairing them with responses
            // store them somewhere inside of the world.
            let mut events = world.page.event_listener::<EventRequestWillBeSent>().await.unwrap();
            let req_resp = world.req_resp.clone();
            tokio::task::spawn(async move {
                while let Some(event) = events.next().await {
                    let event: EventRequestWillBeSent = Arc::<EventRequestWillBeSent>::try_unwrap(event).unwrap();
                    req_resp.write().await.insert(event.request_id.inner().clone(),RequestPair{ req:Some(event.request), resp: None });
                }
            });
            let mut events = world.page.event_listener::<EventResponseReceived>().await.unwrap();
            let req_resp = world.req_resp.clone();
            tokio::task::spawn(async move {
                while let Some(event) = events.next().await {
                    let event: EventResponseReceived = Arc::<EventResponseReceived>::try_unwrap(event).unwrap();
                    let req_id = event.request_id.inner().clone();
                    if let Some(request_pair) = req_resp.write().await.get_mut(&req_id) {
                        request_pair.resp = Some(event.response);
                    } else {
                        req_resp.write().await.insert(req_id.clone(),RequestPair{req:None,resp:Some(event.response)});
                    }
                }
            });
            // We don't need to join on our join handles, they will run detached and clean up whenever.
        }))
        .after(|_feature, _rule, _scenario, ev, world| Box::pin(async move {
            let world = world.unwrap();
            if let cucumber::event::ScenarioFinished::StepFailed(_,_,_) = ev {
                world.page
                .save_screenshot(
                ScreenshotParams::builder()
                            .capture_beyond_viewport(true)
                            .full_page(true)
                            .build(),
                        "./chromedriver_screenshot.png"
                )
                .await
                .unwrap();
                // print network
                let network_output = world.req_resp.read().await.values().map(|val|val.to_string())
                    .collect::<Vec<String>>().join("\n");
                std::fs::write("./network_output", network_output.as_bytes()).unwrap();
                // print html
                let html = world.page.content().await.unwrap();
                std::fs::write("./html", html.as_bytes()).unwrap();
            }
            world.browser.close().await.unwrap();
            world.browser.wait().await.unwrap();
        }))
        .run_and_exit("./features")
        .await;
    Ok(())
}

#[tracing::instrument]
async fn build_browser() -> Result<Browser, Box<dyn std::error::Error>> {
    let ( browser, mut handler) =
    Browser::launch(BrowserConfig::builder()
        //.enable_request_intercept()
        .disable_cache()
        .request_timeout(Duration::from_secs(1))
        //.with_head()
        //.arg("--remote-debugging-port=9222")
        .build()?)
        .await?;

        tokio::task::spawn(async move {
            while let Some(h) = handler.next().await {

                if h.is_err() {
                    tracing::info!("{h:?}");
                    break;
                }
            }
        });

   Ok(browser)
}

pub const HOST: &str = "https://127.0.0.1:3000";


#[derive(World)]
#[world(init = Self::new)]
pub struct AppWorld {
    pub browser:Browser,
    pub page:Page,
    pub current_element:Option<Element>,
    pub req_resp:Arc<RwLock<HashMap<String,RequestPair>>>,
    pub clipboard:HashMap<&'static str,String>,
}

impl std::fmt::Debug for AppWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppWorld")
            .finish()
    }
}

impl AppWorld {
    async fn new() -> Result<Self, anyhow::Error> {
        let browser = build_browser().await.unwrap();
 
        let page = browser
            .new_page("about:blank")
            .await?;
        
        Ok(Self {
            browser,
            page,
            req_resp:Arc::new(RwLock::new(HashMap::new())),
            current_element:None,
            clipboard:HashMap::new(),
        })
    }

    
    pub async fn errors(&self) -> Result<()> {
        if let Ok(error) = self.find(ids::ERROR_ERROR_ID).await {
            Err(anyhow!("{}",error.inner_text().await.unwrap().unwrap()))
        } else {
            Ok(())
        }
    }

    pub async fn find(&self, id:&'static str) -> Result<Element> {
        let mut count = 0 ;
        loop {
            let result = self.page
            .find_element(format!("#{id}"))
            .await;
            if result.is_err() && count < 4 {
                count += 1;
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;   
            } else {
                return Ok(result?);
            }
        }
    }
    pub async fn find_submit(&self) -> Result<Element> {
        let mut count = 0 ;
        loop {
            let result = self.page
            .find_element(format!("input[type=submit]"))
            .await;
            if result.is_err() && count < 4 {
                count += 1;
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;   
            } else {
                return Ok(result?);
            }
        }
    }
    pub async fn find_and_update(&mut self, id:&'static str) -> Result<()> {
        self.current_element = Some(self.find(id).await?);
        Ok(())
    }
    pub async fn find_all(&self, id:&'static str) -> Result<ElementList> {
        Ok(ElementList(
            self.page
                .find_elements(format!("#{id}"))
                .await?
        ))
    }


     #[instrument(skip(self))]
    pub async fn goto_url(&self, url: &str) -> Result<()> {
        self.page
            .goto(NavigateParams::builder()
                .url(url)
                .build()
                .map_err(|err|anyhow!(err))?
            )
            .await?
            .wait_for_navigation()
            .await?;
        Ok(())
    }
    #[instrument(skip(self))]
    pub async fn goto_path(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", HOST, path);
        self.page
            .goto(NavigateParams::builder()
                .url(url)
                .build()
                .map_err(|err|anyhow!(err))?
            )
            .await?;
        Ok(())
    }

    
    pub async fn set_field<S:AsRef<str> + std::fmt::Display>(&mut self, id:&'static str,value:S) -> Result<()> {
        let element = self.find(id).await?;
        element.click().await?.type_str(value).await?;
        
        Ok(())
    }

    pub async fn click(&self,id:&'static str) -> Result<()> {
        self.find(id).await?.click().await?;
        Ok(())
    }
    pub async fn submit(&self) -> Result<()> {
        self.find_submit().await?.click().await?;
        Ok(())
    }
    
    pub async fn verify_route(&self,path:&'static str) -> Result<()> {
        let url = format!("{}{}", HOST, path);
        if let Some(current) = self.page
        .url().await? {
            if current != url {
                return Err(anyhow!("EXPECTING ROUTE: {path}\n but FOUND:\n {current:#?}"));
            }
        } else {
            return Err(anyhow!("EXPECTING ROUTE: {path}\n but NO CURRENT URL FOUND"));
        }
        Ok(())
    }

}

#[derive(Debug)]
pub struct ElementList(Vec<Element>);
/*
impl ElementList {
    /// iterates over elements, finds first element whose text (as rendered) contains text given as function's argument.
    pub async fn find_by_text(&self,text:&'static str) -> Result<Element> {
        for element in self.0.iter() {
            if let Ok(Some(inner_text)) = element.inner_text().await {
                if inner_text.contains(text) {
                    return Ok(element);
                }
            }
        }
        Err(anyhow!(format!("given text {} no element found",text)))
    }

}*/


#[derive(Serialize, Deserialize, Debug)]
struct Email {
    id: Uuid,
    to: Vec<Recipient>,
}


#[derive(Serialize, Deserialize, Debug)]
struct Recipient {
    name: Option<String>,
    email: String,
}