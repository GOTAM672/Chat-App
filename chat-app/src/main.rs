// Import all the rocket macros globally
#[macro_use] extern crate rocket;


use rocket::tokio::sync::broadcast::{channel};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::form::Form;
use rocket::serde::{Serialize, Deserialize};



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

/* get message */

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();   // To create new receiver
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[launch]

fn rocket() -> _ {
      rocket::build() 
          .manage(channet::<Message>(1024).0)
          .mount("/", routes![post, events])
          .mount("/", FileServer::from(relative!("static")))
}
                

