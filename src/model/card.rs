use actix::*;
use actix_web::*;
use utils::schema::card;
use chrono::NaiveDateTime;
use model::response::{Msgs, CardMsgs, CardListMsgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Card {
    pub id: i32,
    pub user_id: i32,
    pub front: String,
    pub back: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="card"]
pub struct NewCard<'a> {
    pub user_id: i32,
    pub front: &'a str,
    pub back: &'a str,
    pub body: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CardNew {
    pub user_id: i32,
    pub front: String,
    pub back: String,
    pub body: String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CardId{
    pub card_id: i32,
}

pub struct CardList;

impl Message for CardList{
    type Result = Result<CardListMsgs, Error>;
}

impl Message for CardId {
    type Result = Result<CardMsgs, Error>;
}

impl Message for CardNew {
    type Result = Result<CardMsgs, Error>;
}
