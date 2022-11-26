// Import all the rocket macros globally
#[macro_use] extern crate rocket;


use rocket::tokio::sync::broadcast::{channel};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::form::Form;
use rocket::serde::{Serialize, Deserialize};

#[get("/world")]   // get request to world path


// handler function
fn world() -> &'static str {
     "Hello, world!"
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]


struct Message{
     /* Message has three fields room, username, message */
 
     #[field(validate = len(..30))]
     pub room: String,
     #[field(validate = len(..20))]
     pub username: String,
     pub message: String,
}

// Implementing end points

/* post messages */

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, Queue: &State<Sender<Message>>) {

         let _res = queue.send(form.into_inner());
}


#[launch]

fn rocket() -> _ {
      rocket::build() 
          .manage(channet::<Message>(1024).0)       // The manage method allows to add state to our rocket server instance
          .mount("/hello", routes![world])         // To create new rocket server instance and mount the route.
}
                

