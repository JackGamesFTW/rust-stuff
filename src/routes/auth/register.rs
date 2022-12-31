use rocket::{post, serde::json::Json};

use crate::{Ctx, prisma::user};

#[post("/register")]
pub async fn handler(ctx: &Ctx) -> Json<Vec<user::Data>> {
    let users = ctx.db
        .user()
        .find_many(vec![])
        .exec()
        .await
        .unwrap();

    Json(users)
}
