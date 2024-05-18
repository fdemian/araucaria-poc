use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::Response;
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt as _};

async fn write_to_stdout(
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

async fn write_to_file(
    filename: &str,
    mut res: Response<Incoming>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create file.
    let mut file = File::create(filename).await?;

    // Write all frames to file.
    while let Some(frame) = res.body_mut().frame().await {
        let frame = frame?;
        if let Some(d) = frame.data_ref() {
            file.write_all(d).await?;
        }
    }

    file.sync_all().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    let https = HttpsConnector::new();
    let main_url = "https://github.com/hyperium/hyper/blob/master/examples/client.rs";
    let url = main_url.parse::<hyper::Uri>()?;

    // Create the Hyper client
    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);
    let res = client.get(url).await?;
    assert_eq!(res.status(), 200);

    write_to_file("client.html", res).await?;

    Ok(())
}
