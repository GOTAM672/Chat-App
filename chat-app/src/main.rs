// Import all the rocket macros globally
#[macro_use] extern crate rocket;

use rocket::tokio::sync::broadcast::{channel};

#[get("/world")]   // get request to world path
// handler function
fn world() -> &'static str {
     "Hello, world!"
}

#[launch]

fn rocket() -> _ {
      rocket::build() 
          .manage(channet::<Message>(1024).0)       // The manage method allows to add state to our rocket server instance
          .mount("/hello", routes![world])         // To create new rocket server instance and mount the route.
}
                
