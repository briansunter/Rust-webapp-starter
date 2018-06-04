use model::user::User;
use model::card::Card;

pub enum MyError {
    NotFound,
    DatabaseError,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Msgs {
    pub status: i32,
    pub message : String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct SigninMsgs {
    pub status: i32,
    pub token: String,
    pub signin_user: User,
    pub message : String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct UserInfoMsgs {
    pub status: i32,
    pub message : String,
    pub current_user: User,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CardMsgs{
    pub status: i32,
    pub message : String,
    pub card: Card,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CardListMsgs {
    pub status: i32,
    pub message : String,
    pub card_list: Vec<Card>,
}
