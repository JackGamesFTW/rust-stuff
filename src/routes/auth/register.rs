use rocket::{post, serde::json::Json, form::Form};

use crate::{Ctx, prisma::user};

#[derive(FromForm)]
pub struct Register {
    username: String,
    email: String,
    password: String,
}

#[post("/register", data = "<data>")]
pub async fn handler(ctx: &Ctx, data: Form<Register>) -> Json<user::Data> {
    let user: user::Data = ctx.db
        .user()
        .create(
            data.username.to_string(),
            data.email.to_string(),
            data.password.to_string(),
            vec![]
        )
        .exec()
        .await
        .unwrap();

    Json(user)
}
