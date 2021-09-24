use bcrypt::{hash, DEFAULT_COST};
//use crate::pipayment::client::*;
use crate::PerformCrud;
use actix_web::web::Data;
use lemmy_api_common::{blocking, password_length_check, person::*, pipayment::*};
use lemmy_apub::{
  generate_apub_endpoint, generate_followers_url, generate_inbox_url, generate_shared_inbox_url,
  EndpointType,
};
use lemmy_db_queries::{
  source::{community::Community_, local_user::LocalUser_, person::*, pipayment::*, site::Site_},
  Crud, Followable, Joinable, ListingType, SortType,
};
use lemmy_db_schema::{
  naive_now,
  source::{
    community::*,
    local_user::{LocalUser, LocalUserForm},
    person::*,
    pipayment::*,
    site::*,
  },
  CommunityId, PaymentId, PersonId,
};
use lemmy_db_views::{comment_view::CommentView, local_user_view::LocalUserView};
use lemmy_db_views_actor::person_view::PersonViewSafe;

use lemmy_utils::{
  apub::generate_actor_keypair,
  claims::Claims,
  settings::structs::Settings,
  utils::{check_slurs, is_valid_actor_name},
  ApiError, ConnectionId, LemmyError,
};
use lemmy_websocket::{messages::CheckCaptcha, LemmyContext};
use sha2::{Digest, Sha256, Sha512};
use uuid::Uuid;

