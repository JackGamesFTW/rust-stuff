use std::sync::Arc;

#[macro_use]
extern crate rocket;

pub mod prisma;
pub mod routes;

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

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        prisma::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    #[cfg(debug_assert)]
    db._db_push(false).await.unwrap();

    routes::mount(rocket::build())
        .manage(Context { db })
}
