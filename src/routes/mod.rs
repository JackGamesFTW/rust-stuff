use rocket::{Rocket, Build};

mod auth;

pub fn mount(mut rocket: Rocket<Build>) -> Rocket<Build> {
  rocket = rocket
    .mount("/auth", auth::routes());

  rocket
}
