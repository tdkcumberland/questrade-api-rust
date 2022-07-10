use questtrade_api::test_api;
use reqwest::Client;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn integration_tests(){

    let client = Client::new();
    let response = test_api("get", client, "https://httpbin.org/ip").await;

    match response {
        Ok(rep) => print!("{}", rep.text().await.unwrap_or_default()),
        Err(_) => panic!("trouble")
    }
    
}
