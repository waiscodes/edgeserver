use crate::{models::deployment::{DeploymentFile, DeploymentFileEntry}, state::State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CarRequest {
    pub deployment_id: String,
    pub files: Vec<DeploymentFileEntry>,
}

pub struct IPFSCluster {
    pub cluster_url: String,
}

impl IPFSCluster {
    pub fn new(cluster_url: String) -> Self {
        Self { cluster_url }
    }

    pub async fn pin_deployment(&self, deployment_id: String, state: &State) -> Result<String, Box<dyn std::error::Error>> {
        let deployment_files = DeploymentFile::get_deployment_files(&state.database, &deployment_id).await?;

        let car_request = CarRequest {
            deployment_id,
            files: deployment_files,
        };

        todo!()
    }

    // pub async fn add_car(&self, car_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    //     let client = reqwest::Client::new();
    //     let response = client.post(&format!("{}/add", self.cluster_url))
    //         .header("Content-Type", "multipart/form-data")
    //         .body(format!("file=@{}", car_path))
    //         .send()
    //         .await?;

    //     let body = response.text().await?;
    //     let json: serde_json::Value = serde_json::from_str(&body)?;
    //     let cid = json["Hash"].as_str().ok_or("CID not found in response")?;
    //     Ok(cid.to_string())
    // }
}
