use std::{path::Path, error::Error, fs, fmt};
use fluig_cli::fluig_services::*;

#[derive(Debug)]
struct FileNotFindException(String);

impl fmt::Display for FileNotFindException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Não foi encontrado o arquivo {}", self.0)
    }
}

impl Error for FileNotFindException {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let config = match get_config("fluig.yml")
    {
        Ok(value) => value,
        Err(e) => {println!("{}", e); return Ok(());}
    };

    // println!("{}", config.companyId);

    // println!("{}", get_soap_body(config.clone()));

    let req = soap_client::generate_request(config);

    let res = soap_client::send_request(req).await?;

    println!("status: {}", res.status());

    let buf = hyper::body::to_bytes(res).await?;

    println!("body: {:?}", buf);

    Ok(())
}

fn get_config(path: &str) -> Result<FluigConfig, Box<dyn Error>>
{
    let file = Path::new(&path);

    let config_string = match fs::read_to_string(file) {
        Ok(value) => value,
        Err(_) => { return Err(Box::new(FileNotFindException("fluig.yml".into())));}
    };

    let config: FluigConfig = serde_yaml::from_str(&config_string).unwrap();

    Ok(config)
}

