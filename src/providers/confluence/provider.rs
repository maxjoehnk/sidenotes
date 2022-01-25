use super::api;
use super::models;
use crate::models::Todo;
use crate::providers::Provider;
use druid::im::Vector;
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfluenceConfig {
    url: String,
    username: String,
    password: String,
}

pub struct ConfluenceProvider {
    api: api::ConfluenceApi,
}

impl ConfluenceProvider {
    pub fn new(config: ConfluenceConfig) -> anyhow::Result<Self> {
        Ok(Self {
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
            title: task.item.title,
            state: None,
            tags: Default::default(),
            body: None,
            author: None,
            link: Some(format!("{}{}", self.api.url, task.item.url)),
        }
    }
}

impl Provider for ConfluenceProvider {
    fn name(&self) -> &'static str {
        "Confluence"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_tasks().boxed()
    }
}
