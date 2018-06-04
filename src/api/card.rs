use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use actix_web::error;
use futures::future::Future;
use futures::{future};
use api::index::AppState;
use model::card::{Card,CardList, CardId, CardNew};
use actix_web::http::{Method, StatusCode};

pub fn card(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let card_id = req.match_info()
    .get("card_id")
    .unwrap_or_default()
    .parse();

    if let Ok(card_id) = card_id {
        req.state().db.send(CardId{card_id})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(card) => Ok(HttpResponse::Ok().json(card)),
                Err(e) => Ok(HttpResponse::from_error(e))
            }
        }).responder()
    }
    else {
        Box::new(future::result(Ok(HttpResponse::BadRequest().into())))
    }
}

pub fn card_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(CardList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(card_list) => Ok(HttpResponse::Ok().json(card_list)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}

pub fn card_new(card_new: Json<CardNew>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.db.send(CardNew{
            user_id: card_new.user_id.clone(),
            front : card_new.front.clone(),
            back: card_new.back.clone(),
            body: card_new.body.clone(),
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}
