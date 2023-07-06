use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder}; 

#[get("/")]
async fn hello() -> impl Responder { 
    HttpResponse::Ok().body("Hello world!")
}

#[cfg(test)]
mod tests { 
    use super::*;
    use actix_web::{test, App}; 

    #[actix_web::test]
    async fn test_hello() { 
        let mut app = test::init_service(App::new().service(hello)).await; 
        let req = test::TestRequest::get().uri("/").to_request(); 
        let resp = test::call_service(&mut app, req).await; 
        assert!(resp.status().is_success()); 
        let resp_body = test::read_body(resp).await;
        assert_eq!(resp_body, "Hello world!");
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder { 
    HttpResponse::Ok().body(req_body)
}

#[cfg(test)]
mod tests_post { 
    use super::*;
    use actix_web::{test, App}; 

    #[actix_web::test]
    async fn test_echo() { 
        let mut app = test::init_service(App::new().service(echo)).await; 
        let req = test::TestRequest::post().uri("/echo").set_payload("Hello!").to_request(); 
        let resp = test::call_service(&mut app, req).await; 
        assert!(resp.status().is_success()); 
        let resp_body = test::read_body(resp).await;
        assert_eq!(resp_body, "Hello!");
    }
}

async fn manual_hello() -> impl Responder { 
    HttpResponse::Ok().body("Hey there!")
}

// unit test for manual_hello
#[cfg(test)]
mod tests_manual_hello { 
    use super::*;
    use actix_web::{test, App}; 

    #[actix_web::test]
    async fn test_manual_hello() { 
        let mut app = test::init_service(App::new().route("/hey", web::get().to(manual_hello))).await; 
        let req = test::TestRequest::get().uri("/hey").to_request(); 
        let resp = test::call_service(&mut app, req).await; 
        assert!(resp.status().is_success()); 
        let resp_body = test::read_body(resp).await;
        assert_eq!(resp_body, "Hey there!");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    HttpServer::new(|| { 
        App::new()
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))? 
    .run()
    .await
}