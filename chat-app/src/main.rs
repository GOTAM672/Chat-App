// Import all the rocket macros globally
#[macro_use] extern crate rocket;

#[get("/world")]   // get request to world path
// handler function
fn world() -> &'static str {
     "Hello, world!"
}

#[launch]

fn rocket() -> _ {
      rocket::build().mount("/hello", routes![world]) // To create new rocket server instance and mount the route.
}
                
