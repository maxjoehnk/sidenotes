use serde::Deserialize;
use crate::models::Todo;
use futures::future::BoxFuture;
use druid::im::Vector;

#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "jira")]
mod jira;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ProviderConfig {
    #[cfg(feature = "github")]
    Github(github::GithubConfig),
    #[cfg(feature = "jira")]
    Jira(jira::JiraConfig)
}

impl ProviderConfig {
    pub fn create(self) -> anyhow::Result<Box<dyn Provider>> {
        let provider = match self {
            #[cfg(feature = "github")]
            Self::Github(config) => Box::new(github::GithubProvider::new(config)?) as Box<dyn Provider>,
            #[cfg(feature = "jira")]
            Self::Jira(config) => Box::new(jira::JiraProvider::new(config)?),
        };

        Ok(provider)
    }
}

pub trait Provider {
    fn name(&self) -> String;
    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>>;
}