use super::models::*;
use crate::providers::joplin::models::JoplinResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};
use surf::Response;

#[derive(Clone)]
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

    pub async fn get_todo_notes_for_notebook(
        &self,
        notebook_id: &str,
    ) -> anyhow::Result<Vec<Note>> {
        let url = format!("{}/folders/{}/notes", &self.url, notebook_id);
        let query = PagedQuery {
            token: &self.token,
            fields: "id,parent_id,title,body,is_todo,todo_due,todo_completed",
            page: 1,
        };
        let todos = self.get_paged::<Note>(url, query).await?;
        let todos = todos
            .into_iter()
            .filter(|note| note.is_todo() && !note.is_completed())
            .collect();

        Ok(todos)
    }

    pub async fn get_notebooks(&self) -> anyhow::Result<Vec<Notebook>> {
        let url = format!("{}/folders", &self.url);
        let query = PagedQuery {
            token: &self.token,
            fields: "id,parent_id,title",
            page: 1,
        };
        let notebooks = self.get_paged(url, query).await?;

        Ok(notebooks)
    }

    pub async fn get_note_tags(&self, note_id: &str) -> anyhow::Result<Vec<Tag>> {
        let query = BaseQuery { token: &self.token };
        let mut res = surf::get(format!("{}/notes/{}/tags", &self.url, note_id))
            .content_type("application/json")
            .query(&query)
            .map_err(surf::Error::into_inner)?
            .await
            .map_err(surf::Error::into_inner)?;

        Self::assert_response_status(&mut res).await?;

        let res: JoplinResponse<Tag> = res.body_json().await.map_err(surf::Error::into_inner)?;
        tracing::trace!("{:?}", res);

        Ok(res.items)
    }

    async fn get_paged<T: DeserializeOwned + Debug>(
        &self,
        url: String,
        mut query: PagedQuery<'_>,
    ) -> anyhow::Result<Vec<T>> {
        let mut items = Vec::new();
        loop {
            tracing::trace!("GET {} {:?}", &url, &query);
            let mut res = surf::get(&url)
                .content_type("application/json")
                .query(&query)
                .map_err(surf::Error::into_inner)?
                .await
                .map_err(surf::Error::into_inner)?;

            Self::assert_response_status(&mut res).await?;

            let res: JoplinResponse<T> = res.body_json().await.map_err(surf::Error::into_inner)?;
            tracing::trace!("{:?}", res);
            items.extend(res.items.into_iter());
            if !res.has_more {
                break;
            }
            query.page += 1;
        }
        Ok(items)
    }

    pub async fn mark_as_done(&self, todo_id: String) -> anyhow::Result<()> {
        let query = BaseQuery { token: &self.token };
        let mut res = surf::put(format!("{}/notes/{}", &self.url, todo_id))
            .content_type("application/json")
            .query(&query)
            .map_err(surf::Error::into_inner)?
            .body_json(&UpdateTodo {
                todo_completed: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
            })
            .map_err(surf::Error::into_inner)?
            .await
            .map_err(surf::Error::into_inner)?;

        Self::assert_response_status(&mut res).await?;

        Ok(())
    }

    async fn assert_response_status(res: &mut Response) -> anyhow::Result<()> {
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

        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct PagedQuery<'a> {
    token: &'a str,
    fields: &'a str,
    page: usize,
}

#[derive(Debug, Serialize)]
struct BaseQuery<'a> {
    token: &'a str,
}

#[derive(Debug, Serialize)]
struct UpdateTodo {
    todo_completed: u64,
}
