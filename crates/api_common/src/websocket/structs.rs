use crate::sensitive::Sensitive;
use crate::pipayment::PiUserDto;
use lemmy_db_schema::newtypes::{CommunityId, PostId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserJoin {
  pub auth: Sensitive<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserJoinResponse {
  pub joined: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityJoin {
  pub community_id: Option<CommunityId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunityJoinResponse {
  pub joined: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModJoin {
  pub community_id: CommunityId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModJoinResponse {
  pub joined: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostJoin {
  pub post_id: PostId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostJoinResponse {
  pub joined: bool,
}

// #[derive(Debug)]
// pub struct CaptchaItem {
//   pub uuid: String,
//   pub answer: String,
//   pub expires: chrono::NaiveDateTime,
// }

// #[derive(Debug, Clone)]
// pub struct TokenItem {
//   pub uuid: String,
//   pub answer: String,
//   pub expires: chrono::NaiveDateTime,
// }

// #[derive(Debug, Clone)]
// pub struct PiTokenItem {
//   pub uuid: String,
//   pub answer: PiUserDto,
//   pub expires: chrono::NaiveDateTime,
// }