use octorust::Client;
use octorust::auth::Credentials;
use serde::Deserialize;
use crate::models::Todo;
use octorust::types::{IssuesListState, PullsListSort, Order, PullRequestSimple, PullRequestReviewData};
use druid::im::Vector;
use super::Provider;
use futures::future::BoxFuture;
use futures::{FutureExt};
use async_compat::CompatExt;
use crate::rich_text::Markdown;

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
                    let parts = repo.split('/').collect::<Vec<_>>();

                    (parts[0].into(), parts[1].into())
                })
                .collect()
        })
    }

    async fn fetch_todos(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Github PRs...");
        let mut todos = Vector::new();
        for (owner, repo) in self.repos.iter() {
            let pull_requests = self.client.pulls().list_all(owner, repo, IssuesListState::Open, "", "", PullsListSort::Created, Order::default()).await?;
            for pr in pull_requests {
                let reviews = self.client.pulls().list_all_reviews(owner, repo, pr.number).await?;
                todos.push_back(Todo {
                    title: format!("#{} - {}", pr.number, pr.title),
                    state: Some(Self::get_pr_state(&pr, &reviews)),
                    author: pr.user.map(|user| user.name),
                    body: Some(Markdown(pr.body).into()),
                    link: pr.html_url.map(|url| url.to_string()),
                })
            }
        }
        tracing::info!("Fetched {} Github PRs", todos.len());

        Ok(todos)
    }

    fn get_pr_state(pr: &PullRequestSimple, reviews: &[PullRequestReviewData]) -> String {
        if pr.draft {
            return "Draft".into()
        }
        if reviews.iter().any(|review| review.state == "CHANGES_REQUESTED") {
            return "Changes requested".into()
        }
        if !reviews.is_empty() && reviews.iter().all(|review| review.state == "COMMENTED" || review.state == "APPROVED") {
            return "Approved".into()
        }

        "Open".into()
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
