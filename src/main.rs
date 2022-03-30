use ini::Ini;
use chrono::{Local, DateTime};

#[macro_use]extern crate prettytable;
use prettytable::{Table, format};
use std::env;
use std::path::PathBuf;

fn main() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["AWS_PROFILE","⏰","Expire Time","AWS Account","Rolename"]);

    let mut cred_file = dirs::home_dir().unwrap();
    cred_file.push(".aws/credentials") ; 
    let cred_file = match env::var_os("AWS_SHARED_CREDENTIALS_FILE") {
        Some(v) => PathBuf::from(v),
        None => cred_file
    };

    let cred = match Ini::load_from_file(cred_file) {
        Ok(ini) => ini,
        Err(error) => panic!("{}",error)
    };

    // iniのセクションをソートしておく
    // TODO: defaultをどうする?
    let mut sections = vec![];
    for (sec, _prop) in cred.iter() {
        sections.push(sec);
    }
    sections.sort();

    for sec in sections {
        let prop = match cred.section(sec.to_owned()) {
            Some(p) => p,
            None => panic!()
        };
        let acc ;
        let rolename ;
        let expires ;

        if prop.contains_key("x_principal_arn") {
            let arn = match prop.get("x_principal_arn") {
                Some(v) => v,
                None => panic!()
            };
            let valid ;
            acc = arn.split(":").nth(4).unwrap();
            rolename = arn.split("/").nth(1).unwrap();
            expires = DateTime::parse_from_rfc3339(prop.get("x_security_token_expires").unwrap()).unwrap();
            if Local::now() > expires {
                valid = "❌";
            } else {
                valid = "✅";
            }
            table.add_row(row![sec.unwrap(),valid,expires,acc,rolename]);

        } else {
            table.add_row(row![sec.unwrap(),"❓","","Unknown",""]);
        }

    }
    table.printstd();
    // TODO: exit codeを考慮しなくてもよい?
}
