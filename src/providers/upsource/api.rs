use super::models::*;
use serde::{Deserialize, Serialize};

pub struct UpsourceApi {
    url: String,
    token: String,
}

impl UpsourceApi {
    pub fn new(url: String, token: String) -> Self {
        Self { url, token }
    }

    pub async fn get_reviews(&self, query: &str) -> anyhow::Result<Vec<ReviewDescriptor>> {
        let body = ReviewsRequest { limit: 30, query };
        let mut res = surf::post(format!("{}/~rpc/getReviews", &self.url))
            .content_type("application/json")
            .header("Authorization", format!("Bearer {}", &self.token))
            .body_json(&body)
            .unwrap()
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;

        #[cfg(debug_assertions)]
        if !res.status().is_success() {
            eprintln!("{:?}", res.status());
            eprintln!(
                "{:?}",
                res.body_string()
                    .await
                    .map_err(|err| anyhow::anyhow!("{:?}", err))?
            );
            anyhow::bail!("Upsource api failure");
        }
        anyhow::ensure!(
            res.status().is_success(),
            "Upsource api returned non success status code"
        );

        let res: ApiResult<ReviewList> = res
            .body_json()
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;
        tracing::trace!("{:?}", res);

        Ok(res.result.reviews)
    }
}

#[derive(Serialize)]
struct ReviewsRequest<'a> {
    limit: usize,
    query: &'a str,
}

#[derive(Debug, Deserialize)]
struct ApiResult<T> {
    pub result: T,
}
