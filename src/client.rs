use std::sync::Arc;

use reqwest::{redirect::Policy, Response, StatusCode};

use crate::{campaign::CampaignBuilder, user::UserBuilder};
use anyhow::anyhow;
use serde_json::Value;

static API_ROOT: &'static str = "https://tiltify.com/api/v3/";

pub(crate) struct ClientInner {
    access_token: String,
    client: reqwest::Client,
}

impl ClientInner {
    fn new<S>(access_token: S) -> crate::Result<Self>
    where
        S: Into<String>,
    {
        let client = reqwest::Client::builder()
            .redirect(Policy::none())
            .build()?;

        Ok(Self {
            access_token: access_token.into(),
            client: client,
        })
    }

    pub(crate) async fn get(&self, endpoint: &str) -> crate::Result<Response> {
        let url = format!("{}{}", API_ROOT, endpoint);
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", &self.access_token))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response),
            // TODO: Handle errors much better...
            _ => Err(anyhow!(response.json::<Value>().await?.to_string())),
        }
    }
}

pub struct TiltifyClient {
    inner_client: Arc<ClientInner>,
}

impl TiltifyClient {
    pub fn new<S: Into<String>>(access_token: S) -> crate::Result<Self> {
        Ok(Self {
            inner_client: Arc::new(ClientInner::new(access_token)?),
        })
    }

    pub fn user<S: Into<String>>(&self, user_id: S) -> UserBuilder {
        UserBuilder::new(self.inner_client.clone(), user_id.into())
    }

    pub fn campaign<S: Into<String>>(&self, campaign_id: S) -> CampaignBuilder {
        CampaignBuilder::new(self.inner_client.clone(), campaign_id.into())
    }
}
