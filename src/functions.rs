#![allow(dead_code)]
#[allow(unused_imports)]
use crate::models::{ActionInfo, Admin, NewActionInfo, NewAdmin, NewSiteInfo, NewUsers, SiteInfo, Users, f_fileIpIdPw};
use crate::schema::info_of_action::dsl::info_of_action;
use crate::schema::admin::dsl::{admid, admin};
use crate::schema::info_of_location::dsl::info_of_location;
use crate::schema::users::dsl::{userid, username, users};
use diesel::{
    Connection, ConnectionError, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};
use dotenv::dotenv;
use f_file::{f_fileStream, f_fileError};
use pwhash::bcrypt;
use std::error::Error;
use std::str;
use crate::schema::info_of_location::columns::site_idx;
use std::time::Duration;
use actix_web::{Responder, HttpResponse};
use std::net::{TcpStream, TcpListener};
use std::future::Future;
use serde::de::StdError;
use std::net;
use std::thread;
use std::sync::mpsc;
use crate::models::{f_fileFail, f_fileInfo, f_fileInsert, UserCheck, JsonReturn};



//db 정보 불러오기.
pub fn establish_connection() -> Result<MysqlConnection, Box<ConnectionError>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("환경변수에 값이 존재하지 않습니다.");
    match MysqlConnection::establish(&database_url) {
        Ok(connection) => Ok(connection),
        Err(err) => Err(Box::from(err)),
    }
}


