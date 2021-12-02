use super::Provider;
use crate::models::Todo;
use druid::im::Vector;
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::Deserialize;
use task_hookrs::tw::query;

#[derive(Debug, Clone, Deserialize)]
pub struct TaskwarriorConfig {
    query: String,
}

pub struct TaskwarriorProvider {
    query: String,
}

impl TaskwarriorProvider {
    pub fn new(config: TaskwarriorConfig) -> anyhow::Result<Self> {
        Ok(Self {
            query: config.query,
        })
    }

    async fn fetch_todos(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching TaskWarrior Tasks...");
        let mut todos: Vector<Todo> = Vector::new();
        if let Ok(tasks) = query(&self.query) {
            for task in tasks {
                todos.push_back(Todo {
                    title: task.description().into(),
                    state: Some(task.status().to_string()),
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
    fn name(&self) -> &'static str {
        "TaskWarrior"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_todos().boxed()
    }
}
