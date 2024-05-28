pub mod stream {
    use http_body_util::BodyExt;
    use hyper::body::Incoming;
    use hyper::Response;
    use tokio::io::{self, AsyncWriteExt as _};

    pub async fn write_to_stdout(
        mut res: Response<Incoming>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        while let Some(frame) = res.body_mut().frame().await {
            let frame = frame?;
            if let Some(d) = frame.data_ref() {
                io::stdout().write_all(d).await?;
            }
        }
        Ok(())
    }

    pub async fn get_body_as_astring(
        res: Response<Incoming>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let body_bytes = res.collect().await?.to_bytes();
        let body_str = String::from_utf8(body_bytes.into()).unwrap();
        Ok(body_str)
    }
}
