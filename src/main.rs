use std::net::SocketAddr;
use tiberius::SqlConnection;
use warp::{Filter};

fn main() {
    let conn_str = "";

    
    let future = SqlConnection::connect(conn_str).and_then(|conn| {
        conn.query("SELECT login FROM users where id = @P1",
            &[&1i32]).for_each(|row| {

            let val: i32 = row.get(0);
           
            Ok(())
       })
   });
   


    let routes = warp::path("hello")
            .and(warp::path::param2())
            .and_then(|id: i64| {
                future
            });

    let addr = "0.0.0.0:12345".parse::<SocketAddr>().unwrap();
    warp::serve(routes).run(addr);
}
