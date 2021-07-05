use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    client::{ClientInner, ClientResponse},
    paginator::Paginator,
    user::UserSummary,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Donation {
    pub id: u64,
    pub amount: f64,
    pub name: String,
    pub comment: Option<String>,

    #[serde(rename = "completedAt")]
    pub completed_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Avatar {
    pub alt: String,
    pub src: String,
    pub height: u64,
    pub width: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Campaign {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub status: String,
    #[serde(rename = "amountRaised")]
    pub amount_raised: f64,
    #[serde(rename = "totalAmountRaised")]
    pub total_amount_raised: f64,

    pub supportable: bool,
    #[serde(rename = "supportingAmountRaised")]
    pub supporting_amount_raised: f64,

    pub avatar: Avatar,
    pub user: UserSummary,

    #[serde(rename = "causeCurrency")]
    pub cause_currency: String,

    pub description: String,

    #[serde(rename = "startsAt")]
    pub starts_at: u64,

    #[serde(rename = "endsAt")]
    pub ends_at: Option<u64>,

    #[serde(rename = "fundraiserGoalAmount")]
    pub fundraiser_goal_amount: f64,

    pub r#type: String,
}

pub struct CampaignBuilder {
    campaign_id: String,
    client: Arc<ClientInner>,
}

impl CampaignBuilder {
    pub(crate) fn new(client: Arc<ClientInner>, campaign_id: String) -> Self {
        Self {
            client,
            campaign_id,
        }
    }

    pub async fn get(&self) -> crate::Result<Campaign> {
        let url = format!("/api/v3/campaigns/{}", &self.campaign_id);
        let response = self
            .client
            .get(&url, None)
            .await?
            .json::<ClientResponse<Campaign>>()
            .await?;
        Ok(response.data)
    }

    pub async fn donations(&self) -> crate::Result<Paginator<Donation>> {
        let endpoint = format!("/api/v3/campaigns/{}/donations", &self.campaign_id);
        Ok(Paginator::get(self.client.clone(), &endpoint).await?)
    }
}
