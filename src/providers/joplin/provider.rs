use super::api;
use super::models;
use crate::models::Todo;
use crate::providers::joplin::models::Note;
use crate::providers::Provider;
use druid::im::Vector;
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct JoplinConfig {
    token: String,
    #[serde(default)]
    notebooks: Option<Vec<String>>,
}

pub struct JoplinProvider {
    api: api::JoplinApi,
    notebooks: Option<Vec<String>>,
}

impl JoplinProvider {
    pub fn new(config: JoplinConfig) -> anyhow::Result<Self> {
        Ok(Self {
            api: api::JoplinApi::new(config.token),
            notebooks: config.notebooks,
        })
    }

    async fn fetch_notes(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Joplin notes...");
        let notes = self.api.get_todo_notes().await?;
        let mut notes_with_tags = Vec::new();
        for note in notes {
            let tags = self.api.get_note_tags(&note.id).await?;
            notes_with_tags.push((note, tags));
        }
        let todos: Vector<_> = notes_with_tags
            .into_iter()
            .filter(|(note, _)| {
                if let Some(notebooks) = self.notebooks.as_ref() {
                    notebooks.contains(&note.notebook_id)
                } else {
                    true
                }
            })
            .map(Todo::from)
            .collect();
        tracing::info!("Fetched {} Joplin notes", todos.len());

        Ok(todos)
    }
}

impl Provider for JoplinProvider {
    fn name(&self) -> &'static str {
        "Joplin"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_notes().boxed()
    }
}

impl From<(models::Note, Vec<models::Tag>)> for Todo {
    fn from((note, tags): (Note, Vec<models::Tag>)) -> Self {
        Self {
            title: note.title,
            state: None,
            tags: tags.into_iter().map(|tag| tag.title).collect(),
            author: None,
            body: Some(note.body.into()),
            link: None,
        }
    }
}
