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
