//! Support for Microsoft Planner
use serde::{Serialize, Deserialize};
use crate::providers::Provider;
use futures::future::BoxFuture;
use futures::future::FutureExt;
use druid::im::Vector;
use crate::models::Todo;
use graph_http::AsyncHttpClient;
use self::models::*;
use graph_rs_sdk::client::Graph;
use graph_oauth::oauth::OAuth;
use futures::future::*;
use futures::stream::StreamExt;
use async_compat::CompatExt;

mod models;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MicrosoftPlannerConfig {
    #[serde(default)]
    name: Option<String>,
    client_id: String,
    client_secret: String,
    // TODO: store in keyring instead of config file
    #[serde(default)]
    access_token: Option<String>,
}

pub struct MicrosoftPlannerProvider {
    name: Option<String>,
    client: graph_rs_sdk::client::Graph<AsyncHttpClient>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

impl MicrosoftPlannerProvider {
    pub async fn new(mut config: MicrosoftPlannerConfig) -> anyhow::Result<Self> {
        if config.access_token.is_none() {
            Self::login(&mut config).await?;
        }

        let access_token = config.access_token.unwrap();
        let client = Graph::new_async(&access_token);

        Ok(Self {
            name: config.name,
            client,
        })
    }

    async fn login(config: &mut MicrosoftPlannerConfig) -> anyhow::Result<()> {
        let mut oauth = Self::oauth_web_client(config);
        let mut request = oauth.build().authorization_code_grant();
        request.browser_authorization().open()?;

        let code = Self::start_access_token_server().await;
        let mut oauth = Self::oauth_web_client(config);
        oauth.access_code(code.code.as_str());
        let mut request = oauth.build().authorization_code_grant();

        let access_token = request.access_token().send()?;
        oauth.access_token(access_token.clone());

        config.access_token = Some(access_token.bearer_token().to_string());

        Ok(())
    }

    async fn start_access_token_server() -> AccessCode {
        use warp::http::Response;
        use warp::Filter;

        let (tx, mut rx) = smol::channel::bounded(1);

        let query = warp::query::<AccessCode>()
            .map(Some)
            .or_else(|_| async { Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,)) });

        let routes = warp::get().and(warp::path("redirect")).and(query).map(
            move |code_option: Option<AccessCode>| match code_option {
                Some(code) => {
                    tx.try_send(code).unwrap();
                    Response::builder().body(String::from(
                        "Successfully Logged In! You can close your browser.",
                    ))
                }
                None => Response::builder()
                    .body(String::from("There was an issue getting the access code.")),
            },
        );

        let server = warp::serve(routes).run(([127, 0, 0, 1], 5858)).compat();
        let token = rx.next();

        futures::pin_mut!(server);
        futures::pin_mut!(token);

        let token = match futures::future::select(server, token).await {
            Either::Right((token, _)) => token,
            _ => unreachable!(),
        };

        token.unwrap()
    }

    fn oauth_web_client(config: &MicrosoftPlannerConfig) -> OAuth {
        let mut oauth = OAuth::new();

        oauth
            .client_id(&config.client_id)
            .client_secret(&config.client_secret)
            .add_scope("tasks.read")
            .add_scope("tasks.read.shared")
            .redirect_uri("http://localhost:5858/redirect")
            .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
            .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
            .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
            .response_type("code")
            .logout_url("https://login.microsoftonline.com/common/oauth2/v2.0/logout");

        oauth
    }

    async fn fetch_tasks(&self) -> anyhow::Result<Vector<Todo>> {
        let res: GraphResponse<PlannerListResponse<PlannerTask>> = self.client.v1()
            .me()
            .planner()
            .list_tasks()
            .json()
            .await?;

        let res = res.into_result()?;

        let todos = res.value
            .into_iter()
            .map(Todo::from)
            .collect();

        Ok(todos)
    }
}

impl Provider for MicrosoftPlannerProvider {
    fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| "Planner".into())
    }

    fn fetch_todos(&self) -> BoxFuture<anyhow::Result<Vector<Todo>>> {
        self.fetch_tasks().compat().boxed()
    }
}

impl From<PlannerTask> for Todo {
    fn from(task: PlannerTask) -> Self {
        Self {
            title: task.title,
            link: None,
            author: None,
            body: None,
            state: None,
        }
    }
}
