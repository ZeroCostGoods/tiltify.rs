use std::sync::Arc;

use serde::Deserialize;
use serde_json::Value;

use crate::client::ClientInner;

pub struct CampaignBuilder {
    campaign_id: String,
    client: Arc<ClientInner>,
}

#[derive(Deserialize)]
pub struct Donation {}

impl CampaignBuilder {
    pub(crate) fn new(client: Arc<ClientInner>, campaign_id: String) -> Self {
        Self {
            client,
            campaign_id,
        }
    }

    pub async fn get(&self) -> crate::Result<Value> {
        let url = format!("campaigns/{}", &self.campaign_id);
        Ok(self.client.get(&url).await?.json::<Value>().await?)
    }

    pub async fn donations(&self) -> crate::Result<Value> {
        let url = format!("campaigns/{}/donations", &self.campaign_id);
        Ok(self.client.get(&url).await?.json::<Value>().await?)
    }
}
