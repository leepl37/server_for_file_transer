#![allow(dead_code)]
#[allow(unused_imports)]
use super::schema::{info_of_action, admin, info_of_location, users};
// use argonautica::Hasher;
use diesel::{Insertable, Queryable};
use dotenv::dotenv;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize)]
pub struct Admin {
    pub admidx: i32,
    pub admid: String,
    pub admpw: String,
    pub admname: Option<String>,
    pub admmemo: Option<String>,
    pub admregdate: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "admin"]
pub struct NewAdmin {
    pub admid: String,
    pub admpw: String,
    pub admname: Option<String>,
    pub admmemo: Option<String>,
}
impl NewAdmin {
    pub fn new(aid: String, apw: String, name: Option<String>, memo: Option<String>) -> Self {
        dotenv().ok();
        let h = bcrypt::hash(apw).unwrap();
        NewAdmin {
            admid: aid.to_string(),
            admpw: h,
            admname: name,
            admmemo: memo,
        }
    }
    pub fn check(aid: String, apw: String) -> Self {
        NewAdmin {
            admid: aid.to_string(),
            admpw: apw.to_string(),
            admname: None,
            admmemo: None,
        }
    }
}

#[derive(Debug, Queryable, Serialize)]
pub struct Users {
    pub useridx: i32,
    pub username: String,
    pub userid: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCheck {
    pub id: String,
    pub pw: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUsers {
    pub username: Option<String>,
    pub userid: String,
    pub password: String,
}

impl NewUsers {
    pub fn new(uid: String, password: String, username: Option<String>) -> Self {
        dotenv().ok();
        let h = bcrypt::hash(password).unwrap();
        NewUsers {
            username,
            userid: uid,
            password: h,
        }
    }
    pub fn check(uid: String, password: String) -> Self {
        NewUsers {
            username: None,
            userid: uid,
            password,
        }
    }
}

#[derive(Debug, Queryable, Serialize)]
pub struct ActionInfo {
    pub idx: i32,
    pub userid: String,
    pub flapid: String,
    pub action: String,
    pub memo: Option<String>,
    pub insdaytime: String,
    pub f_file_ip: String,
    pub site_idx: i32,
    pub result:Option<String>,
    pub reason:Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "info_of_action"]
pub struct NewActionInfo {
    pub userid: String,
    pub flapid: String,
    pub action: String,
    pub memo: Option<String>,
    pub f_file_ip: String,
    pub site_idx: i32,
    pub result:Option<String>,
    pub reason:Option<String>,
}
impl NewActionInfo {
    pub fn new(
        userid: String,
        flapid: String,
        action: String,
        memo: Option<String>,
        f_file_ip: String,
        site_idx_action: i32,
        result:Option<String>,
        reason:Option<String>,
    ) -> Self {
        NewActionInfo {
            userid,
            flapid,
            action,
            memo,
            f_file_ip,
            site_idx: site_idx_action,
            result,
            reason
        }
    }
}

#[derive(Debug, Queryable, Serialize)]
pub struct SiteInfo {
    pub site_idx: i32,
    pub site_id: String,
    pub site_pw: String,
    pub site_name: String,
    pub site_type: Option<String>,
    pub open_date: Option<chrono::NaiveDateTime>,
    pub regi_date: Option<chrono::NaiveDateTime>,
    pub exe_date: Option<chrono::NaiveDateTime>,
    pub site_system: Option<String>,
    pub site_name2: Option<String>,
    pub site_ip: String,
    pub site_ping: Option<String>,
    pub site_memo: Option<String>,
    pub site_router: Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "info_of_location"]
pub struct NewSiteInfo {
    pub site_id: String,
    pub site_pw: String,
    pub site_name: String,
    pub site_type: Option<String>,
    pub open_date: Option<chrono::NaiveDateTime>,
    pub regi_date: Option<chrono::NaiveDateTime>,
    pub exe_date: Option<chrono::NaiveDateTime>,
    pub site_system: Option<String>,
    pub site_name2: Option<String>,
    pub site_ip: String,
    pub site_ping: Option<String>,
    pub site_memo: Option<String>,
    pub site_router: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct f_fileInfo {
    pub userid: String,
    pub siteidx: String,
    pub flapnumber: String,
    pub action: String,
    pub memo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct f_fileFail {
    pub r#type:String,
    pub msg:String,
    pub result:bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonReturn {
    pub r#type: Option<String>,
    pub result: bool,
}
impl JsonReturn {
    pub fn return_result(name: Option<String>) -> Self {
        match name {
            None => JsonReturn {
                r#type: None,
                result: false,
            },
            Some(u) => {
                let mut name = String::from("");
                if u.contains("admin") {
                    name = "admin".to_string();
                } else if u.contains("f_file") {
                    name = "f_file".to_string();
                } else if u.contains("signup") {
                    name = "user".to_string();
                } else {
                    name = "user".to_string();
                }
                JsonReturn {
                    r#type: Some(name),
                    result: true,
                }
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct f_fileIpIdPw {
    pub ip:String,
    pub id:String,
    pub pw:String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct f_fileInsert {
    pub ip:String,
    pub id:String,
    pub pw:String,
    pub flap_number:String,
    pub action:String

}

