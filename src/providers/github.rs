use octorust::Client;
use octorust::auth::Credentials;
use serde::Deserialize;
use crate::models::Todo;
use octorust::types::{IssuesListState, PullsListSort, Order};
use druid::im::Vector;
use super::Provider;
use futures::future::BoxFuture;
use futures::{FutureExt};
use async_compat::CompatExt;

#[derive(Debug, Clone, Deserialize)]
pub struct GithubConfig {
    #[serde(default)]
    name: Option<String>,
    token: String,
    repos: Vec<String>
}

pub struct GithubProvider {
    name: Option<String>,
    client: Client,
    repos: Vec<(String, String)>
}

impl GithubProvider {
    pub fn new(config: GithubConfig) -> anyhow::Result<Self> {
        let client = Client::new("sidenotes", Credentials::Token(config.token))?;

        Ok(Self {
            name: config.name,
            client,
            repos: config.repos.into_iter()
                .map(|repo| {
                    let parts = repo.split("/").collect::<Vec<_>>();

                    (parts[0].into(), parts[1].into())
                })
                .collect()
        })
    }

    async fn fetch_todos(&self) -> anyhow::Result<Vector<Todo>> {
        let mut todos = Vector::new();
        for (owner, repo) in self.repos.iter() {
            let pull_requests = self.client.pulls().list_all(owner, repo, IssuesListState::Open, "", "", PullsListSort::Created, Order::default()).await?;
            for pr in pull_requests {
                todos.push_back(Todo {
                    title: pr.title,
                    completed: false,
                    state: Some(pr.state),
                })
            }
        }

        Ok(todos)
    }
}

impl Provider for GithubProvider {
    fn name(&self) -> String {
        self.name.clone()
            .unwrap_or_else(|| "Github".into())
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_todos().compat().boxed()
    }
}
