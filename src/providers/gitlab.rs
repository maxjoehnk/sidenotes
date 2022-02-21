use crate::models::Todo;
use crate::providers::{IntoTodo, Provider, ProviderConfig, ProviderId};
use crate::rich_text::Markdown;
use async_compat::CompatExt;
use druid::{Data, Lens};
use futures::future::BoxFuture;
use futures::FutureExt;
use gitlab::api::projects::{self, merge_requests};
use gitlab::{api, api::AsyncQuery, AsyncGitlab, GitlabBuilder, MergeRequest, Project};
use im::Vector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Data, Lens)]
pub struct GitlabConfig {
    url: String,
    token: String,
    #[serde(default)]
    repos: Option<Vector<String>>,
    #[serde(default)]
    show_drafts: bool,
}

#[derive(Clone)]
pub struct GitlabProvider {
    id: ProviderId,
    client: AsyncGitlab,
    repos: Option<Vec<String>>,
    show_drafts: bool,
    url: String,
    token: String,
}

impl GitlabProvider {
    pub async fn new(id: ProviderId, config: GitlabConfig) -> anyhow::Result<Self> {
        let url = config.url.clone();
        let token = config.token.clone();
        let client = GitlabBuilder::new(config.url, config.token)
            .insecure()
            .build_async()
            .compat()
            .await?;

        Ok(Self {
            id,
            client,
            repos: config.repos.map(|repos| repos.into_iter().collect()),
            show_drafts: config.show_drafts,
            url,
            token,
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
                todos.push_back(request.into_todo(self.id));
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
    fn to_config(&self) -> ProviderConfig {
        GitlabConfig {
            repos: self
                .repos
                .as_ref()
                .map(|repos| repos.iter().cloned().collect()),
            url: self.url.clone(),
            token: self.token.clone(),
            show_drafts: self.show_drafts,
        }
        .into()
    }

    fn name(&self) -> &'static str {
        "Gitlab"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_merge_requests().compat().boxed()
    }
}

impl IntoTodo for MergeRequest {
    fn into_todo(self, id: ProviderId) -> Todo {
        Todo {
            provider: id,
            id: self.id.value().into(),
            title: format!("#{} - {}", self.iid, self.title),
            state: Some(format!("{:?}", self.state)),
            tags: self.labels.into(),
            body: self.description.map(|desc| Markdown(desc).into()),
            author: Some(self.author.name),
            link: Some(self.web_url),
            actions: Default::default(),
            comments: Default::default(),
        }
    }
}
