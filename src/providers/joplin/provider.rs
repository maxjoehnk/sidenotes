use super::api;
use super::models;
use crate::providers::Provider;
use futures::future::BoxFuture;
use druid::im::Vector;
use crate::models::Todo;
use futures::FutureExt;
use crate::providers::joplin::models::Note;
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
            notebooks: config.notebooks
        })
    }

    async fn fetch_notes(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Joplin notes...");
        let notes = self.api.get_todo_notes().await?;
        let todos: Vector<_> = notes.into_iter()
            .filter(|note| {
                if let Some(notebooks) = self.notebooks.as_ref() {
                    notebooks.contains(&note.notebook_id)
                }else {
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
    fn name(&self) -> String {
        "Joplin".into()
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_notes().boxed()
    }
}

impl From<models::Note> for Todo {
    fn from(note: Note) -> Self {
        Self {
            completed: note.is_completed(),
            title: note.title,
            state: None,
        }
    }
}
