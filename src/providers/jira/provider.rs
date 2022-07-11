use super::api;
use super::models;
use crate::models::{Todo, TodoComment, TodoId};
use crate::providers::jira::models::Comment;
use crate::providers::{Provider, ProviderConfig, ProviderId};
use druid::im::Vector;
use druid::{Data, Lens};
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Data, Lens)]
pub struct JiraConfig {
    url: String,
    username: String,
    password: String,
    jql: String,
}

#[derive(Clone)]
pub struct JiraProvider {
    id: ProviderId,
    api: api::JiraApi,
    jql: String,
}

impl JiraProvider {
    pub fn new(id: ProviderId, config: JiraConfig) -> anyhow::Result<Self> {
        Ok(Self {
            id,
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

    async fn fetch_issue_comments(&self, cursor: TodoId) -> anyhow::Result<Vector<TodoComment>> {
        tracing::info!("Fetching Jira comments for {:?}...", cursor);
        if let TodoId::String(jira_id) = cursor {
            let comments = self.api.comments(&jira_id).await?;
            let comments = comments.into_iter().map(TodoComment::from).collect();

            Ok(comments)
        } else {
            Ok(Vector::new())
        }
    }

    fn issue_to_todo(&self, issue: models::Issue) -> Todo {
        Todo {
            provider: self.id,
            id: issue.id.into(),
            title: format!("{} - {}", issue.key, issue.fields.summary),
            state: Some(issue.fields.status.name),
            tags: issue
                .fields
                .components
                .into_iter()
                .map(|component| component.name)
                .collect(),
            body: issue.fields.description.map(|desc| desc.into()),
            author: None,
            link: Some(format!("{}/browse/{}", self.api.url, issue.key)),
            actions: Default::default(),
            comments: Default::default(),
            due_date: None,
        }
    }
}

impl Provider for JiraProvider {
    fn fetch_comments(&self, id: TodoId) -> BoxFuture<anyhow::Result<Vector<TodoComment>>> {
        self.fetch_issue_comments(id).boxed()
    }

    fn to_config(&self) -> ProviderConfig {
        JiraConfig {
            url: self.api.url.clone(),
            username: self.api.username.clone(),
            password: self.api.password.clone(),
            jql: self.jql.clone(),
        }
        .into()
    }

    fn name(&self) -> &'static str {
        "Jira"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_issues().boxed()
    }
}

impl From<Comment> for TodoComment {
    fn from(comment: Comment) -> Self {
        Self {
            text: comment.body.into(),
            author: Some(comment.author.display_name),
        }
    }
}
