use crate::models::{Todo, TodoId};
use crate::providers::{Provider, ProviderConfig, ProviderId};
use crate::rich_text::{Markdown, RawRichText};
use async_compat::CompatExt;
use azure_devops_rust_api::git::models::GitPullRequest;
use azure_devops_rust_api::{git::Client, Credential};
use druid::{Data, Lens};
use futures::future::BoxFuture;
use futures::FutureExt;
use im::Vector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Data, Lens)]
pub struct AzureDevopsConfig {
    token: String,
    organization: String,
    project: String,
}

#[derive(Clone)]
pub struct AzureDevopsProvider {
    id: ProviderId,
    client: Client,
    organization: String,
    project: String,
    token: String,
}

impl AzureDevopsProvider {
    pub fn new(id: ProviderId, config: AzureDevopsConfig) -> anyhow::Result<Self> {
        let token = config.token;
        let credential = Credential::from_pat(&token);
        let client = Client::builder(credential).build();

        Ok(Self {
            id,
            client,
            token,
            organization: config.organization,
            project: config.project,
        })
    }

    async fn fetch_todos(&self) -> anyhow::Result<Vector<Todo>> {
        let res = self
            .client
            .pull_requests_client()
            .get_pull_requests_by_project(&self.organization, &self.project)
            .await?;

        let todos = res
            .value
            .into_iter()
            .map(|pr| self.build_todo(pr))
            .collect();

        Ok(todos)
    }

    fn build_todo(&self, value: GitPullRequest) -> Todo {
        Todo {
            provider: self.id,
            id: TodoId::Int(value.pull_request_id as u64),
            title: value.title.unwrap_or_default(),
            state: Some(format!("{:?}", value.status)),
            link: Some(value.url),
            actions: Default::default(),
            body: value.description.map(Markdown).map(RawRichText::from),
            tags: value
                .labels
                .into_iter()
                .flat_map(|label| label.name)
                .collect(),
            comments: Default::default(),
            due_date: Default::default(),
            author: value.created_by.unique_name,
        }
    }
}

impl Provider for AzureDevopsProvider {
    fn to_config(&self) -> ProviderConfig {
        AzureDevopsConfig {
            token: self.token.clone(),
            project: self.project.clone(),
            organization: self.organization.clone(),
        }
        .into()
    }

    fn name(&self) -> &'static str {
        "Azure DevOps"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_todos().compat().boxed()
    }
}
