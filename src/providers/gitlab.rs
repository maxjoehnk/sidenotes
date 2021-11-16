use crate::models::Todo;
use crate::providers::Provider;
use crate::rich_text::Markdown;
use async_compat::CompatExt;
use druid::im::Vector;
use futures::future::BoxFuture;
use futures::FutureExt;
use gitlab::api::projects::{self, merge_requests};
use gitlab::{api, api::AsyncQuery, AsyncGitlab, GitlabBuilder, MergeRequest, Project};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GitlabConfig {
    #[serde(default)]
    name: Option<String>,
    url: String,
    token: String,
    #[serde(default)]
    repos: Option<Vec<String>>,
    #[serde(default)]
    show_drafts: bool,
}

pub struct GitlabProvider {
    name: Option<String>,
    client: AsyncGitlab,
    repos: Option<Vec<String>>,
    show_drafts: bool,
}

impl GitlabProvider {
    pub async fn new(config: GitlabConfig) -> anyhow::Result<Self> {
        let client = GitlabBuilder::new(config.url, config.token)
            .insecure()
            .build_async()
            .compat()
            .await?;

        Ok(Self {
            name: config.name,
            client,
            repos: config.repos,
            show_drafts: config.show_drafts,
        })
    }

    async fn fetch_merge_requests(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Gitlab MRs...");
        let mut todos = Vector::new();
        let repos = self.get_repos().await?;
        for repo in repos {
            tracing::debug!("Fetching MRs for {:?}", repo);
            let mut builder = merge_requests::MergeRequests::builder();
            builder
                .project(repo.as_str())
                .state(merge_requests::MergeRequestState::Opened);
            if !self.show_drafts {
                builder.wip(true);
            }
            let endpoint = builder.build().map_err(|err| anyhow::anyhow!("{}", err))?;
            let requests: Vec<MergeRequest> = endpoint.query_async(&self.client).await?;
            tracing::debug!("{:?}", requests);

            for request in requests {
                todos.push_back(request.into());
            }
        }
        tracing::info!("Fetched {} Gitlab MRs", todos.len());

        Ok(todos)
    }

    async fn get_repos(&self) -> anyhow::Result<Vec<String>> {
        if let Some(repos) = self.repos.as_ref() {
            Ok(repos.clone())
        } else {
            let endpoint = projects::Projects::builder()
                .membership(true)
                .with_merge_requests_enabled(true)
                .build()
                .map_err(|err| anyhow::anyhow!("{}", err))?;
            let projects: Vec<Project> = api::paged(endpoint, api::Pagination::All)
                .query_async(&self.client)
                .await?;
            tracing::trace!("{:?}", projects);

            Ok(projects
                .into_iter()
                .map(|project| project.path_with_namespace)
                .collect())
        }
    }
}

impl Provider for GitlabProvider {
    fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| "Gitlab".into())
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_merge_requests().compat().boxed()
    }
}

impl From<MergeRequest> for Todo {
    fn from(mr: MergeRequest) -> Self {
        Self {
            title: format!("#{} - {}", mr.iid, mr.title),
            state: Some(format!("{:?}", mr.state)),
            body: mr.description.map(|desc| Markdown(desc).into()),
            author: Some(mr.author.name),
            link: Some(mr.web_url),
        }
    }
}
