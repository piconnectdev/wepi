use actix_web::{guard, web, Error, HttpResponse, Result};
use lemmy_api::Perform;
use lemmy_api_common::{
  comment::{
    CreateComment, CreateCommentLike, CreateCommentReport, DeleteComment, DistinguishComment,
    EditComment, GetComment, ListCommentReports, RemoveComment, ResolveCommentReport, SaveComment,
  },
  community::{
    AddModToCommunity, BanFromCommunity, BlockCommunity, CreateCommunity, DeleteCommunity,
    EditCommunity, FollowCommunity, HideCommunity, ListCommunities, RemoveCommunity,
    TransferCommunity,
  },
  context::LemmyContext,
  custom_emoji::{CreateCustomEmoji, DeleteCustomEmoji, EditCustomEmoji},
  person::{
    AddAdmin, BanPerson, BlockPerson, ChangePassword, DeleteAccount, GetBannedPersons, GetCaptcha,
    GetPersonMentions, GetReplies, GetReportCount, GetUnreadCount, Login, MarkAllAsRead,
    MarkCommentReplyAsRead, MarkPersonMentionAsRead, PasswordChangeAfterReset, PasswordReset,
    Register, SaveUserSettings, VerifyEmail,
  },
  pipayment::*,
  post::{
    CreatePost, CreatePostLike, CreatePostReport, DeletePost, EditPost, FeaturePost, GetPost,
    GetSiteMetadata, ListPostReports, LockPost, MarkPostAsRead, RemovePost, ResolvePostReport,
    SavePost,
  },
  private_message::{
    CreatePrivateMessage, CreatePrivateMessageReport, DeletePrivateMessage, EditPrivateMessage,
    GetPrivateMessages, ListPrivateMessageReports, MarkPrivateMessageAsRead,
    ResolvePrivateMessageReport,
  },
  site::{
    ApproveRegistrationApplication, CreateSite, EditSite, GetFederatedInstances, GetModlog,
    GetSite, GetUnreadRegistrationApplicationCount, LeaveAdmin, ListRegistrationApplications,
    PurgeComment, PurgeCommunity, PurgePerson, PurgePost,
  },
  web3::*,
  //websocket::structs::{CommunityJoin, ModJoin, PostJoin, UserJoin},
};
use lemmy_api_crud::PerformCrud;
use lemmy_apub::{
  api::{
    list_comments::list_comments, list_posts::list_posts, read_community::read_community,
    read_person::read_person, resolve_object::resolve_object, search::search,
  },
  SendActivity,
};
use lemmy_utils::rate_limit::RateLimitCell;
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig, rate_limit: &RateLimitCell) {
  cfg.service(
    web::scope("/api/v3")
      // Site
      .service(
        web::scope("/site")
          .wrap(rate_limit.message())
          .route("", web::get().to(route_get_crud::<GetSite>))
          // Admin Actions
          .route("", web::post().to(route_post_crud::<CreateSite>))
          .route("", web::put().to(route_post_crud::<EditSite>)),
      )
      .service(
        web::resource("/modlog")
          .wrap(rate_limit.message())
          .route(web::get().to(route_get::<GetModlog>)),
      )
      .service(
        web::resource("/search")
          .wrap(rate_limit.search())
          .route(web::get().to(search)),
      )
      .service(
        web::resource("/resolve_object")
          .wrap(rate_limit.message())
          .route(web::get().to(resolve_object)),
      )
      // Community
      .service(
        web::resource("/community")
          .guard(guard::Post())
          .wrap(rate_limit.register())
          .route(web::post().to(route_post_crud::<CreateCommunity>)),
      )
      .service(
        web::scope("/community")
          .wrap(rate_limit.message())
          .route("", web::get().to(read_community))
          .route("", web::put().to(route_post_crud::<EditCommunity>))
          .route("/hide", web::put().to(route_post::<HideCommunity>))
          .route("/list", web::get().to(route_get_crud::<ListCommunities>))
          .route("/follow", web::post().to(route_post::<FollowCommunity>))
          .route("/block", web::post().to(route_post::<BlockCommunity>))
          .route(
            "/delete",
            web::post().to(route_post_crud::<DeleteCommunity>),
          )
          // Mod Actions
          .route(
            "/remove",
            web::post().to(route_post_crud::<RemoveCommunity>),
          )
          .route("/transfer", web::post().to(route_post::<TransferCommunity>))
          .route("/ban_user", web::post().to(route_post::<BanFromCommunity>))
          .route("/mod", web::post().to(route_post::<AddModToCommunity>)),
      )
      .service(
        web::scope("/federated_instances")
          .wrap(rate_limit.message())
          .route("", web::get().to(route_get::<GetFederatedInstances>)),
      )
      // Post
      .service(
        // Handle POST to /post separately to add the post() rate limitter
        web::resource("/post")
          .guard(guard::Post())
          .wrap(rate_limit.post())
          .route(web::post().to(route_post_crud::<CreatePost>)),
      )
      .service(
        web::scope("/post")
          .wrap(rate_limit.message())
          .route("", web::get().to(route_get_crud::<GetPost>))
          .route("", web::put().to(route_post_crud::<EditPost>))
          .route("/delete", web::post().to(route_post_crud::<DeletePost>))
          .route("/remove", web::post().to(route_post_crud::<RemovePost>))
          .route(
            "/mark_as_read",
            web::post().to(route_post::<MarkPostAsRead>),
          )
          .route("/lock", web::post().to(route_post::<LockPost>))
          .route("/feature", web::post().to(route_post::<FeaturePost>))
          .route("/list", web::get().to(list_posts))
          .route("/like", web::post().to(route_post::<CreatePostLike>))
          .route("/save", web::put().to(route_post::<SavePost>))
          .route("/report", web::post().to(route_post::<CreatePostReport>))
          .route(
            "/report/resolve",
            web::put().to(route_post::<ResolvePostReport>),
          )
          .route("/report/list", web::get().to(route_get::<ListPostReports>))
          .route(
            "/site_metadata",
            web::get().to(route_get::<GetSiteMetadata>),
          ),
      )
      // Comment
      .service(
        // Handle POST to /comment separately to add the comment() rate limitter
        web::resource("/comment")
          .guard(guard::Post())
          .wrap(rate_limit.comment())
          .route(web::post().to(route_post_crud::<CreateComment>)),
      )
      .service(
        web::scope("/comment")
          .wrap(rate_limit.message())
          .route("", web::get().to(route_get_crud::<GetComment>))
          .route("", web::put().to(route_post_crud::<EditComment>))
          .route("/delete", web::post().to(route_post_crud::<DeleteComment>))
          .route("/remove", web::post().to(route_post_crud::<RemoveComment>))
          .route(
            "/mark_as_read",
            web::post().to(route_post::<MarkCommentReplyAsRead>),
          )
          .route(
            "/distinguish",
            web::post().to(route_post::<DistinguishComment>),
          )
          .route("/like", web::post().to(route_post::<CreateCommentLike>))
          .route("/save", web::put().to(route_post::<SaveComment>))
          .route("/list", web::get().to(list_comments))
          .route("/report", web::post().to(route_post::<CreateCommentReport>))
          .route(
            "/report/resolve",
            web::put().to(route_post::<ResolveCommentReport>),
          )
          .route(
            "/report/list",
            web::get().to(route_get::<ListCommentReports>),
          ),
      )
      // Private Message
      .service(
        web::scope("/private_message")
          .wrap(rate_limit.message())
          .route("/list", web::get().to(route_get_crud::<GetPrivateMessages>))
          .route("", web::post().to(route_post_crud::<CreatePrivateMessage>))
          .route("", web::put().to(route_post_crud::<EditPrivateMessage>))
          .route(
            "/delete",
            web::post().to(route_post_crud::<DeletePrivateMessage>),
          )
          .route(
            "/mark_as_read",
            web::post().to(route_post::<MarkPrivateMessageAsRead>),
          )
          .route(
            "/report",
            web::post().to(route_post::<CreatePrivateMessageReport>),
          )
          .route(
            "/report/resolve",
            web::put().to(route_post::<ResolvePrivateMessageReport>),
          )
          .route(
            "/report/list",
            web::get().to(route_get::<ListPrivateMessageReports>),
          ),
      )
      // User
      .service(
        // Account action, I don't like that it's in /user maybe /accounts
        // Handle /user/register separately to add the register() rate limitter
        web::resource("/user/register")
          .guard(guard::Post())
          .wrap(rate_limit.register())
          .route(web::post().to(route_post_crud::<Register>)),
      )
      .service(
        // Handle captcha separately
        web::resource("/user/get_captcha")
          .wrap(rate_limit.post())
          .route(web::get().to(route_get::<GetCaptcha>)),
      )
      // User actions
      .service(
        web::scope("/user")
          .wrap(rate_limit.message())
          .route("", web::get().to(read_person))
          .route("/mention", web::get().to(route_get::<GetPersonMentions>))
          //.route("/myinfo", web::get().to(route_get_crud::<GetMyUserInfo>))
          .route(
            "/mention/mark_as_read",
            web::post().to(route_post::<MarkPersonMentionAsRead>),
          )
          .route("/replies", web::get().to(route_get::<GetReplies>))
          // Admin action. I don't like that it's in /user
          .route("/ban", web::post().to(route_post::<BanPerson>))
          .route("/banned", web::get().to(route_get::<GetBannedPersons>))
          .route("/block", web::post().to(route_post::<BlockPerson>))
          // Account actions. I don't like that they're in /user maybe /accounts
          .route("/login", web::post().to(route_post::<Login>))
          .route(
            "/delete_account",
            web::post().to(route_post_crud::<DeleteAccount>),
          )
          .route(
            "/password_reset",
            web::post().to(route_post::<PasswordReset>),
          )
          .route(
            "/password_change",
            web::post().to(route_post::<PasswordChangeAfterReset>),
          )
          // mark_all_as_read feels off being in this section as well
          .route(
            "/mark_all_as_read",
            web::post().to(route_post::<MarkAllAsRead>),
          )
          .route(
            "/save_user_settings",
            web::put().to(route_post::<SaveUserSettings>),
          )
          .route(
            "/change_password",
            web::put().to(route_post::<ChangePassword>),
          )
          .route("/report_count", web::get().to(route_get::<GetReportCount>))
          .route("/unread_count", web::get().to(route_get::<GetUnreadCount>))
          .route("/verify_email", web::post().to(route_post::<VerifyEmail>))
          .route("/leave_admin", web::post().to(route_post::<LeaveAdmin>)),
      )
      // Admin Actions
      .service(
        web::scope("/admin")
          .wrap(rate_limit.message())
          .route("/add", web::post().to(route_post::<AddAdmin>))
          .route(
            "/registration_application/count",
            web::get().to(route_get::<GetUnreadRegistrationApplicationCount>),
          )
          .route(
            "/registration_application/list",
            web::get().to(route_get::<ListRegistrationApplications>),
          )
          .route(
            "/registration_application/approve",
            web::put().to(route_post::<ApproveRegistrationApplication>),
          )
          .service(
            web::scope("/purge")
              .route("/person", web::post().to(route_post::<PurgePerson>))
              .route("/community", web::post().to(route_post::<PurgeCommunity>))
              .route("/post", web::post().to(route_post::<PurgePost>))
              .route("/comment", web::post().to(route_post::<PurgeComment>)),
          ),
      )
      // Web3
      .service(
        web::scope("/web3")
          .wrap(rate_limit.message())
          .route("/register", web::post().to(route_post_crud::<Web3Register>))
          .route("/login", web::post().to(route_post_crud::<Web3Login>))
          .route("/web3login", web::post().to(route_post_crud::<Web3Login>)),
      )
      // Pi Payment
      .service(
        web::scope("/pi")
          .wrap(rate_limit.message())
          .route("/found", web::post().to(route_post_crud::<PiPaymentFound>))
          .route("/register", web::post().to(route_post_crud::<PiRegister>))
          .route("/login", web::post().to(route_post_crud::<PiLogin>)) //.route("/payment", web::get().to(route_get_crud::<GetPayment>)),
          .route("/agree", web::post().to(route_post_crud::<PiAgreeRegister>))
          .route(
            "/register_with_fee",
            web::post().to(route_post_crud::<PiRegisterWithFee>),
          )
          .route("/approve", web::post().to(route_post_crud::<PiApprove>))
          .route(
            "/complete",
            web::post().to(route_post_crud::<PiPaymentComplete>),
          )
          .route("/key", web::post().to(route_post_crud::<PiKey>)),
        //.route("/payments", web::get().to(route_get_crud::<GetPayments>)),
      )
      .service(
        web::scope("/payment")
          .wrap(rate_limit.message())
          .route("", web::get().to(route_post_crud::<GetPayment>))
          .route("", web::post().to(route_post_crud::<CreatePayment>))
          .route("/list", web::post().to(route_post_crud::<GetPayments>))
          .route("/send", web::post().to(route_post_crud::<SendPayment>))
          .route("/balance", web::post().to(route_post_crud::<GetPiBalances>))
          .route("/withdraw", web::post().to(route_post_crud::<PiWithdraw>)),
        //.route("/payments", web::get().to(route_get_crud::<GetPayments>)),
      )
      .service(
        web::scope("/custom_emoji")
          .wrap(rate_limit.message())
          .route("", web::post().to(route_post_crud::<CreateCustomEmoji>))
          .route("", web::put().to(route_post_crud::<EditCustomEmoji>))
          .route(
            "/delete",
            web::post().to(route_post_crud::<DeleteCustomEmoji>),
          ),
      ),
  );
}

