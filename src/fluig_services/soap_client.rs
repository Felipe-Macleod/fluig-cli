use super::{FluigConfig, soap_generator};
use std::error::Error;
use hyper::{Request, Uri, Body, Method, Client, Response};
use hyper_tls::HttpsConnector;

pub fn generate_request(config: FluigConfig) -> Request<Body>
{
    let wsdl_server = format!("{}/webdesk/ECMCardIndexService?wsdl", config.server);
    
    let url = wsdl_server.parse::<Uri>().unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", "text/xml")
        .header("SOAPAction", "\"updateSimpleCardIndex\"")
        .body(Body::from(soap_generator::get_soap_body(config))).unwrap();

    return req;
}

pub async fn send_request(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error>>
{
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let res = client.request(req).await?;

    Ok(res)
}