use super::api;
use crate::models::Todo;
use crate::providers::joplin::models::{Notebook, TodoNote};
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
    #[serde(default)]
    show_notebook_names: bool,
}

pub struct JoplinProvider {
    api: api::JoplinApi,
    notebooks: Option<Vec<String>>,
    show_notebook_names: bool,
}

impl JoplinProvider {
    pub fn new(config: JoplinConfig) -> anyhow::Result<Self> {
        Ok(Self {
            api: api::JoplinApi::new(config.token),
            notebooks: config.notebooks,
            show_notebook_names: config.show_notebook_names,
        })
    }

    async fn fetch_notes(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Joplin notes...");
        let mut notes = Vec::new();
        let notebooks = self.get_notebooks().await?;
        for notebook in notebooks.iter() {
            let notebook_notes = self.api.get_todo_notes_for_notebook(&notebook.id).await?;
            for note in notebook_notes {
                let tags = self.api.get_note_tags(&note.id).await?;
                let note = TodoNote {
                    id: note.id,
                    title: note.title,
                    body: note.body,
                    tags,
                    notebook: notebook.clone(),
                };
                notes.push(note);
            }
        }
        let todos: Vector<_> = notes.into_iter().map(|note| note.map(&self)).collect();
        tracing::info!("Fetched {} Joplin notes", todos.len());

        Ok(todos)
    }

    async fn get_notebooks(&self) -> anyhow::Result<Vec<Notebook>> {
        let notebooks = self.api.get_notebooks().await?;
        let notebooks = if let Some(notebook_ids) = self.notebooks.as_ref() {
            notebooks
                .into_iter()
                .filter(|nb| notebook_ids.contains(&nb.id))
                .collect()
        } else {
            notebooks
        };

        Ok(notebooks)
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

impl TodoNote {
    fn map(self, provider: &JoplinProvider) -> Todo {
        let mut tags: Vector<String> = self.tags.into_iter().map(|tag| tag.title).collect();
        if provider.show_notebook_names {
            tags.push_back(self.notebook.title);
        }

        Todo {
            title: self.title,
            state: None,
            tags,
            author: None,
            body: Some(self.body.into()),
            link: None,
        }
    }
}
