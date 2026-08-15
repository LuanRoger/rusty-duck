mod native;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    native::init().await?;
    Ok(())
}