pub fn sign_up(form: NewUsers) -> Result<bool, Box<dyn std::error::Error>> {
    let result = diesel::insert_into(users)
        .values(form)
        .execute(&establish_connection()?);
    let i = result?;
    if i == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn sign_up_admin(form: NewAdmin) -> Result<bool, Box<dyn std::error::Error>> {
    let result = diesel::insert_into(admin)
        .values(form)
        .execute(&establish_connection()?);
    let i = result?;
    if i == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn login(form: NewUsers) -> Result<bool, Box<dyn std::error::Error>> {
    dotenv().ok();
    let user = users
        .filter(userid.eq(&form.userid))
        .first::<Users>(&establish_connection().unwrap());

    return match user {
        Ok(u) => {
            let valid = bcrypt::verify(form.password, &u.password);
            if valid {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => Ok(false),
    };
}


pub fn login_admin(form: NewAdmin) -> Result<bool, Box<dyn std::error::Error>> {
    dotenv().ok();
    let admin_result = admin
        .filter(admid.eq(&form.admid))
        .first::<Admin>(&establish_connection().unwrap());
    return match admin_result {
        Ok(u) => {
            let valid = bcrypt::verify(form.admpw, &u.admpw);
            if valid {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Ok(false),
    };
}

pub fn f_file_data_db_insert(form: NewActionInfo) -> Result<bool, Box<dyn std::error::Error>> {
    let result = diesel::insert_into(info_of_action)
        .values(form)
        .execute(&establish_connection().unwrap());

    let i = result?;
    if i == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn get_action_data() -> Result<Vec<ActionInfo>, Box<dyn std::error::Error>> {
    let data: Vec<ActionInfo> = info_of_action.get_results(&establish_connection()?)?;
    Ok(data)
}

pub fn get_admin_date() -> Result<Vec<Admin>, Box<dyn std::error::Error>> {
    let data: Vec<Admin> = admin.get_results(&establish_connection()?)?;
    Ok(data)
}

pub fn get_site_data() -> Result<Vec<SiteInfo>, Box<dyn std::error::Error>> {
    let data: Vec<SiteInfo> = info_of_location.get_results(&establish_connection()?)?;
    Ok(data)
}

pub fn insert_site_date(form: NewSiteInfo) -> Result<bool, Box<dyn std::error::Error>> {
    let result = diesel::insert_into(info_of_location)
        .values(form)
        .execute(&establish_connection()?);
    let i = result?;
    if i == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn get_the_info_of_location(site_index:i32) -> Result<f_fileIpIdPw, Box<diesel::result::Error>> {
    let siteinfo_result = info_of_location.filter(site_idx.eq(site_index))
        .first::<SiteInfo>(&establish_connection().unwrap());
    println!("{:?}", siteinfo_result);
    match siteinfo_result {
        Ok(info) => {
            let f_file = f_fileIpIdPw {
                ip: info.site_ip,
                id: info.site_id,
                pw: info.site_pw
            };
            Ok(f_file)
        }
        Err(err) => {
            Err(Box::new(err))
        }
    }
}


pub fn api_url() -> String {
    dotenv().ok();
    let string = std::env::var("SERVER_URL").expect("can not find api_address");
    string
}

pub fn address_url(addr: &str, var: Option<&str>) -> String {
    let mut url = api_url();
    url.push_str(addr);
    match var {
        None => {}
        Some(v) => {
            let add_var = format!("/{}", v);
            url.push_str(add_var.as_str());
        }
    }
    url
}

pub fn usefunction_login_check(info:UserCheck) -> HttpResponse {
    let users_info = NewUsers::check(info.id.clone(), info.pw.clone());
    let admin_user = NewAdmin::check(info.id.clone(), info.pw.clone());
    let json_return: JsonReturn;
    println!("로그인 테스트{:?}, {:?}",  info.id,info.pw);
    println!("로그인 테스트{:?}",  users);
    match login(users_info) {
        Ok(login_check) => {
            if login_check {
                json_return = JsonReturn::return_result(Some(info.id.clone()));
            } else {
                let admin_result = login_admin(admin_user).unwrap();
                if admin_result {
                    json_return = JsonReturn::return_result(Some("admin".to_string()));
                } else {
                    json_return = JsonReturn::return_result(None);
                }
            }
            HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Credentials", "true")
                .json(json_return)
        }
        Err(err) => {
            let err_str = format!("로그인 중에 에러가 발생하였습니다. 에러코드 {}", err);
            HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Credentials", "true")
                .json(err_str)
        }
    }
}


pub fn usefunction_insert_f_file(info:f_fileInfo) -> HttpResponse {
    let siteidx: i32 = info.siteidx.parse().unwrap();
    let mut ip_addr = String::from("");
    let flap_number = info.flapnumber.clone();
    let action = info.action.clone();
    let info_object = info;

    return match get_the_info_of_location(siteidx) {
        Ok(f_file_object) => {
            let f_file_reqwest_form = f_fileInsert {
            ip: f_file_object.ip.clone(),
            id: f_file_object.id.clone(),
            pw: f_file_object.pw.clone(),
            flap_number,
            action,
            };
        ip_addr = f_file_object.ip.clone();

        let response = reqwest::blocking::Client::new().get(address_url("/api/f_fileupload", None))
        .form(&f_file_reqwest_form)
        .send();

        if response.is_err(){
            let f_file_return = f_fileFail {
            r#type: "서버에러발생".to_string(),
            msg: "error trying to connect: tcp connect error: 대상 컴퓨터에서 연결을 거부했으므로 연결하지 못했습니다. (os error 10061)".to_string(),
            result: false
            };
            return HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Credentials", "true")
                .json(f_file_return)
        }

        let return_f_file_form_json: f_fileFail = response.unwrap().json().unwrap();
        if return_f_file_form_json.result {
            db_insert(info_object, ip_addr.clone(), siteidx, "성공".to_string(), None).unwrap();
        }else{
            db_insert(info_object, ip_addr.clone(), siteidx, "실패".to_string(), Some(return_f_file_form_json.msg.clone())).unwrap();
        }
        return_httpresponse(return_f_file_form_json)
        }
        Err(err) => {
            let mut err_str = format!("info_of_location를 불러오는 과정에서 에러가 발생하였습니다. 에러코드 :{}", err);
            let result1 = db_insert(info_object, ip_addr, siteidx, "실패".to_string(), Some(err_str.clone()));
            match result1 {
                Ok(ok) => {
                    let return_f_file_form_json = f_fileFail { r#type: "f_file".to_string(), msg: err_str.clone(), result: false, };
                    return_httpresponse(return_f_file_form_json)
        }
        Err(err_str) => {
            let return_f_file_form_json = f_fileFail { r#type: "f_file".to_string(), msg: err_str, result: false, };
        return_httpresponse(return_f_file_form_json)
                }
            }
        }
    }
}

pub fn function_signup(info:NewUsers) -> HttpResponse {
    let users_regi = NewUsers::new(info.userid.clone(), info.password.clone(), info.username.clone());
    match sign_up(users_regi){
        Ok(signup_result) => {
            let json_return: JsonReturn;
            if signup_result {
                json_return = JsonReturn::return_result(Some("signup".to_string()));
            } else {
                json_return = JsonReturn::return_result(None);
            }
            HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Credentials", "true")
                .json(json_return)
        }
        Err(err) => {
            let err_str = format!("회원가입 중에 에러가 발생하였습니다. {}", err);
            HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Credentials", "true")
                .json(err_str)
        }
    }
}

pub fn function_signup_admin(info:NewAdmin) -> HttpResponse {
    let admin_regi = NewAdmin::new(info.admid.clone(), info.admpw.clone(), info.admname.clone(), info.admmemo.clone());
    match sign_up_admin(admin_regi){
        Ok(signup_result) => {
            let json_return: JsonReturn;
            if signup_result {
                json_return = JsonReturn::return_result(Some("signup_admin".to_string()));
            } else {
                json_return = JsonReturn::return_result(None);
            }
            HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Credentials", "true")
                .json(json_return)
        }
        Err(err) => {
            let err_str = format!("회원가입 중에 에러가 발생하였습니다. {}", err);
            HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Credentials", "true")
                .json(err_str)
        }
    }
}

pub fn return_httpresponse(obj: f_fileFail) -> HttpResponse {
    HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Credentials", "true")
        .json(obj)
}

pub fn db_insert(info:f_fileInfo, ip_addr:String, site_index:i32, check:String, err_str:Option<String>) -> Result<bool, String> {
    let result = f_file_data_db_insert(NewActionInfo::new(
        info.userid.clone(), info.flapnumber.clone(), info.action.clone(), info.memo.clone(), ip_addr, site_index, Some(check.to_string()), err_str)
    );
    match result {
        Ok(_) => { Ok(true)}
        Err(err) => {
            let err_return = format!("data에 값을 입력하는 중에 에러가 발생되었습니다. 에러코드: {}", err.to_string());
            Err(err_return)
        }
    }
}

#[test]
fn test() {
    let f_file = f_fileIpIdPw {
        ip: "10.127.1.3".to_string(),
        id: "test".to_string(),
        pw: "test".to_string()
    };
    let result = f_file_data_insert(f_file, 19.to_string(), 19.to_string()).unwrap();
    assert_eq!(result, true);
}



