use crate::models::Todo;
use crate::providers::nextcloud::deck::api::NextcloudApi;
use crate::providers::nextcloud::deck::models::CardModel;
use crate::providers::Provider;
use crate::rich_text::RawRichText;
use druid::im::Vector;
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct NextcloudDeckProviderConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub boards: Vec<BoardConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BoardConfig {
    title: String,
    stacks: Vec<String>,
}

pub struct NextcloudDeckProvider {
    api: NextcloudApi,
    boards: Vec<BoardConfig>,
}

impl NextcloudDeckProvider {
    pub fn new(config: NextcloudDeckProviderConfig) -> Self {
        Self {
            api: NextcloudApi::new(config.host, config.username, config.password),
            boards: config.boards,
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
        let todos: Vector<_> = cards.into_iter().map(Todo::from).collect();
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

impl From<(String, CardModel)> for Todo {
    fn from((stack, card): (String, CardModel)) -> Self {
        Self {
            title: card.title,
            body: Some(RawRichText::Markdown(card.description.into())),
            state: Some(stack),
            tags: Default::default(),
            author: None,
            link: None,
        }
    }
}
