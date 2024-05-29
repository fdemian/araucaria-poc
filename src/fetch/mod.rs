pub mod fetch {

    use anyhow::Error;
    use http_body_util::Empty;
    use hyper::body::Bytes;
    use hyper::body::Incoming;
    use hyper::Response;
    use hyper_tls::HttpsConnector;
    use hyper_util::{client::legacy::Client, rt::TokioExecutor};

    pub async fn get_url_contents(main_url: &str) -> Result<Response<Incoming>, Error> {
        // This is where we will setup our HTTP client requests.
        let https = HttpsConnector::new();
        let url = main_url.parse::<hyper::Uri>()?;

        // Create the Hyper client
        let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);
        let res = client.get(url).await?;

        //println!(res.status().string());
        //assert_eq!(res.status(), 200);

        //networking::file::download_file("Atlantis-SOSP.pdf", res).await?;
        //networking::stream::write_to_stdout(res).await?;

        //let string_res = networking::stream::get_body_as_astring(res).await?;
        //let page_string = string_res.as_str();
        //println!("{}", page_string);
        Ok(res)
    }
}