async fn perform<'a, Data>(
  data: Data,
  context: web::Data<LemmyContext>,
  apub_data: activitypub_federation::config::Data<LemmyContext>,
) -> Result<HttpResponse, Error>
where
  Data: Perform
    + SendActivity<Response = <Data as Perform>::Response>
    + Clone
    + Deserialize<'a>
    + Send
    + 'static,
{
  let res = data.perform(&context).await?;
  SendActivity::send_activity(&data, &res, &apub_data).await?;
  Ok(HttpResponse::Ok().json(res))
}

async fn route_get<'a, Data>(
  data: web::Query<Data>,
  context: web::Data<LemmyContext>,
  apub_data: activitypub_federation::config::Data<LemmyContext>,
) -> Result<HttpResponse, Error>
where
  Data: Perform
    + SendActivity<Response = <Data as Perform>::Response>
    + Clone
    + Deserialize<'a>
    + Send
    + 'static,
{
  perform::<Data>(data.0, context, apub_data).await
}

async fn route_post<'a, Data>(
  data: web::Json<Data>,
  context: web::Data<LemmyContext>,
  apub_data: activitypub_federation::config::Data<LemmyContext>,
) -> Result<HttpResponse, Error>
where
  Data: Perform
    + SendActivity<Response = <Data as Perform>::Response>
    + Clone
    + Deserialize<'a>
    + Send
    + 'static,
{
  perform::<Data>(data.0, context, apub_data).await
}

