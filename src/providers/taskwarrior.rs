use super::Provider;
use crate::models::Todo;
use crate::providers::ProviderConfig;
use crate::providers::ProviderId;
use druid::im::Vector;
use druid::{Data, Lens};
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use task_hookrs::tw::query;

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Data, Lens)]
pub struct TaskwarriorConfig {
    query: String,
}

#[derive(Clone)]
pub struct TaskwarriorProvider {
    id: ProviderId,
    query: String,
}

impl TaskwarriorProvider {
    pub fn new(id: ProviderId, config: TaskwarriorConfig) -> anyhow::Result<Self> {
        Ok(Self {
            id,
            query: config.query,
        })
    }

    async fn fetch_todos(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching TaskWarrior Tasks...");
        let mut todos: Vector<Todo> = Vector::new();
        if let Ok(tasks) = query(&self.query) {
            for task in tasks {
                let project = match task.project() {
                    Some(text) => text.as_str(),
                    None => "",
                };
                todos.push_back(Todo {
                    provider: self.id,
                    id: task.id().unwrap_or_default().into(),
                    title: format!("[{}] {}", &String::from(project), task.description()),
                    state: Some(task.status().to_string()),
                    tags: task
                        .tags()
                        .map(|tags| tags.iter().cloned().collect())
                        .unwrap_or_default(),
                    author: None,
                    body: None,
                    link: None,
                    actions: Default::default(),
                    comments: Default::default(),
                    due_date: None,
                })
            }
            tracing::info!("Fetched {} TaskWarrior tasks", todos.len());
        } else {
            tracing::warn!("Tasks cannot be fetched")
        }

        Ok(todos)
    }
}

impl Provider for TaskwarriorProvider {
    fn to_config(&self) -> ProviderConfig {
        TaskwarriorConfig {
            query: self.query.clone(),
        }
        .into()
    }

    fn name(&self) -> &'static str {
        "TaskWarrior"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_todos().boxed()
    }
}
