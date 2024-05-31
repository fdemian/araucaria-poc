pub mod file {
    use http_body_util::BodyExt;
    use hyper::body::Incoming;
    use hyper::Response;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt as _;

    pub async fn store_response_as_file(
        filepath: &str,
        mut res: Response<Incoming>,
    ) -> Result<&str, Box<dyn std::error::Error + Send + Sync>> {
        // Create file.
        let mut file: File = File::create(filepath).await?;

        // Write all frames to file.
        while let Some(frame) = res.body_mut().frame().await {
            let frame = frame?;
            if let Some(d) = frame.data_ref() {
                file.write_all(d).await?;
            }
        }

        file.sync_all().await?;
        Ok(filepath)
    }
}

pub mod key_value {
    pub async fn store_value(key: &str, value: &str) -> serde_json::Value {
        return serde_json::json!({
        "ok": true,
        "key": key,
        "value": value
        });
    }
}
