#![allow(dead_code)]
#[allow(unused_imports)]
use crate::functions::{f_file_data_db_insert, get_action_data, get_admin_date, get_site_data, login, login_admin, sign_up, sign_up_admin, get_the_info_of_location};
use crate::models::{f_fileInfo, JsonReturn, NewActionInfo, NewAdmin, NewUsers, UserCheck, f_fileFail, f_fileInsert};
use actix_web::web::Query;
use actix_web::{get, HttpResponse, Responder, post};
use crate::functions::{address_url, return_httpresponse, db_insert, usefunction_insert_f_file, usefunction_login_check, function_signup, function_signup_admin};
use std::error::Error;


#[get("/")]
pub async fn gwanho() -> impl Responder {
            //cors 설정하기.
            HttpResponse::Ok()
                .set_header("Access-Control-Allow-Origin", "*")
                .set_header("Access-Control-Allow-Origin", "*")
                .json("hello G_wan")
}

/*
내용:
 해당 주소로 요청한 데이터값이 들어오게 되면 데이터값을 통해 DB에서 필요한 데이터를 가져와 새로운 함수를 구현.
 요청한 데이터값이 들어오지 않으면 자동으로 해당 값이 없다고 반환한다.
 */
#[post("/api/f_file")]
pub async fn insert_f_file_post(info: Query<f_fileInfo>) -> impl Responder {
    //models.rs 에 만들어둔 struct(f_fileInfo) 에 into_inner() 를 통해 주입한다.
    let f_file_info = info.into_inner();
    //functions.rs 에 구현된 usefunction_insert_f_file 라는 함수를 사용하여 데이터 값 비교 후
    //  response 형태로 리턴.
    let response = usefunction_insert_f_file(f_file_info);
    response
}


#[post("/api/login")]
pub async fn login_check_post(info: Query<UserCheck>) -> impl Responder {
    let check = info.into_inner();
    let response = usefunction_login_check(check);
    response
}


#[post("/api/signup")]
pub async fn signup_post(info: Query<NewUsers>) -> impl Responder {
    let users_info = info.into_inner();
    let response = function_signup(users_info);
    response
}


#[post("/api/signup/admin")]
pub async fn signup_admin_post(info: Query<NewAdmin>) -> impl Responder {
    let admin_user = info.into_inner();
    let response = function_signup_admin(admin_user);
    response
}
