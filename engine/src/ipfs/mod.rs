#[derive(Debug, Clone)]
pub struct IPFSModule {
    pub cluster_url: String,
    pub public_cluster_url: String,
}

impl IPFSModule {
    pub fn new(cluster_url: String, public_cluster_url: String) -> Self {
        Self { cluster_url, public_cluster_url }
    }
}
