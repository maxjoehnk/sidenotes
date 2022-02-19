use crate::models::Todo;
use crate::providers::nextcloud::deck::api::NextcloudApi;
use crate::providers::nextcloud::deck::models::CardModel;
use crate::providers::{IntoTodo, Provider, ProviderId};
use crate::rich_text::RawRichText;
use druid::im::Vector;
use druid::{Data, Lens};
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Data, Lens)]
pub struct NextcloudDeckProviderConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub boards: Vector<BoardConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Data, Lens)]
pub struct BoardConfig {
    title: String,
    stacks: Vector<String>,
}

#[derive(Clone)]
pub struct NextcloudDeckProvider {
    id: ProviderId,
    api: NextcloudApi,
    boards: Vec<BoardConfig>,
}

impl NextcloudDeckProvider {
    pub fn new(id: ProviderId, config: NextcloudDeckProviderConfig) -> Self {
        Self {
            id,
            api: NextcloudApi::new(config.host, config.username, config.password),
            boards: config.boards.into_iter().collect(),
        }
    }

    async fn fetch_cards(&self) -> anyhow::Result<Vector<Todo>> {
        tracing::info!("Fetching Nextcloud Deck cards...");
        let user = self.api.get_user().await?;
        let boards = self.api.get_boards().await?;
        let mut cards = Vec::new();
        for board in boards.into_iter() {
            if let Some(board_config) = self.boards.iter().find(|b| board.title == b.title) {
                let stacks = self.api.get_stacks(board.id).await?;
                for stack in stacks
                    .into_iter()
                    .filter(|stack| board_config.stacks.contains(&stack.title))
                {
                    for card in stack.cards {
                        if !card
                            .assigned_users
                            .iter()
                            .any(|u| u.participant.uid == user.user_id)
                        {
                            continue;
                        }
                        cards.push((stack.title.clone(), card));
                    }
                }
            }
        }
        let todos: Vector<_> = cards
            .into_iter()
            .map(|card| card.into_todo(self.id))
            .collect();
        tracing::info!("Fetched {} Nextcloud Deck cards", todos.len());

        Ok(todos)
    }
}

impl Provider for NextcloudDeckProvider {
    fn name(&self) -> &'static str {
        "Nextcloud Deck"
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_cards().boxed()
    }
}

impl IntoTodo for (String, CardModel) {
    fn into_todo(self, id: ProviderId) -> Todo {
        let (stack, card) = self;

        Todo {
            provider: id,
            id: card.id.into(),
            title: card.title,
            body: Some(RawRichText::Markdown(card.description.into())),
            state: Some(stack),
            tags: Default::default(),
            author: None,
            link: None,
            actions: Default::default(),
        }
    }
}
