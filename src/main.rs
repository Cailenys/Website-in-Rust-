use actix_files as fs;
use actix_web::{web, get,  App, HttpServer, Responder, HttpResponse};

async fn home() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello, world!</h1>")
}
#[get("/info")]
async fn get_info() -> impl Responder {
    HttpResponse::Ok().body("<h1>I am using Rust!</h1>") 
}


#[actix_web::main] // Main Function to Start the Web Server
async fn main() { // Defines the main function that runs the web server asynchronously.
    println!("Web Server starting on port: 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new() // Creates a new instance of the Actix web application.
         .service(get_info) // Registers the get_info function to handle /info route.
         .service(fs::Files::new("/", "src/public") //Point to the src/public folder where the web files (HTML, CSS, JS, etc.) are stored.
         .index_file("index.html") )// Sets index.html as the default file that loads when the user visits the root (/).
         //.route("/", web::get().to(home)) // Maps the root path (/) to the home function.
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}