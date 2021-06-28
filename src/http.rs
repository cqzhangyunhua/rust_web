use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use ini::Ini;
use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;

mod  user;






#[derive(Deserialize)]
struct AppState {
    username: String,
}
#[get("/")]
async fn index(pool:web::Data<mysql::Pool>,form:web::Query<AppState>) -> impl Responder {
    println!("{}",form.username);
    let mut conn = pool.get_conn().unwrap();
    //let User=user::user::getuser();
    let User= user::user::User::getUserMore(conn,60);
    //let re=User.dump();
    println!("{:?}",User);
    // let a=User.dump();
        HttpResponse::Ok()
        .content_type("application/json")
        .body(User)
}

#[get("/getuser")]
async fn getuser(pool:web::Data<mysql::Pool>,form:web::Query<AppState>) -> impl Responder {
    
   // let form1==req.query_string();
    println!("{}",form.username);
    let mut conn = pool.get_conn().unwrap();
    //let User=user::user::getuser();
    let User= user::user::User::getuser(conn);
   // println!("{:?}",User);
    //let a=data_arr.dump();
        HttpResponse::Ok()
        .content_type("application/json")
        .body(User)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conf = Ini::load_from_file("config/config.ini").unwrap();
    let section = conf.section(Some("Mysql")).unwrap();
    let mysqllink = section.get("mysqllink").unwrap(); 
    let pool = Pool::new(&mysqllink).unwrap();


    HttpServer::new(move|| {
                    App::new()
                    .data(pool.clone())
                    .service(getuser)
                    .service(index)
            })
            .bind("127.0.0.1:8088")?
            .run()
            .await
}