use super::{api, models};
use crate::models::Todo;
use crate::providers::upsource::models::{ReviewDescriptor, ReviewState};
use crate::providers::{Provider, ProviderConfig, ProviderId};
use crate::rich_text::Markdown;
use druid::im::Vector;
use druid::{Data, Lens};
use futures::future::{BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, Data, Lens)]
pub struct UpsourceConfig {
    url: String,
    token: String,
    #[serde(default)]
    #[lens(ignore)]
    query: UpsourceQuery,
}

impl UpsourceConfig {
    #[allow(non_upper_case_globals)]
    pub const query: UpsourceQueryLens = UpsourceQueryLens;
}

#[derive(Clone, Copy)]
pub struct UpsourceQueryLens;

impl Lens<UpsourceConfig, String> for UpsourceQueryLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &UpsourceConfig, f: F) -> V {
        f(&data.query.0)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut UpsourceConfig, f: F) -> V {
        f(&mut data.query.0)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Data)]
#[repr(transparent)]
#[serde(transparent)]
struct UpsourceQuery(String);

impl Default for UpsourceQuery {
    fn default() -> Self {
        Self("state: open".into())
    }
}

#[derive(Clone)]
pub struct UpsourceProvider {
    id: ProviderId,
    base_url: String,
    api: api::UpsourceApi,
    query: String,
}

impl UpsourceProvider {
    pub fn new(id: ProviderId, config: UpsourceConfig) -> anyhow::Result<Self> {
        Ok(Self {
            id,
            base_url: config.url.clone(),
            api: api::UpsourceApi::new(config.url, config.token),
            query: config.query.0,
        })
    }

    async fn fetch_reviews(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Upsource Reviews...");
        let issues = self.api.get_reviews(&self.query).await?;
        let todos: Vector<_> = issues
            .into_iter()
            .map(|review| Todo::from((self.id, self.base_url.as_str(), review)))
            .collect();
        tracing::info!("Fetched {} Upsource notes", todos.len());

        Ok(todos)
    }
}

impl Provider for UpsourceProvider {
    fn to_config(&self) -> ProviderConfig {
        UpsourceConfig {
            query: UpsourceQuery(self.query.clone()),
            url: self.api.url.clone(),
            token: self.api.token.clone(),
        }
        .into()
    }

    fn name(&self) -> &'static str {
        "Upsource"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_reviews().boxed()
    }
}

impl From<(ProviderId, &str, models::ReviewDescriptor)> for Todo {
    fn from((provider_id, url, review): (ProviderId, &str, ReviewDescriptor)) -> Self {
        Self {
            provider: provider_id,
            title: format!("{} - {}", review.review_id.review_id, review.title),
            link: Some(format!(
                "{}/{}/review/{}",
                url, review.review_id.project_id, review.review_id.review_id
            )),
            state: Some(review.state()),
            body: review.description.map(|desc| Markdown(desc).into()),
            tags: review.labels.into_iter().map(|label| label.name).collect(),
            author: review.created_by,
            id: review.review_id.review_id.into(), // TODO: this should be a combination of review and project id
            actions: Default::default(),
            comments: Default::default(),
        }
    }
}

impl models::ReviewDescriptor {
    fn state(&self) -> String {
        if self.completion_rate.has_concern {
            "Raised concerns".into()
        } else if self.state == ReviewState::Open && self.completion_rate.reviewers_count == 0 {
            "Open".into()
        } else if self.completion_rate.completed_count == self.completion_rate.reviewers_count {
            "Approved".into()
        } else {
            "In Progress".into()
        }
    }
}
