use std::sync::Arc;

use serde::{de::DeserializeOwned, Deserialize};

use crate::client::{ClientInner, ResponseMeta};

#[derive(Deserialize, Debug)]
pub(crate) struct PaginationLinks {
    prev: Option<String>,
    next: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PaginationResponse<T> {
    meta: ResponseMeta,
    data: Vec<T>,
    links: PaginationLinks,
}

pub struct Paginator<T> {
    pub data: Vec<T>,
    links: PaginationLinks,
    client: Arc<ClientInner>,
}

impl<T: DeserializeOwned> Paginator<T> {
    pub(crate) async fn get(
        client: Arc<ClientInner>,
        endpoint: &str,
    ) -> crate::Result<Paginator<T>> {
        let response = client.get(endpoint, None).await?;
        let response = response.json::<PaginationResponse<T>>().await?;
        Ok(Paginator {
            client: client.clone(),
            data: response.data,
            links: response.links,
        })
    }

    pub async fn next(&self) -> crate::Result<Option<Paginator<T>>> {
        match &self.links.next {
            Some(next) => {
                if next.is_empty() {
                    return Ok(None);
                }
                Ok(Some(Paginator::get(self.client.clone(), &next).await?))
            }
            None => Ok(None),
        }
    }

    pub async fn prev(&self) -> crate::Result<Option<Paginator<T>>> {
        match &self.links.prev {
            Some(prev) => {
                if prev.is_empty() {
                    return Ok(None);
                }
                Ok(Some(Paginator::get(self.client.clone(), &prev).await?))
            }
            None => Ok(None),
        }
    }
}
