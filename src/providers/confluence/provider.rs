use super::api;
use super::models;
use crate::models::Todo;
use crate::providers::{Provider, ProviderConfig, ProviderId};
use druid::im::Vector;
use druid::{Data, Lens};
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Data, Lens)]
pub struct ConfluenceConfig {
    url: String,
    username: String,
    password: String,
}

#[derive(Clone)]
pub struct ConfluenceProvider {
    id: ProviderId,
    api: api::ConfluenceApi,
}

impl ConfluenceProvider {
    pub fn new(id: ProviderId, config: ConfluenceConfig) -> anyhow::Result<Self> {
        Ok(Self {
            id,
            api: api::ConfluenceApi::new(config.url, config.username, config.password),
        })
    }

    async fn fetch_tasks(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Confluence Inline Tasks...");
        let issues = self.api.my_tasks().await?;
        let todos: Vector<_> = issues
            .into_iter()
            .filter(|task| task.status == models::Status::Todo)
            .map(|issue| self.task_to_todo(issue))
            .collect();
        tracing::info!("Fetched {} Confluence inline tasks", todos.len());

        Ok(todos)
    }

    fn task_to_todo(&self, task: models::Task) -> Todo {
        Todo {
            provider: self.id,
            id: task.id.into(),
            title: task.item.title,
            state: None,
            tags: Default::default(),
            body: None,
            author: None,
            link: Some(format!("{}{}", self.api.url, task.item.url)),
            actions: Default::default(),
            comments: Default::default(),
            due_date: None,
        }
    }
}

impl Provider for ConfluenceProvider {
    fn to_config(&self) -> ProviderConfig {
        ConfluenceConfig {
            url: self.api.url.clone(),
            username: self.api.username.clone(),
            password: self.api.password.clone(),
        }
        .into()
    }

    fn name(&self) -> &'static str {
        "Confluence"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_tasks().boxed()
    }
}
