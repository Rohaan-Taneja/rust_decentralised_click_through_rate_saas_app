use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::utils::jwt::verify_user;

/**
 * auth middleware to get "bearer ddsfdf" token from ehader and verify then and return user id , if present
 */
pub async fn authenticate_user(mut req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                // bearer token would be => "bearer dfdsfdsfdsaf"
                // so we extracted the actual token
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let user_token = match token {
        Some(auth_jwt_token) => auth_jwt_token,
        None => return (StatusCode::UNAUTHORIZED, "no auth token present").into_response(),
    };

    let userr_data = match verify_user(user_token) {
        Ok(user_data) => user_data,
        Err(_) => return (StatusCode::UNAUTHORIZED, "wrong auth tokens").into_response(),
    };

    println!(
        "this is the user id found in auth middleware {:?}",
        &userr_data
    );

    println!("user data added {userr_data:?}");
    req.extensions_mut().insert(userr_data);

    next.run(req).await
}
