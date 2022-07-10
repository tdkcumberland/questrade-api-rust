use reqwest::{Response, Error, RequestBuilder, Client};




fn build_request(method: &str, client: Client, url: &str) -> RequestBuilder {
    let request_method = match method {
        "get" => client.get(url),
        _ => panic!("nope")
    };

    request_method
}

pub async fn test_api(method: &str, client: Client, url: &str) -> Result<Response, Error> {
    build_request(method, client, url).send().await
}


#[cfg(test)]
mod tests {}
