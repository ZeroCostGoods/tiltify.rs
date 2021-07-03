use std::sync::Arc;

use serde_json::Value;

use crate::client::ClientInner;

pub struct UserBuilder {
    user_id: String,
    client: Arc<ClientInner>,
}

impl UserBuilder {
    pub(crate) fn new(client: Arc<ClientInner>, user_id: String) -> Self {
        Self { client, user_id }
    }

    pub async fn get(&self) -> crate::Result<Value> {
        let url = format!("users/{}", &self.user_id);
        Ok(self.client.get(&url).await?.json::<Value>().await?)
    }

    pub async fn campaigns(&self) -> crate::Result<Value> {
        let url = format!("users/{}/campaigns", &self.user_id);
        Ok(self.client.get(&url).await?.json::<Value>().await?)
    }
}
