use std::sync::Arc;

use color_eyre::eyre::Result;
use figment::{Figment, providers::Env};
use serde::Deserialize;

use crate::{cache::Cache, database::Database, handlers::TaskRabbit, ipfs::IPFSModule, storage::Storage};

pub type State = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: AppConfig,
    pub database: Database,
    pub storage: Storage,
    pub cache: Cache,
    pub rabbit: Option<TaskRabbit>,
    pub ipfs: Option<IPFSModule>,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    /// Information
    pub database_url: String,
    pub s3: S3Config,
    pub s3_previews: Option<S3PreviewsConfig>,
    pub s3_car: Option<S3CarConfig>,
    pub github_app: Option<GithubAppConfig>,
    pub amqp: Option<AMQPConfig>,
    pub ipfs: Option<IPFSConfig>,
}

#[derive(Deserialize, Debug)]
pub struct S3Config {
    pub endpoint_url: String,
    pub region: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Deserialize, Debug)]
pub struct S3PreviewsConfig {
    pub endpoint_url: String,
    pub region: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Deserialize, Debug)]
pub struct S3CarConfig {
    pub endpoint_url: String,
    pub region: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Deserialize, Debug)]
pub struct AMQPConfig {
    pub addr: String,
    pub previews_queue: Option<String>,
    pub car_queue: Option<String>,
    pub car_result_queue: Option<String>,
}

/// Github App Config
/// 
/// Setup your app with Callback URL
/// /api/github/oauth
/// 
/// Setup URL (optional)
/// /github/setup
/// 
/// Webhook URL (optional)
/// /api/github/webhook
#[derive(Deserialize, Debug)]
pub struct GithubAppConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize, Debug)]
pub struct IPFSConfig {
    pub cluster_url: String,
    pub public_cluster_url: String,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        // let config = Config::builder()
        //     .add_source(Environment::default())
        //     .build()?;

        // Deserialize into the AppConfig struct
        // let config: AppConfig = config.try_deserialize()?;

        let config = Figment::new()
            .merge(Env::prefixed("DATABASE_").map(|key| format!("database_{}", key.as_str().to_lowercase()).into()))
            .merge(Env::prefixed("S3_")
                .map(|key| format!("s3.{}", key.as_str().to_lowercase()).into()))
            .merge(Env::prefixed("S3_PREVIEWS_")
                .map(|key| format!("s3_previews.{}", key.as_str().to_lowercase()).into()))
            .merge(Env::prefixed("S3_CAR_")
                .map(|key| format!("s3_car.{}", key.as_str().to_lowercase()).into()))
            .merge(Env::prefixed("GITHUB_APP_")
                .map(|key| format!("github_app.{}", key.as_str().to_lowercase()).into()))
            .merge(Env::prefixed("AMQP_")
                .map(|key| format!("amqp.{}", key.as_str().to_lowercase()).into()))
            .merge(Env::prefixed("IPFS_")
                .map(|key| format!("ipfs.{}", key.as_str().to_lowercase()).into()))
            .extract::<AppConfig>()
            .expect("Failed to load AppConfig configuration");

        let database = Database::new(&config.database_url).await?;

        let storage = Storage::from_config(&config);

        let cache = Cache::default();

        let rabbit = if let Some(amqp) = &config.amqp {
            Some(TaskRabbit::init(amqp).await)
        } else {
            None
        };

        let ipfs = if let Some(ipfs) = &config.ipfs {
            Some(IPFSModule::new(ipfs.cluster_url.clone(), ipfs.public_cluster_url.clone()))
        } else {
            None
        };

        Ok(Self {
            config,
            database,
            storage,
            cache,
            rabbit,
            ipfs,
        })
    }
}
