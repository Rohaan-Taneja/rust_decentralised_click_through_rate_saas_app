use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{models::user::UserTypeEnum, structs::EncodedUserData};


/**
 * middleware to check if the incoming req is from workr or not 
 */
pub async fn creator_validator_middleware(req: Request, next: Next) -> Response {
    match req.extensions().get::<EncodedUserData>() {
        Some(user_type_details) => {

            match user_type_details.user_type {
                UserTypeEnum::CREATOR =>{
                    next.run(req).await

                },
                UserTypeEnum::WORKER =>{
                    return (StatusCode::UNAUTHORIZED , "user of type worker , cannot create taks").into_response()
                }
            }
            
            
        
        },
        None => return (StatusCode::BAD_REQUEST, "user data is not present").into_response(),
    }
}


/**
 * middleware to check if the incoming req is from workr or not 
 */
pub async fn worker_validator_middleware(req: Request, next: Next) -> Response {
    match req.extensions().get::<EncodedUserData>() {
        Some(user_type_details) => {

            match user_type_details.user_type {
                UserTypeEnum::CREATOR =>{
                    return (StatusCode::UNAUTHORIZED , "user of type creator , cannot do worker task").into_response()
                   

                },
                UserTypeEnum::WORKER =>{
                     next.run(req).await
                    
                }
            }
            
            
        
        },
        None => return (StatusCode::BAD_REQUEST, "user data is not present").into_response(),
    }
}