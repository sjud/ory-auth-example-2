use std::str::FromStr;

use super::AppWorld;
use anyhow::{Ok, Result,anyhow};
use cucumber::{given, when,then};
use fantoccini::Locator;
// EDITOR BEGIN

#[given("I pass")]
pub async fn i_pass(_world:&mut AppWorld) -> Result<()> {
    tracing::info!("I pass and I trace.");
    Ok(())
}

#[given("I am on the homepage")]
pub async fn navigate_to_homepage(world:&mut AppWorld) -> Result<()> {
    world.goto_path("/").await?;
    Ok(())
}

#[when("I click register")]
pub async fn click_register(world:&mut AppWorld) -> Result<()> {
    world.find_and_update(ids::REGISTER_BUTTON_ID).await?;
    world.click().await?;
    Ok(())
}

#[given("I see the registration form")]
#[then("I see the registration form")]
pub async fn find_registration_form(world:&mut AppWorld) -> Result<()> {
    world.form_and_update(ids::REGISTRATION_FORM_ID).await?;
    Ok(())
}