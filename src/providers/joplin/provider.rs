use super::api;
use crate::models::{LocalDateTime, Todo, TodoId};
use crate::providers::joplin::models::{Notebook, TodoNote};
use crate::providers::{Provider, ProviderConfig, ProviderId, TodoAction};
use druid::{Data, Lens};
use futures::future::BoxFuture;
use futures::FutureExt;
use im::Vector;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

const MARK_AS_DONE_ACTION: &str = "JOPLIN_MARK_AS_DONE";

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Data, Lens)]
pub struct JoplinConfig {
    token: String,
    #[serde(default)]
    notebooks: Option<Vector<String>>,
    #[serde(default)]
    show_notebook_names: bool,
}

#[derive(Clone)]
pub struct JoplinProvider {
    id: ProviderId,
    api: api::JoplinApi,
    notebooks: Option<Vec<String>>,
    show_notebook_names: bool,
}

impl JoplinProvider {
    pub fn new(id: ProviderId, config: JoplinConfig) -> anyhow::Result<Self> {
        Ok(Self {
            id,
            api: api::JoplinApi::new(config.token),
            notebooks: config
                .notebooks
                .map(|notebooks| notebooks.into_iter().collect()),
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
                    due_date: if note.todo_due == 0 {
                        None
                    } else {
                        Some(LocalDateTime::from_timestamp(note.todo_due))
                    },
                };
                notes.push(note);
            }
        }
        let mut todos: Vector<_> = notes.into_iter().map(|note| note.map(self)).collect();
        todos.sort_by(|lhs, rhs| match (&lhs.due_date, &rhs.due_date) {
            (Some(lhs), Some(rhs)) if lhs > rhs => Ordering::Greater,
            (Some(lhs), Some(rhs)) if lhs < rhs => Ordering::Less,
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            _ => Ordering::Equal,
        });
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

    async fn mark_done(&self, id: TodoId) -> anyhow::Result<()> {
        if let TodoId::String(id) = id {
            self.api.mark_as_done(id).await?;

            Ok(())
        } else {
            unimplemented!()
        }
    }
}

impl Provider for JoplinProvider {
    fn to_config(&self) -> ProviderConfig {
        JoplinConfig {
            token: self.api.token.clone(),
            notebooks: self
                .notebooks
                .as_ref()
                .map(|notebooks| notebooks.iter().cloned().collect()),
            show_notebook_names: self.show_notebook_names,
        }
        .into()
    }

    fn name(&self) -> &'static str {
        "Joplin"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_notes().boxed()
    }

    fn run_action(&self, todo: TodoId, action: TodoAction) -> BoxFuture<anyhow::Result<()>> {
        async move {
            if action.id == MARK_AS_DONE_ACTION {
                self.mark_done(todo).await?;
                Ok(())
            } else {
                unimplemented!()
            }
        }
        .boxed()
    }
}

impl TodoNote {
    fn map(self, provider: &JoplinProvider) -> Todo {
        let mut tags: Vector<String> = self.tags.into_iter().map(|tag| tag.title).collect();
        if provider.show_notebook_names {
            tags.push_back(self.notebook.title);
        }

        Todo {
            provider: provider.id,
            id: self.id.into(),
            title: self.title,
            state: None,
            tags,
            author: None,
            body: Some(self.body.into()),
            link: None,
            actions: vec![TodoAction {
                id: MARK_AS_DONE_ACTION,
                label: "Done",
            }]
            .into(),
            comments: Default::default(),
            due_date: self.due_date,
        }
    }
}
