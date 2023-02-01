use crate::providers::nextcloud::deck::models::*;
use base64::prelude::*;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

#[derive(Clone)]
pub struct NextcloudApi {
    pub(super) host: String,
    pub(super) username: String,
    pub(super) password: String,
}

impl NextcloudApi {
    pub fn new(host: String, username: String, password: String) -> Self {
        Self {
            host,
            username,
            password,
        }
    }

    pub async fn get_user(&self) -> anyhow::Result<UserStatusModel> {
        let res = self
            .get::<OcsResponse<UserStatusModel>>("ocs/v2.php/apps/user_status/api/v1/user_status")
            .await?;

        Ok(res.ocs.data)
    }

    pub async fn get_boards(&self) -> anyhow::Result<Vec<BoardModel>> {
        self.get_deck_api("boards").await
    }

    pub async fn get_stacks(&self, board_id: u32) -> anyhow::Result<Vec<StackModel>> {
        self.get_deck_api(&format!("boards/{}/stacks", board_id))
            .await
    }

    async fn get_deck_api<T: DeserializeOwned + Debug>(&self, url: &str) -> anyhow::Result<T> {
        self.get(&format!("index.php/apps/deck/api/v1.0/{}", url))
            .await
    }

    async fn get<T: DeserializeOwned + Debug>(&self, url: &str) -> anyhow::Result<T> {
        let mut res = surf::get(format!("{}/{}", &self.host, url))
            .header("Accept", "application/json")
            .header("OCS-APIRequest", "true")
            .header("Authorization", self.auth_header())
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;

        #[cfg(debug_assertions)]
        if !res.status().is_success() {
            eprintln!("{:?}", res.status());
            eprintln!(
                "{:?}",
                res.body_string()
                    .await
                    .map_err(|err| anyhow::anyhow!("{:?}", err))?
            );
            anyhow::bail!("Nextcloud deck api failure");
        }
        anyhow::ensure!(
            res.status().is_success(),
            "Nextcloud deck api returned non success status code"
        );

        let res: T = res
            .body_json()
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;
        tracing::trace!("{:?}", res);

        Ok(res)
    }

    fn auth_header(&self) -> String {
        let unencoded = format!("{}:{}", self.username, self.password);
        let encoded = BASE64_STANDARD.encode(unencoded);

        format!("Basic {}", encoded)
    }
}
