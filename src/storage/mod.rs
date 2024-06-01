pub mod file {
    use http_body_util::BodyExt;
    use hyper::body::Incoming;
    use hyper::Response;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt as _;

    pub async fn store_response_as_file(
        filename: &str,
        filepath: &str,
        mut res: Response<Incoming>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Create file.
        let mut fullpath: String = String::from(filepath);
        fullpath.push_str(filename);

        let mut file: File = File::create(fullpath).await?;

        // Write all frames to file.
        while let Some(frame) = res.body_mut().frame().await {
            let frame = frame?;
            if let Some(d) = frame.data_ref() {
                file.write_all(d).await?;
            }
        }

        file.sync_all().await?;

        Ok(filepath.to_string())
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
