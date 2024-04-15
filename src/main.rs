use core::panic;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn main() {
    let match_result = clap::Command::new("addressbook")
        .version("0.1")
        .author("Tyler James Hutchinson")
        .arg(
            clap::Arg::new("input")
                .short('i')
                .long("input")
                .help("path to input file")
                .required(true),
        )
        .arg(
            clap::Arg::new("output")
                .short('o')
                .long("output")
                .help("output path")
                .required(true),
        )
        .get_matches();

    let file_path = PathBuf::from(
        match_result
            .get_one::<String>("input")
            .expect("Failed to provide Input file"),
    );

    let out_path = PathBuf::from(
        match_result
            .get_one::<String>("output")
            .expect("Failed to resolve output destination"),
    );

    let mut input_records: Vec<InputDoc> = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .from_path(file_path)
        .expect("Failed to Read Input csv");

    for records in reader.deserialize::<InputDoc>() {
        match records {
            Ok(x) => input_records.push(x),
            Err(_) => panic!("Failed to parse CSV input"),
        }
    }

    let writer = csv::Writer::from_path(out_path);

    let mut counter: i32 = 100;

    match writer {
        Ok(mut x) => {
            for record in input_records {
                x.serialize(OutputDoc {
                    address: "data".to_string(),
                    searchid: format!("{}", &counter),
                    name: format!("{} {}", record.first_name, record.last_name),
                    search_string: format!("{} {}", record.first_name, record.last_name)
                        .chars()
                        .take(10)
                        .collect::<String>(),
                    category_id: "1".to_string(),
                    frequently_used: "FALSE".to_string(),
                    mail_address: record.email_address,
                    fax_number: record.tel_number,
                    ifax_address: "".to_string(),
                    ftp_host: "".to_string(),
                    ftp_directory: "".to_string(),
                    ftp_username: "+xS4FiNvCE4i8EqfPNhjWg==".to_string(),
                    user_encoding_method: "encrypted2".to_string(),
                    ftp_password: "+xS4FiNvCE4i8EqfPNhjWg==".to_string(),
                    password_encoding_method: "encrypted2".to_string(),
                    smb_directory: "".to_string(),
                    smb_username: "+xS4FiNvCE4i8EqfPNhjWg==".to_string(),
                    smb_user_encoding: "encrypted2".to_string(),
                    smb_password: "+xS4FiNvCE4i8EqfPNhjWg==".to_string(),
                    smb_password_encoding: "encrypted2".to_string(),
                    desktop_host: "".to_string(),
                    desktop_port: "".to_string(),
                    desktop_directory: "".to_string(),
                    desktop_username: "+xS4FiNvCE4i8EqfPNhjWg==".to_string(),
                    desktop_username_encoding: "encrypted2".to_string(),
                    desktop_password: "+xS4FiNvCE4i8EqfPNhjWg==".to_string(),
                    desktop_password_encoding: "encrypted2".to_string(),
                })
                .unwrap();

                counter = &counter + 1;
            }
            x.flush().expect("failed to flush writer");
        }
        Err(e) => panic!("{:?}", e),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct InputDoc {
    #[serde(rename = "First Name")]
    first_name: String,
    #[serde(rename = "Last Name")]
    last_name: String,
    #[serde(rename = "Email Address")]
    email_address: String,
    #[serde(rename = "Tel Number")]
    tel_number: String,
    #[serde(rename = "2nd Fax Number")]
    tel_2_number: String,
    #[serde(rename = "IPFax Destination")]
    ipfax_destination: String,
    #[serde(rename = "Facsimile Mode")]
    facsimile_mode: String,
    #[serde(rename = "Company")]
    company: String,
    #[serde(rename = "Department")]
    department: String,
    #[serde(rename = "Keyword")]
    keyword: String,
    #[serde(rename = "Furigana")]
    furigana: String,
    #[serde(rename = "SUB")]
    sub: String,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "SEP")]
    sep: String,
    #[serde(rename = "PWD")]
    pwd: String,
    #[serde(rename = "ECM")]
    ecm: String,
    #[serde(rename = "Line Select")]
    line_select: String,
    #[serde(rename = "Quality Transmit")]
    quality_transmit: String,
    #[serde(rename = "Transmission Type")]
    transmission_type: String,
    #[serde(rename = "Attenuation")]
    attenuation: String,
    #[serde(rename = "FavoriteFax")]
    favoritefax: String,
    #[serde(rename = "FavoritEmail")]
    favoritemail: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OutputDoc {
    address: String,
    #[serde(rename = "search-id")]
    searchid: String,
    name: String,
    #[serde(rename = "search-string")]
    search_string: String,
    #[serde(rename = "category-id")]
    category_id: String,
    #[serde(rename = "frequently-used")]
    frequently_used: String,
    #[serde(rename = "mail-address")]
    mail_address: String,
    #[serde(rename = "fax-number")]
    fax_number: String,
    #[serde(rename = "ifax-address")]
    ifax_address: String,
    #[serde(rename = "ftp-host")]
    ftp_host: String,
    #[serde(rename = "ftp-directory")]
    ftp_directory: String,
    #[serde(rename = "ftp-username")]
    ftp_username: String,
    #[serde(rename = "ftp-username/@encodingMethod")]
    user_encoding_method: String,
    #[serde(rename = "ftp-password")]
    ftp_password: String,
    #[serde(rename = "ftp-password/@encodingMethod")]
    password_encoding_method: String,
    #[serde(rename = "smb-directory")]
    smb_directory: String,
    #[serde(rename = "smb-username")]
    smb_username: String,
    #[serde(rename = "smb-username/@encodingMethod")]
    smb_user_encoding: String,
    #[serde(rename = "smb-password")]
    smb_password: String,
    #[serde(rename = "smb-password/@encodingMethod")]
    smb_password_encoding: String,
    #[serde(rename = "desktop-host")]
    desktop_host: String,
    #[serde(rename = "desktop-port")]
    desktop_port: String,
    #[serde(rename = "desktop-directory")]
    desktop_directory: String,
    #[serde(rename = "desktop-username")]
    desktop_username: String,
    #[serde(rename = "desktop-username/@encodingMethod")]
    desktop_username_encoding: String,
    #[serde(rename = "desktop-password")]
    desktop_password: String,
    #[serde(rename = "desktop-password/@encodingMethod")]
    desktop_password_encoding: String,
}
