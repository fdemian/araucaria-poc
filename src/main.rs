use http_body_util::Empty;
use hyper::body::Bytes;
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
pub mod storage;
pub mod stream;

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
    let main_url = "https://pbs.twimg.com/media/GOXM4VubsAA-Ibs?format=jpg&name=small";
    //let main_url = "https://stackoverflow.com/questions/27734708/println-error-expected-a-literal-format-argument-must-be-a-string-literal";
    let url = main_url.parse::<hyper::Uri>()?;

    // Create the Hyper client
    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);
    let res = client.get(url).await?;
    storage::file::store_response_as_file("GOXM4VubsAA.jpg", res).await?;

    //println!(res.status().string());
    //assert_eq!(res.status(), 200);

    //networking::file::download_file("Atlantis-SOSP.pdf", res).await?;
    //networking::stream::write_to_stdout(res).await?;

    //let string_res = networking::stream::get_body_as_astring(res).await?;
    //let page_string = string_res.as_str();
    //println!("{}", page_string);
    Ok(())
}
