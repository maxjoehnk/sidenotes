use super::models::*;
use serde::Serialize;

#[derive(Clone)]
pub struct JiraApi {
    pub(super) url: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct SearchQuery<'a> {
    jql: &'a str,
}

impl JiraApi {
    pub fn new(url: String, username: String, password: String) -> Self {
        Self {
            url,
            username,
            password,
        }
    }

    pub async fn search(&self, query: &str) -> anyhow::Result<Vec<Issue>> {
        let query = SearchQuery { jql: query };
        let mut res = surf::get(format!("{}/rest/api/2/search", &self.url))
            .content_type("application/json")
            .header("Authorization", self.auth_header())
            .query(&query)
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
            anyhow::bail!("Jira api failure");
        }
        anyhow::ensure!(
            res.status().is_success(),
            "Jira api returned non success status code"
        );

        let res: SearchResponse = res
            .body_json()
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;
        tracing::trace!("{:?}", res);

        Ok(res.issues)
    }

    fn auth_header(&self) -> String {
        let unencoded = format!("{}:{}", self.username, self.password);
        let encoded = base64::encode(&unencoded);

        format!("Basic {}", encoded)
    }
}
