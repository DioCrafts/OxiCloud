use crate::background_job::Job;
use crate::cache::FileGlobal;
use async_trait::async_trait;

pub struct FileGlobalGC {}

#[async_trait]
impl Job for FileGlobalGC {
    async fn run(&self, _argument: Option<serde_json::Value>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        FileGlobal::gc().await?;
        Ok(())
    }
}