#[async_trait::async_trait(?Send)]
impl PerformCrud for PiLogin {
  type Response = LoginResponse;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    _websocket_id: Option<ConnectionId>,
  ) -> Result<LoginResponse, LemmyError> {
    let data: &PiLogin = &self;

    // Make sure site has open registration
    if let Ok(site) = blocking(context.pool(), move |conn| Site::read_simple(conn)).await? {
      if !site.open_registration {
        return Err(ApiError::err("registration_closed").into());
      }
    }

    if data.info.is_some() {
      // let mut info = data.info.unwrap();

      // password_length_check(&info.password)?;

      // // Make sure passwords match
      // if info.password != info.password_verify {
      //   return Err(ApiError::err("passwords_dont_match").into());
      // }

      // // Check if there are admins. False if admins exist
      // let no_admins = blocking(context.pool(), move |conn| {
      //   PersonViewSafe::admins(conn).map(|a| a.is_empty())
      // })
      // .await??;

      // // If its not the admin, check the captcha
      // if !no_admins && Settings::get().captcha.enabled {
      //   let check = context
      //     .chat_server()
      //     .send(CheckCaptcha {
      //       uuid: 
      //         info
      //         .captcha_uuid
      //         .to_owned()
      //         .unwrap_or_else(|| "".to_string()),
      //       answer: 
      //         info
      //         .captcha_answer
      //         .to_owned()
      //         .unwrap_or_else(|| "".to_string()),
      //     })
      //     .await?;
      //   if !check {
      //     return Err(ApiError::err("captcha_incorrect").into());
      //   }
      // }

      // check_slurs(&info.username)?;
      // if !is_valid_actor_name(&info.username) {
      //   //println!("Invalid username {} {}", data.pi_username.to_owned(), &data.info.username);
      //   return Err(ApiError::err("register:invalid_username").into());
      // }
  
  }

    // Hide Pi user name, not store pi_uid
    let _pi_username = data.pi_username.clone();
    let _pi_uid = data.pi_uid.unwrap();
    let _pi_token = data.pi_token.clone();

    println!("PiLogin is processing for {} {} {} ", _pi_uid.clone(), _pi_username.clone(), _pi_token.clone());

    let mut sha256 = Sha256::new();
    sha256.update(Settings::get().pi_seed());
    sha256.update(data.pi_username.to_owned());
    let _pi_alias: String = format!("{:X}", sha256.finalize());
    let _pi_alias2 = _pi_alias.clone();
    let _pi_alias3 = _pi_alias.clone();
    //let _pi_alias = data.pi_username.to_owned();

    let mut username = _pi_username.clone();
    let mut _new_user = username.clone();
    //let _new_user2 = username.clone();
    let _new_password = "".to_string(); //info.password.to_owned();

    let mut person_id: PersonId;
    let mut pi_exist = false;
    let mut result = true;
    let create_new = false;


    let pi_person = match blocking(context.pool(), move |conn| {
      Person::find_by_pi_name(&conn, &_pi_alias)
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

  
    let mut extra_user_id = None;
    match pi_person {
      Some(pi) => {
        person_id = pi.id;
        pi_exist = true;
        username = pi.name.clone();
        _new_user = username.clone();
        extra_user_id = pi.extra_user_id;
      }
      None => {
        if !create_new {
          let err_type = format!("Hi {}, you must register before login.", &username);
          println!("{} {}", _pi_uid.clone(), err_type);
          return Err(ApiError::err(&err_type).into());
        }
      }
    }
    
    if pi_exist {      
       let local_user_id;
       let _local_user = match blocking(context.pool(), move |conn| {
         LocalUserView::read_from_name(&conn, &username.clone())
       })
       .await?
       {
         Ok(lcu) => lcu, 
         Err(_e) => {
           let err_type = format!("PiLogin local user not found {} {}", &_new_user.clone(),  _e.to_string());
           println!("{} {}", _pi_uid.clone(), err_type);
           return Err(ApiError::err(&err_type).into());
          //  return Ok(PiRegisterResponse {
          //   success: false,
          //   jwt: format!(""),
          //   extra: Some(format!("{}",err_type)),
          //   });
         }
       };

       local_user_id = _local_user.local_user.id.clone();

      //  let password_hash = hash(_new_password.clone(), DEFAULT_COST).expect("Couldn't hash password");
      //  let updated_local_user = match blocking(context.pool(), move |conn| {
      //    LocalUser::update_password(&conn, local_user_id, &_new_password)
      //  })
      //  .await
      //  {
      //    Ok(chp) => chp,
      //    Err(_e) => {
      //      let err_type = format!("Register: Update local user password error {} {}", &username, _e.to_string());
      //      return Err(ApiError::err(&err_type).into());
      //     }
      //  };

      let _pi_uid_search = _pi_uid.clone();
      let  _payment = match blocking(context.pool(), move |conn| {
        PiPayment::find_by_pi_uid(&conn, &_pi_uid_search)
      })
      .await?
      {
        Ok(c) => {
          Some(c)
        }
        Err(_e) => {
          let err_type = format!("Invalid pi user id {}", &_new_user.clone());
          println!("{} {}", _pi_uid.clone(), err_type);
          return Err(ApiError::err(&err_type).into());    
        },
      };

      return Ok(LoginResponse {
          jwt: Claims::jwt(local_user_id.0)?,
          })
    }


    // We have to create both a person, and local_user
    if !create_new {
      let err_type = format!("Auto create new account for Pioneers is disabled {} {}", &_new_user.clone(), &_pi_uid.clone());
      println!("{}", err_type);
      return Err(ApiError::err(&err_type).into());
    }

    let mut change_password = false;

    let person = match blocking(context.pool(), move |conn| {
      Person::find_by_name(&conn, &username.clone())
    })
    .await?
    {
      Ok(c) => Some(c),
      Err(_e) => None,
    };

    match person {
      Some(per) => {
        if extra_user_id != per.extra_user_id {
          let err_type = format!("Register: User {} is exist and belong to other Pi Account ", &_new_user.clone());
          println!("{} {} {}", data.pi_username.clone(), err_type, &_pi_alias2);
          result = false;
          return Err(ApiError::err(&err_type).into());
          // return Ok(PiRegisterResponse {
          //   success: false,
          //   jwt: format!(""),
          //   extra: Some(format!("{}",err_type)),
          //   });
        } else {
          // Same name and account: change password ???
          change_password = true;
        }
      }
      None => {
        change_password = true;
        //change_username = true;
        // Not allow change username
        let err_type = format!("Register: You already have user name {}", _new_user.clone());
        println!("{} {} {}", data.pi_username.clone(), err_type, &_pi_alias2);
        result = false;
        //return Err(ApiError::err(&err_type).into());
        // return Ok(PiRegisterResponse {
        //   success: false,
        //   jwt: format!(""),
        //   extra: Some(format!("{}",err_type)),
        //   });
      }
    };

    let actor_keypair = generate_actor_keypair()?;
    let actor_id = generate_apub_endpoint(EndpointType::Person, &_new_user.clone())?;

    // Register the new person
    let person_form = PersonForm {
      name: _new_user.to_owned(),
      actor_id: Some(actor_id.clone()),
      private_key: Some(Some(actor_keypair.private_key)),
      public_key: Some(Some(actor_keypair.public_key)),
      inbox_url: Some(generate_inbox_url(&actor_id)?),
      shared_inbox_url: Some(Some(generate_shared_inbox_url(&actor_id)?)),
      admin: None,
      extra_user_id: Some(_pi_alias2),
      ..PersonForm::default()
    };

    // insert the person
    // let err_type = format!("user_already_exists: {} {}", &data.info.username, _pi_alias3);
    let inserted_person1 = match blocking(context.pool(), move |conn| {
      Person::create(conn, &person_form)
    })
    .await?
    {
      Ok(p) => Some(p),
      Err(_e) => {
      let err_type = format!("Register: user_already_exists: {} {}, exists{},  err:{}", 
                             &_new_user.clone(), _pi_alias3, pi_exist, _e.to_string());
      return Err(ApiError::err(&err_type).into());
      },
    };


    let inserted_person = inserted_person1.unwrap();
    // Create the local user
    let local_user_form = LocalUserForm {
      person_id: inserted_person.id,
      email: None, //Some(info.email.to_owned()),
      password_encrypted: _pi_alias3.to_owned(),
      show_nsfw: Some(true), //Some(info.show_nsfw),
      show_bot_accounts: Some(true),
      theme: Some("browser".into()),
      default_sort_type: Some(SortType::Active as i16),
      default_listing_type: Some(ListingType::Subscribed as i16),
      lang: Some("browser".into()),
      show_avatars: Some(true),
      show_scores: Some(true),
      show_read_posts: Some(true),
      send_notifications_to_email: Some(false),
      show_new_post_notifs: Some(false),
    };

    let inserted_local_user = match blocking(context.pool(), move |conn| {
      LocalUser::register(conn, &local_user_form)
    })
    .await?
    {
      Ok(lu) => lu,
      Err(_e) => {
        let err_type = if _e.to_string()
          == "duplicate key value violates unique constraint \"local_user_email_key\""
        {
          "Register: email_already_exists"
        } else {
          "Register: user_already_exists"
        };

        // If the local user creation errored, then delete that person
        blocking(context.pool(), move |conn| {
          Person::delete(&conn, inserted_person.id)
        })
        .await??;

        return Err(ApiError::err(err_type).into());
      }
    };

    let main_community_keypair = generate_actor_keypair()?;

    // Create the main community if it doesn't exist
    let main_community = match blocking(context.pool(), move |conn| {
      Community::read_from_name(conn, "main")
    })
    .await?
    {
      Ok(c) => c,
      Err(_e) => {
        let default_community_name = "main";
        let actor_id = generate_apub_endpoint(EndpointType::Community, default_community_name)?;
        let community_form = CommunityForm {
          name: default_community_name.to_string(),
          title: "The Default Community".to_string(),
          description: Some("The Default Community".to_string()),
          actor_id: Some(actor_id.to_owned()),
          private_key: Some(main_community_keypair.private_key),
          public_key: Some(main_community_keypair.public_key),
          followers_url: Some(generate_followers_url(&actor_id)?),
          inbox_url: Some(generate_inbox_url(&actor_id)?),
          shared_inbox_url: Some(Some(generate_shared_inbox_url(&actor_id)?)),
          ..CommunityForm::default()
        };
        blocking(context.pool(), move |conn| {
          Community::create(conn, &community_form)
        })
        .await??
      }
    };

    // Sign them up for main community no matter what
    let community_follower_form = CommunityFollowerForm {
      community_id: main_community.id,
      person_id: inserted_person.id,
      pending: false,
    };

    let follow = move |conn: &'_ _| CommunityFollower::follow(conn, &community_follower_form);
    if blocking(context.pool(), follow).await?.is_err() {
      //return Err(ApiError::err("Register: community_follower_already_exists").into());
    };

    // If its an admin, add them as a mod and follower to main
    // if no_admins {
    //   let community_moderator_form = CommunityModeratorForm {
    //     community_id: main_community.id,
    //     person_id: inserted_person.id,
    //   };

    //   let join = move |conn: &'_ _| CommunityModerator::join(conn, &community_moderator_form);
    //   if blocking(context.pool(), join).await?.is_err() {
    //     return Err(ApiError::err("community_moderator_already_exists").into());
    //   }
    // }

    // Return the jwt
    Ok(LoginResponse {
      jwt: Claims::jwt(inserted_local_user.id.0)?,
    })
  }
}