async fn perform_crud<'a, Data>(
  data: Data,
  context: web::Data<LemmyContext>,
  apub_data: activitypub_federation::config::Data<LemmyContext>,
) -> Result<HttpResponse, Error>
where
  Data: PerformCrud
    + SendActivity<Response = <Data as PerformCrud>::Response>
    + Clone
    + Deserialize<'a>
    + Send
    + 'static,
{
  let res = data.perform(&context).await?;
  SendActivity::send_activity(&data, &res, &apub_data).await?;
  Ok(HttpResponse::Ok().json(res))
}

async fn route_get_crud<'a, Data>(
  data: web::Query<Data>,
  context: web::Data<LemmyContext>,
  apub_data: activitypub_federation::config::Data<LemmyContext>,
) -> Result<HttpResponse, Error>
where
  Data: PerformCrud
    + SendActivity<Response = <Data as PerformCrud>::Response>
    + Clone
    + Deserialize<'a>
    + Send
    + 'static,
{
  perform_crud::<Data>(data.0, context, apub_data).await
}

async fn route_post_crud<'a, Data>(
  data: web::Json<Data>,
  context: web::Data<LemmyContext>,
  apub_data: activitypub_federation::config::Data<LemmyContext>,
) -> Result<HttpResponse, Error>
where
  Data: PerformCrud
    + SendActivity<Response = <Data as PerformCrud>::Response>
    + Clone
    + Deserialize<'a>
    + Send
    + 'static,
{
  perform_crud::<Data>(data.0, context, apub_data).await
}

