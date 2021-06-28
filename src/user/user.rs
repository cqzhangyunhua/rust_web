
use std::any::type_name;
use std::result;
use std::collections::HashMap; 

use json::object;
use mysql::*;
use mysql::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::json;



#[derive(Debug, PartialEq, Eq)]
pub struct User {
    id: i32,
    username: Option<String>,
    passwd: Option<String>,
    
}
/*
CREATE TABLE `user` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` char(50) DEFAULT NULL,
  `passwd` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
*/

//必需要实现接口FromRow
impl FromRow  for User{
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        
        match Self::from_row_opt(row) {
            Ok(x) => x,
            Err(FromRowError(row)) => panic!(
                "Couldn't convert {:?} to type {}. (see FromRow documentation)",
                row,
                type_name::<Self>(),
            ),
        }
    }   
    fn from_row_opt(row: Row) -> result::Result<Self, FromRowError>
    where
        Self: Sized {
             let t_id:Option<i32> = row.get(0);
             let id=match t_id {
                None =>0,
                Some(v) =>v
        
            };
             let username:Option<String> = row.get(1);
             let passwd:Option<String> = row.get(2);           
             Ok(User { id, username, passwd,})
       // todo!()
    } 
}



impl User {

    pub  fn getUserMore(mut conn:PooledConn,id:i32) ->String {
        
        let stmt = conn.prep("SELECT id,username,passwd FROM user where id>=? limit 0,10").unwrap();
       
        let val: Vec<(i32,String,String)> = conn.exec(&stmt ,(1,)).unwrap();
        let val_str=vec![String::from("id"), String::from("username"),String::from("passwd")];
         
        let mut data = object!{
            foo: false,
            bar: null,
            answer: 42,
            list: []
        };
        
        //println!("{:?}",val);   
        //转成JSON输出    
        let code = 200;
        let value = json!({
            "code": code,
            "success": code == 200,
            "payload": {
               // "data": scores,
            }
        });
        
       
        //value.as_object_mut().unwrap().into_iter();
        let a=value.to_string();
        return a;
        //serde_json::to_string(&val);
        // let j="rrrr".to_string();//serde_json::to_string(&val).unwrap();
        // return j;
        // println!("{}",&j);
    }

    pub  fn getUserMore_index(mut conn:PooledConn,id:i32) ->String {
        
        let stmt = conn.prep("SELECT id,username,passwd FROM user where id>=? limit 0,10").unwrap();
       
        let val: Vec<(i32,String,String)> = conn.exec(&stmt ,(1,)).unwrap();
        //println!("{:?}",val);   
        //转成JSON输出    
        let mut user_arr = json::JsonValue::new_array();
        for u in  val{
            //println!("id:{:?}",u);
             let mut data = json::JsonValue::new_object();
             data["id"] =u.0.into();
             data["username"] =u.1.into();
             data["passwd"] = u.2.into();
             user_arr.push(data);
           // println!("id:{},username:{}",u.0.into(),u.1.into());
           
        }
        
        let a=user_arr.dump();
        return a;
        //serde_json::to_string(&val);
        // let j="rrrr".to_string();//serde_json::to_string(&val).unwrap();
        // return j;
        // println!("{}",&j);
    }
    pub  fn getUserMore_User(mut conn:PooledConn,id:i32) ->String {
        
        let stmt = conn.prep("SELECT id,username,passwd FROM user where id>=? limit 0,10").unwrap();
        //必需要实现接口FromRow
        let val: Vec<User> = conn.exec(&stmt ,(1,)).unwrap();
        println!("{:?}",val);   
        //转成JSON输出    
        let mut user_arr = json::JsonValue::new_array();
        for u in  val{           
            let username_s=match &u.username {
                None =>"None",
                Some(v) =>v
        
            };
            let mut data = json::JsonValue::new_object();
            data["id"] = u.id.into();             
            data["username"] = username_s.into();
            data["passwd"] = u.passwd.into();
            user_arr.push(data);
            println!("id:{},username:{}",u.id,username_s);
        }
        let a=user_arr.dump();
        return a;
        //serde_json::to_string(&val);
        // let j="rrrr".to_string();//serde_json::to_string(&val).unwrap();
        // return j;
        // println!("{}",&j);
    }
    pub  fn getUserMore_ok(mut conn:PooledConn,id:i32) ->String {
        
        let stmt = conn.prep("SELECT id,username,passwd FROM user where id>=? limit 0,10").unwrap();
        let val: Vec<(i32,String,String)> = conn.exec(&stmt ,(1,)).unwrap();
        println!("{:?}",val);
        let j=serde_json::to_string(&val).unwrap();
        return j;
        // println!("{}",&j);
    }
    pub  fn getUserOne_ok(mut conn:PooledConn,id:i32)  {
        
        let stmt = conn.prep("SELECT id,username,passwd FROM user where id=? limit 0,1").unwrap();
        let val: Option<(i32,String,String)> = conn.exec_first(&stmt ,(2,)).unwrap();
        println!("{:?}",val);
    }
    pub fn getuser(mut conn:PooledConn) -> String {
        let selected_payments = conn
        .query_map(
            "SELECT id, username, passwd from User",
            |(id, username, passwd)| {
                User { id, username, passwd }
            },
        ).unwrap();
        let mut data_arr = json::JsonValue::new_array();
        for i in &selected_payments {
            
        //  println!("{:}", i.customer_id);
            let mut data = json::JsonValue::new_object();
            data["id"] = i.id.into();
            let b=match &i.username {
                None =>"None",
                Some(v) =>v

            };
            data["username"] = b.into();
            let c=match &i.passwd {
                None =>"None",
                Some(v) =>v

            };
            data["passwd"] = c.into();
            data_arr.push(data);
        }
        let a=data_arr.dump();
        return a;
        }
}