mod fixtures;

use anyhow::Result;
use cucumber::World;
use fixtures::AppWorld;

#[tokio::main]
async fn main() -> Result<()> {

    AppWorld::cucumber()
        .init_tracing()
        .fail_on_skipped()
        .max_concurrent_scenarios(1)
        .fail_fast()
        .run_and_exit("./features")
        .await;
    Ok(())
}