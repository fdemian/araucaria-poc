use http_body_util::{BodyExt, Empty};
use hyper::body::{Body, Bytes};
//use hyper::body::Incoming;
//use hyper::Response;
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
//use regex::Regex;
use tokio::io::{self, AsyncWriteExt as _};

pub mod networking {
    pub mod file {
        use http_body_util::BodyExt;
        use hyper::body::Incoming;
        use hyper::Response;
        use tokio::fs::File;
        use tokio::io::AsyncWriteExt as _;

        pub async fn download_file(
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
    }

    pub mod stream {
        use std::io::Read;

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
}

/*
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
}*/

/*
async fn get_file_url(url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let re = Regex::new(r"(http)s^::/(d+)/(.+)$").unwrap();
    let mut results = vec![];
    for (_, [path, lineno, line]) in re.captures_iter(url).map(|c| c.extract()) {
        results.push((path, lineno.parse::<u64>()?, line));
    }
    Ok(())
    }*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    let https = HttpsConnector::new();
    let main_url =
        "https://www.microsoft.com/en-us/research/wp-content/uploads/2011/10/Atlantis-SOSP.pdf";
    let url = main_url.parse::<hyper::Uri>()?;

    // Create the Hyper client
    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);
    let res = client.get(url).await?;
    assert_eq!(res.status(), 200);

    //networking::file::download_file("Atlantis-SOSP.pdf", res).await?;

    let pageString = networking::stream::get_body_as_astring(res).await?;
    //write_to_file("Atlantis-SOSP.pdf", res).await?;

    Ok(())
}
