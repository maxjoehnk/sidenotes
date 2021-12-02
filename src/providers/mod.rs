use crate::models::Todo;
use druid::im::Vector;
use druid::Data;
use futures::future::BoxFuture;
use serde::Deserialize;

#[cfg(feature = "github")]
mod github;
#[cfg(feature = "gitlab")]
mod gitlab;
#[cfg(feature = "jira")]
mod jira;
#[cfg(feature = "joplin")]
mod joplin;
#[cfg(feature = "nextcloud")]
mod nextcloud;
#[cfg(feature = "taskwarrior")]
mod taskwarrior;
#[cfg(feature = "upsource")]
mod upsource;

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfigEntry {
    #[serde(flatten)]
    pub provider: ProviderConfig,
    #[serde(flatten, default)]
    pub settings: ProviderSettings,
}

#[derive(Default, Debug, Clone, Deserialize, Data)]
pub struct ProviderSettings {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default, rename = "exclude")]
    pub exclude_status: Vector<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ProviderConfig {
    #[cfg(feature = "github")]
    Github(github::GithubConfig),
    #[cfg(feature = "gitlab")]
    Gitlab(gitlab::GitlabConfig),
    #[cfg(feature = "jira")]
    Jira(jira::JiraConfig),
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
    pub async fn create(self) -> anyhow::Result<Box<dyn Provider>> {
        let provider = match self {
            #[cfg(feature = "github")]
            Self::Github(config) => {
                Box::new(github::GithubProvider::new(config)?) as Box<dyn Provider>
            }
            #[cfg(feature = "gitlab")]
            Self::Gitlab(config) => {
                Box::new(gitlab::GitlabProvider::new(config).await?) as Box<dyn Provider>
            }
            #[cfg(feature = "jira")]
            Self::Jira(config) => Box::new(jira::JiraProvider::new(config)?),
            #[cfg(feature = "joplin")]
            Self::Joplin(config) => Box::new(joplin::JoplinProvider::new(config)?),
            #[cfg(feature = "taskwarrior")]
            Self::Taskwarrior(config) => Box::new(taskwarrior::TaskwarriorProvider::new(config)?),
            #[cfg(feature = "upsource")]
            Self::Upsource(config) => Box::new(upsource::UpsourceProvider::new(config)?),
            #[cfg(feature = "nextcloud")]
            Self::NextcloudDeck(config) => {
                Box::new(nextcloud::deck::NextcloudDeckProvider::new(config))
            }
        };

        Ok(provider)
    }
}

pub trait Provider {
    fn name(&self) -> &'static str;
    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>>;
}
