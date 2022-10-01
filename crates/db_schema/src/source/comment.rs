use crate::newtypes::{CommentId, DbUrl, LanguageId, LtreeDef, PersonId, PostId, CommentSavedId, CommentLikeId};
use diesel_ltree::Ltree;
use serde::{Deserialize, Serialize};

#[cfg(feature = "full")]
use crate::schema::{comment, comment_like, comment_saved};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Associations, Identifiable))]
#[cfg_attr(feature = "full", diesel(belongs_to(crate::source::post::Post)))]
#[cfg_attr(feature = "full", diesel(table_name = comment))]
pub struct Comment {
  pub id: CommentId,
  pub creator_id: PersonId,
  pub post_id: PostId,
  pub content: String,
  pub removed: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: bool,
  pub ap_id: DbUrl,
  pub local: bool,
  #[serde(with = "LtreeDef")]
  pub path: Ltree,
  pub distinguished: bool,
  pub language_id: LanguageId,
  // pub cert: Option<String>,
  pub auth_sign: Option<String>,
  pub srv_sign: Option<String>,
  pub tx : Option<String>,
}


#[derive(Clone, Default)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = comment))]
pub struct CommentForm {
  pub creator_id: PersonId,
  pub post_id: PostId,
  pub content: String,
  pub removed: Option<bool>,
  pub published: Option<chrono::NaiveDateTime>,
  pub updated: Option<chrono::NaiveDateTime>,
  pub deleted: Option<bool>,
  pub ap_id: Option<DbUrl>,
  pub local: Option<bool>,
  pub distinguished: Option<bool>,
  pub language_id: Option<LanguageId>,
  pub auth_sign: Option<String>,
  pub srv_sign: Option<String>,
  pub tx : Option<String>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "full", derive(Identifiable, Queryable, Associations))]
#[cfg_attr(feature = "full", diesel(belongs_to(crate::source::comment::Comment)))]
#[cfg_attr(feature = "full", diesel(table_name = comment_like))]
pub struct CommentLike {
  pub id: CommentLikeId,
  pub person_id: PersonId,
  pub comment_id: CommentId,
  pub post_id: PostId, // TODO this is redundant
  pub score: i16,
  pub published: chrono::NaiveDateTime,
}

#[derive(Clone)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = comment_like))]
pub struct CommentLikeForm {
  pub person_id: PersonId,
  pub comment_id: CommentId,
  pub post_id: PostId, // TODO this is redundant
  pub score: i16,
}

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "full", derive(Identifiable, Queryable, Associations))]
#[cfg_attr(feature = "full", diesel(belongs_to(crate::source::comment::Comment)))]
#[cfg_attr(feature = "full", diesel(table_name = comment_saved))]
pub struct CommentSaved {
  pub id: CommentSavedId,
  pub comment_id: CommentId,
  pub person_id: PersonId,
  pub published: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = comment_saved))]
pub struct CommentSavedForm {
  pub comment_id: CommentId,
  pub person_id: PersonId,
}
