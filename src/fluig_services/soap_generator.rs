use super::FluigConfig;
use std::{path::Path, fs, io::prelude::*};
use base64::{engine::general_purpose, Engine as _};
use same_file::is_same_file;

pub fn get_soap_body(config: FluigConfig) -> String
{
    let soap_body = format!(r##"
    <soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ws="http://ws.dm.ecm.technology.totvs.com/">
        <soapenv:Header/>
        <soapenv:Body>
            <ws:updateSimpleCardIndex>
                <username>{conf_name}</username>
                <password>{conf_pass}</password>
                <companyId>{conf_comp}</companyId>
                <documentId>{conf_doc}</documentId>
                <publisherId>{conf_pub}</publisherId>
                <cardDescription>{conf_desc}</cardDescription>
                <descriptionField>Editado via Soap</descriptionField>
                <Attachments>
                    {conf_item}
                </Attachments>
                <customEvents>
                    {conf_event}
                </customEvents>
            </ws:updateSimpleCardIndex>
        </soapenv:Body>
    </soapenv:Envelope>
    "##,
        conf_name = config.username,
        conf_pass = config.password,
        conf_comp = config.companyId,
        conf_doc = config.documentId,
        conf_pub = config.publisherId,
        conf_desc = config.formName,
        conf_item = get_item_list(config.clone()),
        conf_event =  get_event_list(config.clone())
    );

    return soap_body.to_string();
}

fn get_item_list(config: FluigConfig) -> String
{
    let mut items = String::new();

    let home_path = Path::new("./");

    items += &get_item_list_from_dir(config, home_path);

    return items;
}

fn get_item_list_from_dir(config: FluigConfig, dir: &Path) -> String
{
    let mut items = String::new();

    let dir_path = fs::read_dir(dir).unwrap();

    for path in dir_path
    {
        let file_path = path.unwrap().path();

        let index = is_same_file(file_path.as_path(), Path::new(&config.principal)).unwrap_or(false);

        if !is_ignore(file_path.as_path(), config.ignore.clone()) 
        {
            if file_path.is_file()
            {
                items += &get_item(file_path.as_path(), index);
            }
            else
            {
                items += &get_item_list_from_dir(config.clone(), &file_path);
            }
        }
    }

    return items;
}

fn get_item(path: &Path, principal: bool) -> String
{
    let file = Path::new(path);

    let file_name = file.file_name().unwrap().to_string_lossy();

    let mut file_content = Vec::new();

    match fs::File::open(path).unwrap().read_to_end(&mut file_content) {
        Ok(_) => {},
        Err(_) => {return "".to_string(); }
    };

    let item = format!("
        <item>
            <attach>{}</attach>
            <fileName>{}</fileName>
            <fileSize>{}</fileSize>
            <filecontent>{}</filecontent>
            <pathName>{}</pathName>
            <principal>{}</principal>
        </item>",
        !principal,
        file_name,
        file.metadata().unwrap().len(),
        general_purpose::STANDARD.encode(&file_content),
        file.parent().unwrap().display(),
        principal
    );

    return item;
}

fn get_event_list(config: FluigConfig) -> String
{
    let mut items = String::new();

    let dir_path = match fs::read_dir("./events")
    {
        Ok(value) => value,
        Err(_) => {return "".to_string();}
    };

    for path in dir_path
    {
        let file_path = path.unwrap().path();

        if !is_ignore(file_path.as_path(), config.ignore.clone()) && file_path.is_file() && file_path.extension().unwrap() == "js"
        {
            items += &get_event(file_path.as_path());
        }
    }

    return items;
}

fn get_event(path: &Path) -> String
{
    let file = Path::new(path);

    let file_name = file.file_stem().unwrap().to_string_lossy();

    let file_content = match fs::read_to_string(file)
    {
        Ok(value) => value,
        Err(_) => {return "".to_string();} 
    };

    let item = format!("
        <item>
            <eventDescription><![CDATA[{}]]></eventDescription>
            <eventId>{}</eventId>
            <eventVersAnt>false</eventVersAnt>
        </item>",
        file_content,
        file_name
    );

    return item;
}

fn is_ignore(path: &Path, mut ignore_list: Vec<String>) -> bool
{
    let mut default_ignore:Vec<String> = vec![
        "./events".to_string()
    ];
    ignore_list.append(&mut default_ignore);

    for (_, ignore_path) in ignore_list.clone().iter().enumerate()
    {
        if is_same_file(path, Path::new(&ignore_path)).unwrap_or(false) {
            // ignore_list.remove(i);
            return true;
        }
    }

    return false;
}