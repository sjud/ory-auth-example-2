
mod fixtures;

use std::collections::HashMap;

use anyhow::Result;
use cucumber::World;
use thirtyfour::{extensions::{cdp::ChromeDevTools, query::ElementQueryOptions}, fantoccini::Locator, prelude::*, Form};
use tracing::instrument;
use thirtyfour::extensions::query::ElementQueryWaitOptions;
use anyhow::anyhow;
use tokio_tungstenite::connect_async;
use futures_util::stream::StreamExt;
use uuid::Uuid;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use serde::{Serialize,Deserialize};

static EMAIL_ID_MAP: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

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
        //.max_concurrent_scenarios(1)
        .fail_fast()
        .after(|_feature, _rule, _scenario, ev, world| Box::pin(async move {
            if let cucumber::event::ScenarioFinished::StepFailed(_,_,_) = ev {
                let client = &world.unwrap().client;
                // take a screenshot
                client
                .screenshot(&std::path::Path::new("./chromedriver_screenshot.png"))
                .await
                .unwrap();
                // print network

            }
        }))
        .run_and_exit("./features")
        .await;
    Ok(())
}

#[tracing::instrument]
async fn build_client() -> WebDriverResult<WebDriver> {
    let mut cap = DesiredCapabilities::chrome();
    cap.set_headless().unwrap();
    //cap.set_disable_gpu().unwrap();
    //cap.set_no_sandbox().unwrap();
    //cap.add_chrome_arg("--enable-automation").unwrap();
    //cap.add_chrome_arg("--remote-debugging-pipe").unwrap();
    //cap.add_chrome_arg("--disable-extensions").unwrap();
    //cap.add_chrome_arg("--disable-dns-prefetch").unwrap();
    cap.set_ignore_certificate_errors().unwrap();
    // --remote-debugging-pipe solves https://bugs.chromium.org/p/chromedriver/issues/detail?id=4403#c35  Issue 4403: DevToolsActivePort not found
    // pulled out \"--headless\", 
    // everything else is because of time out issues?
    // except --accept-insecure-certs is to get https working with our mkcert certifcates

    tracing::info!("connecting to webdriver");
    let driver = WebDriver::new("http://localhost:4444",cap)
        .await?;
    let timeouts = TimeoutConfiguration::new(None, Some(std::time::Duration::from_secs(5)), None);
    driver.update_timeouts(timeouts).await?;
    Ok(driver)
    //let dev_tools = ChromeDevTools::new(client.handle.clone());
}

pub const HOST: &str = "https://host.docker.internal:3000";


#[derive(World)]
#[world(init = Self::new)]
pub struct AppWorld {
    pub client : WebDriver,
    pub current_element:Option<WebElement>,
    pub current_form:Option<Form>,
}

impl std::fmt::Debug for AppWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppWorld")
            // We skip the 'client' field since we don't want it included in the debug output
            .field("current_element", &self.current_element)
            .field("current_form", &self.current_form)
            .finish()
    }
}

impl AppWorld {
    async fn new() -> Result<Self, anyhow::Error> {
        let client = build_client().await.unwrap();

        Ok(Self {
            client,
            current_element:None,
            current_form:None,
        })
    }
    
    pub async fn errors(&self) -> Result<()> {
        if let Ok(error) = self.find(ids::ERROR_ERROR_ID).await {
            Err(anyhow!("{}",error.text().await.unwrap()))
        } else {
            Ok(())
        }
    }

    pub async fn find(&self, id:&'static str) -> Result<WebElement> {
        Ok(self.client
            .query(By::Id(id))
            .options(ElementQueryOptions::default().set_wait(Some(
                    ElementQueryWaitOptions::Wait { 
                        timeout: std::time::Duration::from_secs(1), 
                        interval: std::time::Duration::from_millis(100)
                    })
                )
            )
            .first()
            .await?)
    }
    pub async fn find_and_update(&mut self, id:&'static str) -> Result<()> {
        self.current_element = Some(self.find(id).await?);
        Ok(())
    }
    pub async fn find_all(&self, id:&'static str) -> Result<ElementList> {
        Ok(ElementList(
            self.client
                .find_all(By::Id(id))
                .await?
        ))
    }

    pub async fn form(&self, id:&'static str) -> Result<Form> {
        Ok(self.client
            .form(By::Id(id))
            .await?)
    }
    
    pub async fn form_and_update(&mut self,id:&'static str) -> Result<()> {
        self.current_form = Some(self.form(id).await?);
        Ok(())
    }
    pub async fn submit_form(&self) -> Result<()> {
        self.current_form
        .clone()
        .ok_or(anyhow!("Current form is none, thus can't submit"))?
        .submit()
        .await?;
        Ok(())
    }
    #[instrument(skip(self))]
    pub async fn goto_path(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", HOST, path);
        self.client
            .goto(&url)
            .await?;
        Ok(())
    }
    pub async fn find_child_of_element(&mut self,id:&'static str) -> Result<WebElement> {
        let element = self.current_element.clone()
            .ok_or(anyhow!(format!("current element is none, therefore child id of :{} is invalid",id)))?;
        let element = element.find(By::Id(id)).await?;
        Ok(element)
    }

    pub async fn find_child_of_element_and_update(&mut self,id:&'static str) -> Result<()> {
        let element = self.find_child_of_element(id).await?;
        self.current_element = Some(element);
        Ok(())
    }
    pub async fn find_children_of_element(&self,id:&'static str) -> Result<ElementList>{
        let element = self.current_element.clone()
            .ok_or(anyhow!(format!("current element is none, therefore child id of :{} is invalid",id)))?;
        let elements = element.find_all(By::Id(id)).await?;
        Ok(ElementList(elements))
    }
    pub async fn set_field<S:AsRef<str> + std::fmt::Display>(&mut self, id:&'static str,value:S) -> Result<()> {
        let element = self.find(id).await?;
        element.send_keys(value).await?;
        
        Ok(())
    }
    pub async fn click(&self) -> Result<()> {
        self.current_element.clone().ok_or(anyhow!("Can't click because no current element."))?.click().await?;
        Ok(())
    }
    pub async fn verify_route(&self,path:&'static str) -> Result<()> {
        let current = self.client
        .current_url().await?;
        if current.path() != path {
            return Err(anyhow!("EXPECTING ROUTE: {path}\n but FOUND:\n {current:#?}"));
        }
        Ok(())
    }

}

#[derive(Clone,Debug)]
pub struct ElementList(Vec<WebElement>);

impl ElementList {
    /// iterates over elements, finds first element whose text (as rendered) contains text given as function's argument.
    pub async fn find_by_text(&self,text:&'static str) -> Result<WebElement> {
        for element in self.0.iter() {
            if let Ok(inner_text) = element.text().await {
                if inner_text.contains(text) {
                    return Ok(element.clone());
                }
            }
        }
        Err(anyhow!(format!("given text {} no element found",text)))
    }

}


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