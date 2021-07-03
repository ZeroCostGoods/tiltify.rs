use std::sync::Arc;

use serde::Deserialize;

use crate::{campaign::Campaign, client::{ClientInner, ClientResponse}};

#[derive(Deserialize, Debug)]
pub struct Avatar {
    alt: String,
    src: String,
    height: u64,
    width: u64,
}

#[derive(Deserialize, Debug)]
pub struct Social {
    discord: Option<String>,
    youtube: Option<String>,
    twitch: Option<String>,
    twitter: Option<String>,
    website: Option<String>,
    mixer: Option<String>,
    facebook: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    id: u64,
    username: String,
    slug: String,
    url: String,
    avatar: Avatar,
    about: Option<String>,
    #[serde(rename = "totalAmountRaised")]
    total_amount_raised: f64,
    social: Social,
}

#[derive(Deserialize, Debug)]
pub struct UserSummary {
    id: u64,
    username: String,
    slug: String,
    url: String,
    avatar: Avatar,
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
