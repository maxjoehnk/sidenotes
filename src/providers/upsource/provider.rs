use super::{api, models};
use serde::Deserialize;
use druid::im::Vector;
use crate::models::Todo;
use crate::providers::upsource::models::{ReviewDescriptor, ReviewState};
use crate::rich_text::Markdown;
use futures::future::{FutureExt, BoxFuture};
use crate::providers::Provider;

#[derive(Debug, Clone, Deserialize)]
pub struct UpsourceConfig {
    #[serde(default)]
    name: Option<String>,
    url: String,
    token: String,
    #[serde(default)]
    query: UpsourceQuery,
}

#[derive(Debug, Clone, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
struct UpsourceQuery(String);

impl Default for UpsourceQuery {
    fn default() -> Self {
        Self("state: open".into())
    }
}

pub struct UpsourceProvider {
    base_url: String,
    name: Option<String>,
    api: api::UpsourceApi,
    query: String,
}

impl UpsourceProvider {
    pub fn new(config: UpsourceConfig) -> anyhow::Result<Self> {
        Ok(Self {
            base_url: config.url.clone(),
            name: config.name,
            api: api::UpsourceApi::new(config.url, config.token),
            query: config.query.0,
        })
    }

    async fn fetch_reviews(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Upsource Reviews...");
        let issues = self.api.get_reviews(&self.query).await?;
        let todos: Vector<_> = issues
            .into_iter()
            .map(|review| Todo::from((self.base_url.as_str(), review)))
            .collect();
        tracing::info!("Fetched {} Upsource notes", todos.len());

        Ok(todos)
    }
}

impl Provider for UpsourceProvider {
    fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| "Upsource".into())
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_reviews().boxed()
    }
}

impl From<(&str, models::ReviewDescriptor)> for Todo {
    fn from((url, review): (&str, ReviewDescriptor)) -> Self {
        Self {
            title: format!("{} - {}", review.review_id.review_id, review.title),
            state: Some(review.state()),
            body: review.description.map(|desc| Markdown(desc).into()),
            author: None,
            link: Some(format!("{}/{}/review/{}", url, review.review_id.project_id, review.review_id.review_id)),
        }
    }
}

impl models::ReviewDescriptor {
    fn state(&self) -> String {
        if self.completion_rate.has_concern {
            "Raised concerns".into()
        }else if self.state == ReviewState::Open && self.completion_rate.reviewers_count == 0 {
            "Open".into()
        }else if self.completion_rate.completed_count == self.completion_rate.reviewers_count {
            "Approved".into()
        }else {
            "In Progress".into()
        }
    }
}