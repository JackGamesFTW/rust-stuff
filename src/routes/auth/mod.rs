use rocket::Route;

mod register;

pub fn routes() -> Vec<Route> {
  routes![
    register::handler
  ]
}
