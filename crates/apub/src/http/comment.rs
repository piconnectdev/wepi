use crate::{
  http::{create_apub_response, create_apub_tombstone_response},
  objects::comment::ApubComment,
};
use activitypub_federation::traits::ApubObject;
use actix_web::{web, web::Path, HttpResponse};
use diesel::result::Error::NotFound;
use lemmy_api_common::utils::blocking;
use lemmy_db_schema::{newtypes::CommentId, source::comment::Comment, traits::Crud};
use lemmy_utils::error::LemmyError;
use lemmy_websocket::LemmyContext;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct CommentQuery {
  comment_id: String,
}

/// Return the ActivityPub json representation of a local comment over HTTP.
#[tracing::instrument(skip_all)]
pub(crate) async fn get_apub_comment(
  info: Path<CommentQuery>,
  context: web::Data<LemmyContext>,
) -> Result<HttpResponse, LemmyError> {
   /// TODO: DinhHa 
  /// Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
  //let id = CommentId(info.comment_id.parse::<i32>()?);
  let uuid = Uuid::parse_str(&info.comment_id.clone()).unwrap();
  let id = CommentId(uuid);
  let comment: ApubComment = blocking(context.pool(), move |conn| Comment::read(conn, id))
    .await??
    .into();
  if !comment.local {
    return Err(NotFound.into());
  }

  if !comment.deleted {
    Ok(create_apub_response(&comment.into_apub(&**context).await?))
  } else {
    Ok(create_apub_tombstone_response(comment.ap_id.clone()))
  }
}
