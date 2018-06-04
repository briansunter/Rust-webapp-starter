use diesel;
use diesel::prelude::*;
use actix::*;
use actix_web::*;
use chrono::Utc;
use model::response::{CardListMsgs, CardMsgs, Msgs};
use model::card::{Card, CardList, CardId, NewCard, CardNew};
use model::db::ConnDsl;

impl Handler<CardList> for ConnDsl {
    type Result = Result<CardListMsgs, Error>;

    fn handle(&mut self, card_list: CardList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::card::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let cards = card.load::<Card>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(CardListMsgs {
            status: 200,
            message : "Card_list result.".to_string(),
            card_list: cards,
        })
    }
}

impl Handler<CardId> for ConnDsl {
    type Result = Result<CardMsgs, Error>;

    fn handle(&mut self, Card_id: CardId, _: &mut Self::Context) -> Self::Result {
        use utils::schema::card::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let the_Card =  card.filter(&id.eq(&Card_id.card_id)).load::<Card>(conn).map_err(error::ErrorInternalServerError)?.pop();
        match the_Card {
            Some(the_Card) => {
                    let current_Card = Card {
                        id: the_Card.id,
                        user_id: the_Card.user_id,
                        front: the_Card.front.clone(),
                        back: the_Card.back.clone(),
                        body: the_Card.body.clone(),
                        tags: the_Card.tags.clone(),
                        created_at: the_Card.created_at.clone(),
                    };
                    Ok(CardMsgs {
                        status: 200,
                        message : "The  current_user info.".to_string(),
                        card: current_Card,
                    })
            },
            None => {
                    let no_Card = Card {
                        id: 0,
                        user_id: 0,
                        front: "".to_owned(),
                        back: "".to_owned(),
                        body: "".to_owned(),
                        tags: vec![].to_owned(),
                        created_at: Utc::now().naive_utc(),
                    };
                    Err(error::ErrorNotFound::<String>("foo".to_string()))
            },
        }
    }
}


impl Handler<CardNew> for ConnDsl {
    type Result = Result<CardMsgs, Error>;

    fn handle(&mut self, Card_new: CardNew, _: &mut Self::Context) -> Self::Result {
        use utils::schema::card::dsl::*;

        let new_Card = NewCard {
            user_id: Card_new.user_id,
            front: &Card_new.front,
            back: &Card_new.back,
            body: &Card_new.body,
            tags: Card_new.tags,
            created_at: Utc::now().naive_utc(),
        };
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let inserted_card = diesel::insert_into(card).values(&new_Card).get_result(conn).map_err(error::ErrorInternalServerError)?;
        Ok(CardMsgs {
                    status: 200,
                    card: inserted_card,
                    message : "Card Publish Successful.".to_string(),
        })
    }
}
