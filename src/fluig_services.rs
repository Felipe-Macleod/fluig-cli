use serde::Deserialize;

pub mod soap_client;
mod soap_generator;

#[derive(Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct FluigConfig
{
    server: String,
    username: String,
    password: String,
    #[serde(default = "FluigConfig::default_company")]
    companyId: i32,
    documentId: i32,
    publisherId: String,
    formName: String,
    principal: String,
    #[serde(default = "FluigConfig::default_ignore")]
    ignore: Vec<String>
}

impl FluigConfig {
    fn default_company() -> i32 {1}
    fn default_ignore() -> Vec<String> {[].to_vec()}
}