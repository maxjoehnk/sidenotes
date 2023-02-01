use super::models::*;
use base64::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Clone)]
pub struct JiraApi {
    pub(super) url: String,
    pub(super) username: String,
    pub(super) password: String,
}

#[derive(Serialize)]
struct SearchQuery<'a> {
    jql: &'a str,
}

#[derive(Serialize)]
struct GetIssueQuery<'a> {
    fields: Option<&'a str>,
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
        let res: SearchResponse = self.get("rest/api/2/search", &query).await?;

        Ok(res.issues)
    }

    pub async fn comments(&self, id: &str) -> anyhow::Result<Vec<Comment>> {
        let query = GetIssueQuery {
            fields: Some("comment"),
        };
        let res: IssueWithComments = self
            .get(&format!("rest/agile/1.0/issue/{id}"), &query)
            .await?;

        Ok(res.fields.comment.comments)
    }

    async fn get<T: DeserializeOwned + Debug>(
        &self,
        url: &str,
        query: &impl Serialize,
    ) -> anyhow::Result<T> {
        let mut res = surf::get(format!("{}/{}", &self.url, url))
            .content_type("application/json")
            .header("Authorization", self.auth_header())
            .query(query)
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

        let res: T = res
            .body_json()
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;
        tracing::trace!("{:?}", res);

        Ok(res)
    }

    fn auth_header(&self) -> String {
        let unencoded = format!("{}:{}", self.username, self.password);
        let encoded = BASE64_STANDARD.encode(unencoded);

        format!("Basic {encoded}")
    }
}
