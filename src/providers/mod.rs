use crate::models::Todo;
use druid::im::Vector;
use druid::Data;
use futures::future::BoxFuture;
use serde::Deserialize;

#[cfg(feature = "p_confluence")]
mod confluence;
#[cfg(feature = "p_github")]
mod github;
#[cfg(feature = "p_gitlab")]
mod gitlab;
#[cfg(feature = "p_jira")]
mod jira;
#[cfg(feature = "p_joplin")]
mod joplin;
#[cfg(feature = "p_nextcloud")]
mod nextcloud;
#[cfg(feature = "p_taskwarrior")]
mod taskwarrior;
#[cfg(feature = "p_upsource")]
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
    #[cfg(feature = "p_github")]
    Github(github::GithubConfig),
    #[cfg(feature = "p_gitlab")]
    Gitlab(gitlab::GitlabConfig),
    #[cfg(feature = "p_jira")]
    Jira(jira::JiraConfig),
    #[cfg(feature = "p_confluence")]
    Confluence(confluence::ConfluenceConfig),
    #[cfg(feature = "p_joplin")]
    Joplin(joplin::JoplinConfig),
    #[cfg(feature = "p_taskwarrior")]
    Taskwarrior(taskwarrior::TaskwarriorConfig),
    #[cfg(feature = "p_upsource")]
    Upsource(upsource::UpsourceConfig),
    #[cfg(feature = "p_nextcloud")]
    NextcloudDeck(nextcloud::deck::NextcloudDeckProviderConfig),
}

impl ProviderConfig {
    pub async fn create(self) -> anyhow::Result<Box<dyn Provider>> {
        let provider = match self {
            #[cfg(feature = "p_github")]
            Self::Github(config) => {
                Box::new(github::GithubProvider::new(config)?) as Box<dyn Provider>
            }
            #[cfg(feature = "p_gitlab")]
            Self::Gitlab(config) => {
                Box::new(gitlab::GitlabProvider::new(config).await?) as Box<dyn Provider>
            }
            #[cfg(feature = "p_jira")]
            Self::Jira(config) => Box::new(jira::JiraProvider::new(config)?),
            #[cfg(feature = "p_confluence")]
            Self::Confluence(config) => Box::new(confluence::ConfluenceProvider::new(config)?),
            #[cfg(feature = "p_joplin")]
            Self::Joplin(config) => Box::new(joplin::JoplinProvider::new(config)?),
            #[cfg(feature = "p_taskwarrior")]
            Self::Taskwarrior(config) => Box::new(taskwarrior::TaskwarriorProvider::new(config)?),
            #[cfg(feature = "p_upsource")]
            Self::Upsource(config) => Box::new(upsource::UpsourceProvider::new(config)?),
            #[cfg(feature = "p_nextcloud")]
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
