use hyper::body::Incoming;
use hyper::{Method, Response};
use jsonrpsee::server::{RpcModule, Server};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

// Storage and streaming mods.
pub mod fetch;
pub mod storage;
pub mod stream;
pub mod url;

const PING_STR: &str = "Hello there!!";
const PARAMS_ERROR: &str = r#"
{
   "status": "400",
   "error": "BAD_REQUEST",
   "message": "Request must include url parameter."
}"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");

    // Start up a JSON-RPC server that allows cross origin requests.
    let server_addr = run_server().await?;

    // Print instructions for testing CORS from a browser.
    println!("Run the following snippet in the developer console in any Website.");
    println!(
        r#"
       fetch("http://{}", {{
           method: 'POST',
           mode: 'cors',
           headers: {{ 'Content-Type': 'application/json' }},
           body: JSON.stringify({{
               jsonrpc: '2.0',
               method: 'say_hello',
               id: 1
           }})
       }}).then(res => {{
           console.log("Response:", res);
           return res.text()
       }}).then(body => {{
           console.log("Response Body:", body)
       }});
    "#,
        server_addr
    );
    futures::future::pending().await
}

async fn run_server() -> anyhow::Result<SocketAddr> {
    // Add a CORS middleware for handling HTTP requests.
    // This middleware does affect the response, including appropriate
    // headers to satisfy CORS. Because any origins are allowed, the
    // "Access-Control-Allow-Origin: *" header is appended to the response.
    let cors = CorsLayer::new()
        // Allow `POST` when accessing the resource
        .allow_methods([Method::POST])
        // Allow requests from any origin
        .allow_origin(Any)
        .allow_headers([hyper::header::CONTENT_TYPE]);
    //let middleware = tower::ServiceBuilder::new().layer(cors);

    // The RPC exposes the access control for filtering and the middleware for
    // modifying requests / responses. These features are independent of one another
    // and can also be used separately.
    // In this example, we use both features.
    /*let server = Server::builder()
        .set_http_middleware(middleware)
        .build("127.0.0.1:0".parse::<SocketAddr>()?)
        .await?;

    */

    let server = Server::builder().build("127.0.0.1:9944").await?;
    let mut module = RpcModule::new(());

    module.register_method("say_hello", |_, _| {
        println!("say_hello method called!");
        return serde_json::json!({
           "ok": true,
           "message": PING_STR,
        });
    })?;

    // Network methods.
    module.register_async_method("download_file", |params, _| async move {
        println!("donwload method called!");
        let kv: &str = params.as_str().unwrap();
        let parsed_params: serde_json::Value = serde_json::from_str(kv).unwrap();
        let url: &str = parsed_params.get("url").unwrap().as_str().unwrap();
        let path: &str = parsed_params.get("path").unwrap().as_str().unwrap();
        println!("{}", url);

        let filename: &str = url::utils::get_file_url(url).await.unwrap();
        let file_contents: Response<Incoming> = fetch::fetch::get_url_contents(url).await.unwrap();
        let path: String = storage::file::store_response_as_file(filename, path, file_contents)
            .await
            .unwrap();

        return serde_json::json!({
           "status": 200,
           "path": path,
           "filename": filename,
           "size": 224
        });
    })?;

    module.register_async_method("get_page", |params, _| async move {
        println!("get_page method called!");
        let kv: &str = params.as_str().expect(PARAMS_ERROR);
        let parsed_params: serde_json::Value = serde_json::from_str(kv).unwrap();
        let url: &str = parsed_params.get("url").unwrap().as_str().unwrap();
        let response: Response<Incoming> = fetch::fetch::get_url_contents(url).await.unwrap();
        let page_contents: String = stream::stream::get_content_as_astring(response)
            .await
            .unwrap();

        return serde_json::json!({
           "status": 200,
           "text": page_contents
        });
    })?;

    module.register_async_method("store_value", |params, _| async move {
        println!("store_value method called!");
        let kv: &str = params.as_str().unwrap();
        let parsed_params: serde_json::Value = serde_json::from_str(kv).unwrap();
        let key: &str = parsed_params.get("key").unwrap().as_str().unwrap();
        let value: &str = parsed_params.get("value").unwrap().as_str().unwrap();

        let retval = storage::key_value::store_value(key, value).await;
        return retval;
    })?;

    let addr = server.local_addr()?;
    let handle = server.start(module);

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    tokio::spawn(handle.stopped());

    Ok(addr)
}
