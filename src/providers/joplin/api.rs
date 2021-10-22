use super::models::Note;
use crate::providers::joplin::models::JoplinResponse;
use serde::Serialize;

pub struct JoplinApi {
    token: String,
    url: String,
}

impl JoplinApi {
    pub fn new(token: String) -> Self {
        Self {
            token,
            url: "http://localhost:41184".into(),
        }
    }

    pub async fn get_todo_notes(&self) -> anyhow::Result<Vec<Note>> {
        let mut query = GetNotesQuery {
            token: &self.token,
            fields: "id,parent_id,title,body,is_todo,todo_due,todo_completed",
            page: 0,
        };
        let mut todos = Vec::new();
        loop {
            let mut res = surf::get(format!("{}/notes", &self.url))
                .content_type("application/json")
                .query(&query)
                .map_err(surf::Error::into_inner)?
                .await
                .map_err(surf::Error::into_inner)?;

            #[cfg(debug_assertions)]
            if !res.status().is_success() {
                let body = res.body_string().await.map_err(surf::Error::into_inner)?;
                tracing::error!("{:?}", res.status());
                tracing::error!("{:?}", body);
                anyhow::bail!("Joplin api failure");
            }
            anyhow::ensure!(
                res.status().is_success(),
                "Joplin api returned non success status code"
            );

            let res: JoplinResponse<Note> =
                res.body_json().await.map_err(surf::Error::into_inner)?;
            tracing::trace!("{:?}", res);
            for note in res.items {
                if note.is_todo() && !note.is_completed() {
                    todos.push(note);
                }
            }
            if !res.has_more {
                break;
            }
            query.page += 1;
        }

        Ok(todos)
    }
}

#[derive(Debug, Serialize)]
struct GetNotesQuery<'a> {
    token: &'a str,
    fields: &'a str,
    page: usize,
}
