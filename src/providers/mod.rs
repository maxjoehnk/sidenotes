use crate::models::{Todo, TodoAction, TodoComment, TodoId};
use derive_more::From;
use druid::im::Vector;
use druid::{Data, Lens};
use enum_dispatch::enum_dispatch;
use futures::future::{ok, BoxFuture};
use futures::FutureExt;
use serde::{Deserialize, Serialize};

#[cfg(feature = "confluence")]
pub mod confluence;
#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "gitlab")]
pub mod gitlab;
#[cfg(feature = "jira")]
pub mod jira;
#[cfg(feature = "joplin")]
pub mod joplin;
#[cfg(feature = "nextcloud")]
pub mod nextcloud;
#[cfg(feature = "taskwarrior")]
pub mod taskwarrior;
#[cfg(feature = "upsource")]
pub mod upsource;

#[derive(Debug, Clone, Deserialize, Serialize, Data, Lens)]
pub struct ProviderConfigEntry {
    #[serde(skip, default)]
    #[data(ignore)]
    pub id: ProviderId,
    #[serde(flatten)]
    pub provider: ProviderConfig,
    #[serde(flatten, default)]
    pub settings: ProviderSettings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ProviderId(uuid::Uuid);

impl Default for ProviderId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Data for ProviderId {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, Data, Lens)]
pub struct ProviderSettings {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "exclude")]
    pub exclude_status: Vector<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, From, PartialEq, Eq, Data)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ProviderConfig {
    #[cfg(feature = "github")]
    Github(github::GithubConfig),
    #[cfg(feature = "gitlab")]
    Gitlab(gitlab::GitlabConfig),
    #[cfg(feature = "jira")]
    Jira(jira::JiraConfig),
    #[cfg(feature = "confluence")]
    Confluence(confluence::ConfluenceConfig),
    #[cfg(feature = "joplin")]
    Joplin(joplin::JoplinConfig),
    #[cfg(feature = "taskwarrior")]
    Taskwarrior(taskwarrior::TaskwarriorConfig),
    #[cfg(feature = "upsource")]
    Upsource(upsource::UpsourceConfig),
    #[cfg(feature = "nextcloud")]
    NextcloudDeck(nextcloud::deck::NextcloudDeckProviderConfig),
}

impl ProviderConfig {
    pub async fn create(self, id: ProviderId) -> anyhow::Result<ProviderImpl> {
        let provider = match self {
            #[cfg(feature = "github")]
            Self::Github(config) => github::GithubProvider::new(id, config)?.into(),
            #[cfg(feature = "gitlab")]
            Self::Gitlab(config) => gitlab::GitlabProvider::new(id, config).await?.into(),
            #[cfg(feature = "jira")]
            Self::Jira(config) => jira::JiraProvider::new(id, config)?.into(),
            #[cfg(feature = "jira")]
            Self::Confluence(config) => confluence::ConfluenceProvider::new(id, config)?.into(),
            #[cfg(feature = "joplin")]
            Self::Joplin(config) => joplin::JoplinProvider::new(id, config)?.into(),
            #[cfg(feature = "taskwarrior")]
            Self::Taskwarrior(config) => taskwarrior::TaskwarriorProvider::new(id, config)?.into(),
            #[cfg(feature = "upsource")]
            Self::Upsource(config) => upsource::UpsourceProvider::new(id, config)?.into(),
            #[cfg(feature = "nextcloud")]
            Self::NextcloudDeck(config) => {
                nextcloud::deck::NextcloudDeckProvider::new(id, config).into()
            }
        };

        Ok(provider)
    }
}

#[enum_dispatch]
#[derive(Clone)]
pub enum ProviderImpl {
    #[cfg(feature = "confluence")]
    Confluence(confluence::ConfluenceProvider),
    #[cfg(feature = "github")]
    Github(github::GithubProvider),
    #[cfg(feature = "gitlab")]
    Gitlab(gitlab::GitlabProvider),
    #[cfg(feature = "jira")]
    Jira(jira::JiraProvider),
    #[cfg(feature = "joplin")]
    Joplin(joplin::JoplinProvider),
    #[cfg(feature = "nextcloud")]
    NextcloudDeck(nextcloud::deck::NextcloudDeckProvider),
    #[cfg(feature = "taskwarrior")]
    Taskwarrior(taskwarrior::TaskwarriorProvider),
    #[cfg(feature = "upsource")]
    Upsource(upsource::UpsourceProvider),
}

#[enum_dispatch(ProviderImpl)]
pub trait Provider: Sync + Send {
    fn fetch_comments(&self, _: TodoId) -> BoxFuture<anyhow::Result<Vector<TodoComment>>> {
        ok(Default::default()).boxed()
    }
    fn to_config(&self) -> ProviderConfig;
    fn name(&self) -> &'static str;
    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>>;

    fn run_action(&self, _: TodoId, _: TodoAction) -> BoxFuture<anyhow::Result<()>> {
        unimplemented!()
    }
}

pub(self) trait IntoTodo {
    fn into_todo(self, provider_id: ProviderId) -> Todo;
}
