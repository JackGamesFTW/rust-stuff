use std::sync::Arc;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

pub mod prisma;

use prisma::{user};

// This is the struct that will hold the prisma client. This will be managed by
// Rocket, accessible in the routing functions using the type alias `Ctx` below.
// See https://rocket.rs/v0.5-rc/guide/state/
#[derive(Clone)]
pub struct Context {
    pub db: Arc<prisma::PrismaClient>,
}

// Type alias not required, just personal preference for this in particular so I
// don't have to write `rocket::State<Context>` every single time.
pub type Ctx = rocket::State<Context>;

#[get("/")]
async fn index(ctx: &Ctx) -> Json<Vec<user::Data>> {
    let users = ctx.db
        .user()
        .find_many(vec![])
        .exec()
        .await
        .unwrap();

    Json(users)
}

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        prisma::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    #[cfg(debug_assert)]
    db._db_push(false).await.unwrap();

    rocket::build()
        .manage(Context { db })
        .mount("/", routes![index])
}
