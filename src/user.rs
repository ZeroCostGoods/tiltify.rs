use std::sync::Arc;

use serde::Deserialize;

use crate::{campaign::Campaign, client::{ClientInner, ClientResponse}};

#[derive(Deserialize, Debug)]
pub struct Avatar {
    pub alt: String,
    pub src: String,
    pub height: u64,
    pub width: u64,
}

#[derive(Deserialize, Debug)]
pub struct Social {
    pub discord: Option<String>,
    pub youtube: Option<String>,
    pub twitch: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub mixer: Option<String>,
    pub facebook: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub slug: String,
    pub url: String,
    pub avatar: Avatar,
    pub about: Option<String>,
    #[serde(rename = "totalAmountRaised")]
    pub total_amount_raised: f64,
    pub social: Social,
}

#[derive(Deserialize, Debug)]
pub struct UserSummary {
    pub id: u64,
    pub username: String,
    pub slug: String,
    pub url: String,
    pub avatar: Avatar,
}

pub struct UserBuilder {
    user_id: String,
    client: Arc<ClientInner>,
}

impl UserBuilder {
    pub(crate) fn new(client: Arc<ClientInner>, user_id: String) -> Self {
        Self { client, user_id }
    }

    pub async fn get(&self) -> crate::Result<User> {
        let url = format!("/api/v3/users/{}", &self.user_id);
        let response = self
            .client
            .get(&url, None)
            .await?
            .json::<ClientResponse<User>>()
            .await?;
        Ok(response.data)
    }

    pub async fn campaigns(&self) -> crate::Result<Vec<Campaign>> {
        let url = format!("/api/v3/users/{}/campaigns", &self.user_id);
        let response = self.client.get(&url, None).await?.json::<ClientResponse<Vec<Campaign>>>().await?;
        Ok(response.data)
    }
}
