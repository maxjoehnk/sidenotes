use async_compat::CompatExt;
use druid::im::Vector;
use futures::future::BoxFuture;
use futures::FutureExt;
use octorust::auth::Credentials;
use octorust::Client;
use octorust::types::{IssueSearchResultItem, IssuesListState, Order, PullRequestReviewData, PullRequestSimple, PullsListSort, SearchIssuesPullRequestsSort, SimpleUser};
use serde::Deserialize;

use crate::models::Todo;
use crate::rich_text::Markdown;

use super::Provider;

#[derive(Debug, Clone, Deserialize)]
pub struct GithubConfig {
    #[serde(default)]
    name: Option<String>,
    token: String,
    #[serde(default)]
    repos: Vec<String>,
    #[serde(default)]
    query: Option<String>,
}

pub struct GithubProvider {
    name: Option<String>,
    client: Client,
    repos: Vec<(String, String)>,
    query: Option<String>,
}

impl GithubProvider {
    pub fn new(config: GithubConfig) -> anyhow::Result<Self> {
        let client = Client::new("sidenotes", Credentials::Token(config.token))?;

        Ok(Self {
            name: config.name,
            client,
            repos: config
                .repos
                .into_iter()
                .map(|repo| {
                    let parts = repo.split('/').collect::<Vec<_>>();

                    (parts[0].into(), parts[1].into())
                })
                .collect(),
            query: config.query,
        })
    }

    async fn fetch_todos(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Github Todos...");
        let (lhs, rhs): (Vector<Todo>, Vector<Todo>) = futures::future::try_join(
            self.fetch_repo_pull_requests(),
            self.search_issues_and_prs()
        ).await?;
        let mut todos = lhs;
        todos.append(rhs);

        tracing::info!("Fetched {} Github Todos", todos.len());

        Ok(todos)
    }

    async fn fetch_repo_pull_requests(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Github PRs...");
        let mut todos = Vector::new();
        for (owner, repo) in self.repos.iter() {
            let pull_requests = self
                .client
                .pulls()
                .list_all(
                    owner,
                    repo,
                    IssuesListState::Open,
                    "",
                    "",
                    PullsListSort::Created,
                    Order::default(),
                )
                .await?;
            for pr in pull_requests {
                let reviews = self
                    .client
                    .pulls()
                    .list_all_reviews(owner, repo, pr.number)
                    .await?;
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

    async fn search_issues_and_prs(&self) -> anyhow::Result<Vector<Todo>> {
        if let Some(query) = self.query.as_ref() {
            tracing::info!("Searching for Github Issues and PRs...");
            let mut todos = Vector::new();
            let res = self.client.search()
                .issues_and_pull_requests(query, SearchIssuesPullRequestsSort::Created, Order::Asc, 20, 0)
                .await?;
            for item in res.items {
                let state = if item.pull_request.is_some() {
                    if let Some(repository_url) = item.repository_url.as_ref().and_then(|url| url.path_segments()) {
                        let paths = repository_url.skip(1).collect::<Vec<_>>();

                        let reviews = self
                            .client
                            .pulls()
                            .list_all_reviews(paths[0], paths[1], item.number)
                            .await?;
                        Some(Self::get_pr_state(&item, &reviews))
                    }else {
                        None
                    }
                }else {
                    Some(item.state)
                };
                todos.push_back(Todo {
                    title: format!("#{} - {}", item.number, item.title),
                    state,
                    author: item.user.map(|user| user.name),
                    body: Some(Markdown(item.body).into()),
                    link: item.html_url.map(|url| url.to_string()),
                });
            }

            tracing::info!("Found {} Github Issues and PRs", todos.len());

            Ok(todos)
        }else {
            Ok(Default::default())
        }
    }

    fn get_pr_state(pr: &impl GithubIssueOrPullRequest, reviews: &[PullRequestReviewData]) -> String {
        if pr.is_draft() {
            return "Draft".into();
        }
        if reviews
            .iter()
            .any(|review| review.state == "CHANGES_REQUESTED")
        {
            return "Changes requested".into();
        }
        if !reviews.is_empty()
            && reviews
                .iter()
                .all(|review| review.state == "COMMENTED" || review.state == "APPROVED")
        {
            return "Approved".into();
        }

        "Open".into()
    }
}

impl Provider for GithubProvider {
    fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| "Github".into())
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_todos().compat().boxed()
    }
}

trait GithubIssueOrPullRequest {
    fn is_draft(&self) -> bool;
    fn user(&self) -> Option<&SimpleUser>;
}

impl GithubIssueOrPullRequest for PullRequestSimple {
    fn is_draft(&self) -> bool {
        self.draft
    }

    fn user(&self) -> Option<&SimpleUser> {
        self.user.as_ref()
    }
}

impl GithubIssueOrPullRequest for IssueSearchResultItem {
    fn is_draft(&self) -> bool {
        self.draft
    }

    fn user(&self) -> Option<&SimpleUser> {
        self.user.as_ref()
    }
}