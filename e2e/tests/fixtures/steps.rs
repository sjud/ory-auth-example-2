
use crate::AppWorld;
use anyhow::{Ok, Result};
use cucumber::{given, when,then};
use fake::faker::name;
use fake::locales::EN;
use fake::{
    faker::internet::raw::FreeEmail,
    Fake,
};
use thirtyfour::extensions::query::ElementQueryable;
use thirtyfour::By;
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
    world.find(ids::REGISTRATION_FORM_ID).await?;
    world.form_and_update(ids::REGISTRATION_FORM_ID).await?;
    Ok(())
}


#[given("I am on the registration page")]
pub async fn navigate_to_register(world:&mut AppWorld) -> Result<()> {
    world.goto_path("/register").await?;
    Ok(())
}

#[when("I enter valid credentials")]
pub async fn fill_form_fields_with_credentials(world:&mut AppWorld) -> Result<()> {
    world.set_field(ids::EMAIL_INPUT_ID, &FreeEmail(EN).fake::<String>()).await?;
    world.set_field(ids::PASSWORD_INPUT_ID,"SuPeRsAfEpAsSwOrD1234!").await?;
    world.submit_form().await?;
    world.errors().await?;
    Ok(())
}

#[then("I am on the verify email page")]
pub async fn check_url_to_be_verify_page(world:&mut AppWorld) -> Result<()> {
    world.find(ids::VERIFY_EMAIL_DIV_ID).await?;
    world.verify_route(ids::VERIFY_EMAIL_ROUTE).await?;
    Ok(())
}

