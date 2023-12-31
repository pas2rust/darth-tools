use super::darth_tools::DarthTools;
use async_trait::async_trait;
use serde_json::Value;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[async_trait]
pub trait FsTrait {
    async fn fs_insert_json(data: &Value, path: &str) -> std::io::Result<()>;
    async fn fs_read_json(path: &str) -> std::io::Result<Value>;
    async fn fs_update_item(id: &str, new_data: &Value, path: &str) -> std::io::Result<()>;
}

#[async_trait]
impl FsTrait for DarthTools {
    async fn fs_insert_json(data: &Value, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path).await?;
        let data_string = data.to_string();
        file.write_all(data_string.as_bytes()).await
    }
    async fn fs_read_json(path: &str) -> std::io::Result<Value> {
        let mut file = File::open(path).await?;
        let mut data_string = String::new();
        file.read_to_string(&mut data_string).await?;
        let data: Value = serde_json::from_str(&data_string).unwrap();
        Ok(data)
    }
    async fn fs_update_item(id: &str, new_data: &Value, path: &str) -> std::io::Result<()> {
        let mut data = Self::fs_read_json(path).await?;
        if let Some(obj) = data.as_object_mut() {
            if obj.contains_key(id) {
                obj.insert(id.to_string(), new_data.clone());
            }
        }
        Self::fs_insert_json(&data, path).await
    }
}