/*
    UserOperation::GetToken => do_websocket_operation::<GetToken>(context, id, op, data).await,




    UserOperationCrud::PiRegister => {
      do_websocket_operation_crud::<PiRegister>(context, id, op, data).await
    }
    UserOperationCrud::PiLogin => {
      do_websocket_operation_crud::<PiLogin>(context, id, op, data).await
    }
    UserOperationCrud::PiAgreeRegister => {
      do_websocket_operation_crud::<PiAgreeRegister>(context, id, op, data).await
    }
    UserOperationCrud::PiRegisterWithFee => {
      do_websocket_operation_crud::<PiRegisterWithFee>(context, id, op, data).await
    }
    UserOperationCrud::PiApprove => {
      do_websocket_operation_crud::<PiApprove>(context, id, op, data).await
    }
    UserOperationCrud::PiPaymentComplete => {
      do_websocket_operation_crud::<PiPaymentComplete>(context, id, op, data).await
    }
    UserOperationCrud::PiPaymentFound => {
      do_websocket_operation_crud::<PiPaymentFound>(context, id, op, data).await
    }
    UserOperationCrud::PiKey => {
      do_websocket_operation_crud::<PiKey>(context, id, op, data).await
    }
    UserOperationCrud::Web3Register => {
      do_websocket_operation_crud::<Web3Register>(context, id, op, data).await
    }
    UserOperationCrud::Web3Login => {
      do_websocket_operation_crud::<Web3Login>(context, id, op, data).await
    }
    UserOperationCrud::CreatePayment => {
      do_websocket_operation_crud::<CreatePayment>(context, id, op, data).await
    }
    UserOperationCrud::GetPayment => {
      do_websocket_operation_crud::<GetPayment>(context, id, op, data).await
    }
    UserOperationCrud::GetPayments => {
      do_websocket_operation_crud::<GetPayments>(context, id, op, data).await
    }
    UserOperationCrud::GetPiBalances => {
      do_websocket_operation_crud::<GetPiBalances>(context, id, op, data).await
    }
    UserOperationCrud::PiWithdraw => {
      do_websocket_operation_crud::<PiWithdraw>(context, id, op, data).await
    }
    UserOperationCrud::SendPayment => {
      do_websocket_operation_crud::<SendPayment>(context, id, op, data).await
    }
    UserOperationCrud::GetMyUserInfo => {
      do_websocket_operation_crud::<GetMyUserInfo>(context, id, op, data).await
    }

*/
