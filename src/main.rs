use ini::Ini;
use chrono::{Local, DateTime};

#[macro_use]extern crate prettytable;
use prettytable::{Table, format};

fn main() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["AWS_PROFILE","⏰","Expire Time","AWS Account","Rolename"]);

    let mut cred_file = dirs::home_dir().unwrap();
    cred_file.push(".aws/credentials") ; 
    let cred = Ini::load_from_file(cred_file).unwrap();

    for (sec, prop) in cred.iter() {
        let acc ;
        let rolename ;
        let expires ;

        if prop.contains_key("x_principal_arn") {
            let arn = prop.get("x_principal_arn").unwrap();
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
    
}

