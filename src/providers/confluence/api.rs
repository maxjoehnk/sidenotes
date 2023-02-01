use super::models::*;
use base64::prelude::*;

#[derive(Clone)]
pub struct ConfluenceApi {
    pub(super) url: String,
    pub(super) username: String,
    pub(super) password: String,
}

impl ConfluenceApi {
    pub fn new(url: String, username: String, password: String) -> Self {
        Self {
            url,
            username,
            password,
        }
    }

    pub async fn my_tasks(&self) -> anyhow::Result<Vec<Task>> {
        let mut res = surf::get(format!("{}/rest/mywork/1/task", &self.url))
            .content_type("application/json")
            .header("Authorization", self.auth_header())
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
            anyhow::bail!("Confluence api failure");
        }
        anyhow::ensure!(
            res.status().is_success(),
            "Confluence api returned non success status code"
        );

        let res: Vec<Task> = res
            .body_json()
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;
        tracing::trace!("{:?}", res);

        Ok(res)
    }

    fn auth_header(&self) -> String {
        let unencoded = format!("{}:{}", self.username, self.password);
        let encoded = BASE64_STANDARD.encode(unencoded);

        format!("Basic {}", encoded)
    }
}
