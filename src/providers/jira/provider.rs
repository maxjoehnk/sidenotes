use super::api;
use super::models;
use crate::models::Todo;
use crate::providers::Provider;
use druid::im::Vector;
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct JiraConfig {
    url: String,
    username: String,
    password: String,
    jql: String,
}

pub struct JiraProvider {
    api: api::JiraApi,
    jql: String,
}

impl JiraProvider {
    pub fn new(config: JiraConfig) -> anyhow::Result<Self> {
        Ok(Self {
            api: api::JiraApi::new(config.url, config.username, config.password),
            jql: config.jql,
        })
    }

    async fn fetch_issues(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Jira issues...");
        let issues = self.api.search(&self.jql).await?;
        let todos: Vector<_> = issues
            .into_iter()
            .map(|issue| self.issue_to_todo(issue))
            .collect();
        tracing::info!("Fetched {} Jira notes", todos.len());

        Ok(todos)
    }

    fn issue_to_todo(&self, issue: models::Issue) -> Todo {
        Todo {
            title: format!("{} - {}", issue.key, issue.fields.summary),
            state: Some(issue.fields.status.name),
            body: issue.fields.description.map(|desc| desc.into()),
            author: None,
            link: Some(format!("{}/browse/{}", self.api.url, issue.key)),
        }
    }
}

impl Provider for JiraProvider {
    fn name(&self) -> &'static str {
        "Jira"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_issues().boxed()
    }
}
