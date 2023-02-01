use crate::providers::ProviderConfig;
use async_compat::CompatExt;
use druid::{Data, Lens};
use futures::future::BoxFuture;
use futures::FutureExt;
use im::{vector, Vector};
use octorust::auth::Credentials;
use octorust::types::{
    IssueSearchResultItem, IssuesListState, Order, PullRequestReviewData, PullRequestSimple,
    PullsListSort, SearchIssuesPullRequestsSort, SimpleUser,
};
use octorust::Client;
use serde::{Deserialize, Serialize};

use crate::models::{Todo, TodoAction, TodoId};
use crate::providers::ProviderId;
use crate::rich_text::Markdown;

use super::Provider;

const MARK_NOTIFICATION_AS_READ: &str = "GITHUB_MARK_AS_READ";

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Data, Lens)]
pub struct GithubConfig {
    token: String,
    #[serde(default)]
    repos: Vector<String>,
    #[serde(default)]
    query: Option<String>,
    #[serde(default)]
    notifications: bool,
}

#[derive(Clone)]
pub struct GithubProvider {
    id: ProviderId,
    client: Client,
    token: String,
    repos: Vec<(String, String)>,
    query: Option<String>,
    notifications: bool,
}

impl GithubProvider {
    pub fn new(id: ProviderId, config: GithubConfig) -> anyhow::Result<Self> {
        let token = config.token.clone();
        let client = Client::new("sidenotes", Credentials::Token(config.token))?;

        Ok(Self {
            id,
            client,
            token,
            repos: config
                .repos
                .into_iter()
                .map(|repo| {
                    let parts = repo.split('/').collect::<Vec<_>>();

                    (parts[0].into(), parts[1].into())
                })
                .collect(),
            query: config.query,
            notifications: config.notifications,
        })
    }

    async fn fetch_todos(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Github Todos...");
        let (todos1, todos2, todos3) = futures::future::try_join3(
            self.fetch_repo_pull_requests(),
            self.search_issues_and_prs(),
            self.get_notifications(),
        )
        .await?;
        let mut todos = todos1;
        todos.append(todos2);
        todos.append(todos3);

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
                    provider: self.id,
                    id: (pr.id as u64).into(),
                    title: format!("#{} - {}", pr.number, pr.title),
                    state: Some(Self::get_pr_state(&pr, &reviews)),
                    tags: pr.labels.into_iter().map(|label| label.name).collect(),
                    author: pr.user.map(|user| user.name),
                    body: Some(Markdown(pr.body).into()),
                    link: pr.html_url.map(|url| url.to_string()),
                    actions: Default::default(),
                    comments: Default::default(),
                    due_date: None,
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
            let res = self
                .client
                .search()
                .issues_and_pull_requests(
                    query,
                    SearchIssuesPullRequestsSort::Created,
                    Order::Asc,
                    20,
                    0,
                )
                .await?;
            for item in res.items {
                let state = if item.pull_request.is_some() {
                    if let Some(repository_url) = item
                        .repository_url
                        .as_ref()
                        .and_then(|url| url.path_segments())
                    {
                        let paths = repository_url.skip(1).collect::<Vec<_>>();

                        let reviews = self
                            .client
                            .pulls()
                            .list_all_reviews(paths[0], paths[1], item.number)
                            .await?;
                        Some(Self::get_pr_state(&item, &reviews))
                    } else {
                        None
                    }
                } else {
                    Some(item.state)
                };
                todos.push_back(Todo {
                    provider: self.id,
                    id: (item.id as u64).into(),
                    title: format!("#{} - {}", item.number, item.title),
                    state,
                    tags: item.labels.into_iter().map(|label| label.name).collect(),
                    author: item.user.map(|user| user.name),
                    body: Some(Markdown(item.body).into()),
                    link: item.html_url.map(|url| url.to_string()),
                    actions: Default::default(),
                    comments: Default::default(),
                    due_date: None,
                });
            }

            tracing::info!("Found {} Github Issues and PRs", todos.len());

            Ok(todos)
        } else {
            Ok(Default::default())
        }
    }

    async fn get_notifications(&self) -> anyhow::Result<Vector<Todo>> {
        if !self.notifications {
            return Ok(Vector::new());
        }
        let threads = self
            .client
            .activity()
            .list_all_notifications_for_authenticated_user(false, false, None, None)
            .await?;

        let todos = threads
            .into_iter()
            .map(|thread| Todo {
                id: thread.id.into(),
                title: thread.subject.title,
                link: thread.subject.url.into(),
                author: None,
                tags: vector![thread.repository.full_name],
                body: None,
                provider: self.id,
                state: thread.reason.into(),
                comments: Default::default(),
                actions: vector![TodoAction {
                    id: MARK_NOTIFICATION_AS_READ,
                    label: "Mark as read"
                }],
                due_date: None,
            })
            .collect();

        Ok(todos)
    }

    async fn mark_thread_as_read(&self, thread_id: String) -> anyhow::Result<()> {
        let thread_id = thread_id.parse::<i64>()?;
        self.client
            .activity()
            .mark_thread_as_read(thread_id)
            .await?;

        Ok(())
    }

    fn get_pr_state(
        pr: &impl GithubIssueOrPullRequest,
        reviews: &[PullRequestReviewData],
    ) -> String {
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
    fn to_config(&self) -> ProviderConfig {
        GithubConfig {
            query: self.query.clone(),
            token: self.token.clone(),
            repos: self
                .repos
                .iter()
                .map(|(owner, repo)| format!("{owner}/{repo}"))
                .collect(),
            notifications: self.notifications,
        }
        .into()
    }

    fn name(&self) -> &'static str {
        "Github"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_todos().compat().boxed()
    }

    fn run_action(&self, todo: TodoId, action: TodoAction) -> BoxFuture<anyhow::Result<()>> {
        match (todo, action.id) {
            (TodoId::String(thread), MARK_NOTIFICATION_AS_READ) => {
                self.mark_thread_as_read(thread).compat().boxed()
            }
            _ => unimplemented!(),
        }
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
