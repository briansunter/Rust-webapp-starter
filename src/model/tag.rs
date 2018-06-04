use actix::*;
use actix_web::*;
use utils::schema::tags;
use utils::schema::card_tags;
use utils::schema::card;
use chrono::NaiveDateTime;
use model::response::{Msgs, CardMsgs, CardListMsgs};
use model::card::{Card};

#[derive(Insertable)]
#[table_name="card_tags"]
pub struct NewCardTag {
    pub card_id: i32,
    pub tag_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Card)]
#[belongs_to(Tag)]
pub struct CardTag {
    pub id: i32,
    pub card_id: i32,
    pub tag_id: i32,
}


#[derive(Identifiable, Queryable, Associations)]
#[has_many(card_tags)]
pub struct Tag {
    pub id: i32,
    pub label: String,
}

#[derive(Insertable)]
#[table_name="tags"]
pub struct NewTag<'a> {
    pub label: &'a str,
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
