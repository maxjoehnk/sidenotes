use crate::providers::Provider;
use futures::future::BoxFuture;
use druid::im::Vector;
use crate::models::Todo;
use serde::Deserialize;
use futures::FutureExt;

mod api;
mod models;

#[derive(Debug, Clone, Deserialize)]
pub struct JiraConfig {
    #[serde(default)]
    name: Option<String>,
    url: String,
    username: String,
    password: String,
    jql: String,
}

pub struct JiraProvider {
    name: Option<String>,
    api: api::JiraApi,
    jql: String,
}

impl JiraProvider {
    pub fn new(config: JiraConfig) -> anyhow::Result<Self> {
        Ok(Self {
            name: config.name,
            api: api::JiraApi::new(config.url, config.username, config.password),
            jql: config.jql,
        })
    }

    async fn fetch_issues(&self) -> anyhow::Result<Vector<Todo>> {
        let issues = self.api.search(&self.jql).await?;
        let todos = issues
            .into_iter()
            .map(Todo::from)
            .collect();

        Ok(todos)
    }
}

impl Provider for JiraProvider {
    fn name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| "Jira".into())
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_issues().boxed()
    }
}

impl From<models::Issue> for Todo {
    fn from(issue: models::Issue) -> Self {
        Self {
            title: format!("{} ({})", issue.fields.summary, issue.key),
            completed: false,
            state: Some(issue.fields.status.name),
        }
    }
}
