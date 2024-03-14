pub mod steps;
use tracing::{instrument,info};
use std::{str::FromStr};

use anyhow::{Result,anyhow};
use cucumber::World;
use fantoccini::{
    elements::{Element,Form},
    error::NewSessionError, 
    wd::Capabilities, 
    Client, 
    ClientBuilder, 
    Locator,
};

pub const HOST: &str = "http://host.docker.internal:3000";

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct AppWorld {
    pub client: Client,
    pub current_element:Option<Element>,
    pub current_form:Option<Form>,
}


impl AppWorld {
    async fn new() -> Result<Self, anyhow::Error> {
        let webdriver_client = build_client().await?;

        Ok(Self {
            client: webdriver_client,
            current_element:None,
            current_form:None,
        })
    }


    pub async fn find(&self, id:&'static str) -> Result<Element> {
        Ok(self.client
            .find(Locator::Id(id))
            .await?)
    }
    pub async fn find_and_update(&mut self, id:&'static str) -> Result<()> {
        self.current_element = Some(self.find(id).await?);
        Ok(())
    }
    pub async fn find_all(&self, id:&'static str) -> Result<ElementList> {
        Ok(ElementList(
            self.client
                .find_all(Locator::Id(id))
                .await?
        ))
    }
    pub async fn form(&self, id:&'static str) -> Result<Form> {
        Ok(self.client
            .form(Locator::Id(id))
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
        info!("START GOTO:{url}");
        self.client
            .goto(&url)
            .await?;
        info!("FINISHED GOTO:{url}");
        Ok(())
    }
    pub async fn find_child_of_element(&mut self,id:&'static str) -> Result<Element> {
        let element = self.current_element.clone()
            .ok_or(anyhow!(format!("current element is none, therefore child id of :{} is invalid",id)))?;
        let element = element.find(Locator::Id(id)).await?;
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
        let elements = element.find_all(Locator::Id(id)).await?;
        Ok(ElementList(elements))
    }
    pub async fn set_field<S:AsRef<str> + std::fmt::Display>(&mut self, id:&'static str,value:S) -> Result<()> {
        let form = self.current_form
            .clone()
            .ok_or(anyhow!("There is no current for, thus no relevant field with id {} for value {}",id,value))?;
        form.set(Locator::Id(id), value.as_ref()).await?;
        Ok(())
    }
    pub async fn click(&self) -> Result<()> {
        self.current_element.clone().ok_or(anyhow!("Can't click because no current element."))?.click().await?;
        Ok(())
    }

}

#[derive(Clone,Debug)]
pub struct ElementList(Vec<Element>);

impl ElementList {
    /// iterates over elements, finds first element whose text (as rendered) contains text given as function's argument.
    pub async fn find_by_text(&self,text:&'static str) -> Result<Element> {
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

async fn build_client() -> Result<Client, NewSessionError> {
    let mut cap = Capabilities::new();
    // --remote-debugging-pipe solves https://bugs.chromium.org/p/chromedriver/issues/detail?id=4403#c35  Issue 4403: DevToolsActivePort not found
    // pulled out \"--headless\", 
    // everything else is because of time out issues?
    let arg = serde_json::from_str("{\"args\": [\"--enable-automation\",\"--remote-debugging-pipe\",\"--no-sandbox\",\"--disable-extensions\",\"--disable-dns-prefetch\",\"--disable-gpu\"]}").unwrap();
    cap.insert("goog:chromeOptions".to_string(), arg);
    let client = ClientBuilder::native()
        .capabilities(cap)
        .connect("http://localhost:4444")
        .await?;

    Ok(client)
}