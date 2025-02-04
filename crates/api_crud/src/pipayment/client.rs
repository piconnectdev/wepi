use chrono::Duration;

use actix_web::web::Data;
use lemmy_api_common::context::LemmyContext;
use lemmy_api_common::pipayment::*;
use lemmy_db_schema::{
  newtypes::{CommentId, *},
  schema::pipayment::user_cancelled,
  source::{comment::*, person::*, pipayment::*, post::*},
  traits::{Crud, Signable},
  utils::naive_now,
};
use lemmy_utils::{error::LemmyError, request::retry, settings::SETTINGS, REQWEST_TIMEOUT};
use reqwest_middleware::ClientWithMiddleware;

use sha2::{Digest, Sha256};
use uuid::Uuid;

pub fn hide_username(name: &str) -> String {
  let settings = SETTINGS.to_owned();
  let mut sha256 = Sha256::new();
  sha256.update(settings.pi_seed());
  sha256.update(name.clone().to_owned());
  let username: String = format!("{:X}", sha256.finalize());
  return username;
}

pub async fn pi_payment(
  client: &ClientWithMiddleware,
  id: &str,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments/{}", settings.pi_api_host(), id);

  let response = retry(|| {
    client
      .get(&fetch_url)
      .timeout(REQWEST_TIMEOUT)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let content = response.text().await?;
  match serde_json::from_str(&content) {
    Ok(r) => Ok(r),
    Err(_e) => {
      return Err(LemmyError::from_message(&content));
    }
  }
}

pub async fn pi_incompleted_server_payments(
  client: &ClientWithMiddleware,
) -> Result<Vec<PiPaymentDto>, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!(
    "{}/payments/incomplete_server_payments",
    settings.pi_api_host()
  );

  let response = retry(|| {
    client
      .get(&fetch_url)
      .timeout(REQWEST_TIMEOUT)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let content = response.text().await?;
  let res: IncompleteServerPayments = match serde_json::from_str(&content) {
    Ok(r) => r,
    Err(_e) => {
      return Err(LemmyError::from_message(&content));
    }
  };

  Ok(res.incomplete_server_payments)
}

pub async fn pi_approve(
  client: &ClientWithMiddleware,
  id: &str,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments/{}/approve", settings.pi_api_host(), id);

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let content = response.text().await?;
  match serde_json::from_str(&content) {
    Ok(r) => Ok(r),
    Err(_e) => {
      return Err(LemmyError::from_message(&content));
    }
  }
}

pub async fn pi_create(
  client: &ClientWithMiddleware,
  payment: &PiPaymentCreate,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments", settings.pi_api_host());

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .json(&payment)
      .send()
  })
  .await?;

  let content = response.text().await?;
  match serde_json::from_str(&content) {
    Ok(r) => Ok(r),
    Err(_e) => {
      return Err(LemmyError::from_message(&content));
    }
  }
}

pub async fn pi_cancel(
  client: &ClientWithMiddleware,
  id: &str,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments/{}/cancel", settings.pi_api_host(), id);

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  let content = response.text().await?;
  match serde_json::from_str(&content) {
    Ok(r) => Ok(r),
    Err(_e) => {
      return Err(LemmyError::from_message(&content));
    }
  }
}

pub async fn pi_complete(
  client: &ClientWithMiddleware,
  id: &str,
  txid_: &str,
) -> Result<PiPaymentDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/payments/{}/complete", settings.pi_api_host(), id);

  let r = TxRequest {
    txid: txid_.to_owned(),
  };

  let response = retry(|| {
    client
      .post(&fetch_url)
      .header("Authorization", format!("Key {}", settings.pi_key()))
      .header("Content-Type", format!("application/json"))
      .json(&r)
      .send()
  })
  .await?;

  let content = response.text().await?;
  match serde_json::from_str(&content) {
    Ok(r) => Ok(r),
    Err(_e) => {
      return Err(LemmyError::from_message(&content));
    }
  }
}

pub async fn pi_me(context: &Data<LemmyContext>, key: &str) -> Result<PiUserDto, LemmyError> {
  let settings = SETTINGS.to_owned();
  let fetch_url = format!("{}/me", settings.pi_api_host());
  let client = context.client();

  match context
    .chat_server()
    .check_pi_token(key.to_string().clone(), "".to_string())?
  {
    Some(p) => return Ok(p),
    None => {}
  }

  let response = retry(|| {
    client
      .get(&fetch_url)
      .header("Authorization", format!("Bearer {}", &key.clone()))
      .header("Content-Type", format!("application/json"))
      .send()
  })
  .await?;

  // let mut res: PiUserDto = response
  //   .json::<PiUserDto>()
  //   .await
  //   .map_err(|e| LemmyError::from_error_message(e, "Fetch /me error"))?;

  let content = response.text().await?;
  let mut res: PiUserDto = match serde_json::from_str(&content) {
    Ok(r) => r,
    Err(_e) => {
      return Err(LemmyError::from_message(&content));
    }
  };

  if settings.pi_hide_account {
    res.username = hide_username(&res.username.clone());
  }

  let token_item = PiTokenItem {
    answer: res.clone(),
    uuid: key.to_string(),
    expires: naive_now() + Duration::days(1), // expires in 5 days
  };

  // Stores the PiTokenItem item on the queue
  context.chat_server().add_pi_token(token_item)?;

  Ok(res)
}
