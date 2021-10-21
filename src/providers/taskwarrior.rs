use serde::Deserialize;
use super::Provider;
use futures::future::BoxFuture;
use futures::FutureExt;
use crate::models::Todo;
use druid::im::Vector;
use task_hookrs::tw::query;

#[derive(Debug, Clone, Deserialize)]
pub struct TaskwarriorConfig {
    name: Option<String>,
    query: String
}

pub struct TaskwarriorProvider {
    name: Option<String>,
    query: String
}

impl TaskwarriorProvider {
    pub fn new(config: TaskwarriorConfig) -> anyhow::Result<Self> {
        Ok(Self {
            name: config.name,
            query: config.query
        })
    }

    async fn fetch_todos(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching TaskWarrior Tasks...");
        let mut todos: Vector<Todo> = Vector::new();
        if let Ok(tasks) = query(&self.query) {
            for task in tasks {
                todos.push_back(Todo{
                    title: task.description().into(),
                    state: Some(task.status().into()),
                    author: None,
                    body: None,
                    link: None,
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
    fn name(&self) -> String {
        self.name.clone()
            .unwrap_or_else(|| "TaskWarrior".into())
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_todos().boxed()
    }
}
