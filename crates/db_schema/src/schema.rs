table! {
    activity (id) {
        id -> Uuid,
        data -> Jsonb,
        local -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        ap_id -> Text,
        sensitive -> Nullable<Bool>,
    }
}

table! {
  use diesel_ltree::sql_types::Ltree;
  use diesel::sql_types::*;

    comment (id) {
        id -> Uuid,
        creator_id -> Uuid,
        post_id -> Uuid,
        content -> Text,
        removed -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Bool,
        ap_id -> Varchar,
        local -> Bool,
        path -> Ltree,
        distinguished -> Bool,
        language_id -> Int4,
        //private_id -> Uuid,
        cert -> Nullable<Text>,
        tx -> Nullable<Text>,
    }
}

table! {
    comment_aggregates (id) {
        id -> Uuid,
        comment_id -> Uuid,
        score -> Int8,
        upvotes -> Int8,
        downvotes -> Int8,
        published -> Timestamp,
        child_count ->  Int4,
    }
}

table! {
    comment_like (id) {
        id -> Uuid,
        person_id -> Uuid,
        comment_id -> Uuid,
        post_id -> Uuid,
        score -> Int2,
        published -> Timestamp,
    }
}

table! {
    comment_report (id) {
        id -> Uuid,
        creator_id -> Uuid,
        comment_id -> Uuid,
        original_comment_text -> Text,
        reason -> Text,
        resolved -> Bool,
        resolver_id -> Nullable<Uuid>,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    comment_saved (id) {
        id -> Uuid,
        comment_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    community (id) {
        id -> Uuid,
        name -> Varchar,
        title -> Varchar,
        description -> Nullable<Text>,
        removed -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Bool,
        nsfw -> Bool,
        actor_id -> Varchar,
        local -> Bool,
        private_key -> Nullable<Text>,
        public_key -> Text,
        last_refreshed_at -> Timestamp,
        icon -> Nullable<Varchar>,
        banner -> Nullable<Varchar>,
        followers_url -> Varchar,
        inbox_url -> Varchar,
        shared_inbox_url -> Nullable<Varchar>,
        hidden -> Bool,
        posting_restricted_to_mods -> Bool,
		cert -> Nullable<Text>,
        tx -> Nullable<Text>,        
    }
}

table! {
    community_aggregates (id) {
        id -> Uuid,
        community_id -> Uuid,
        subscribers -> Int8,
        posts -> Int8,
        comments -> Int8,
        published -> Timestamp,
        users_active_day -> Int8,
        users_active_week -> Int8,
        users_active_month -> Int8,
        users_active_half_year -> Int8,
    }
}

table! {
    community_follower (id) {
        id -> Uuid,
        community_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
        pending -> Nullable<Bool>,
    }
}

table! {
    community_moderator (id) {
        id -> Uuid,
        community_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    community_person_ban (id) {
        id -> Uuid,
        community_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
        expires -> Nullable<Timestamp>,
    }
}

table! {
    local_user (id) {
        id -> Uuid,
        person_id -> Uuid,
        password_encrypted -> Text,
        email -> Nullable<Text>,
        show_nsfw -> Bool,
        theme -> Varchar,
        default_sort_type -> Int2,
        default_listing_type -> Int2,
        interface_language -> Varchar,
        show_avatars -> Bool,
        send_notifications_to_email -> Bool,
        validator_time -> Timestamp,
        show_bot_accounts -> Bool,
        show_scores -> Bool,
        show_read_posts -> Bool,
        show_new_post_notifs -> Bool,
        email_verified -> Bool,
        accepted_application -> Bool,
    }
}

table! {
    mod_add (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_add_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        community_id -> Uuid,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_transfer_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        community_id -> Uuid,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_ban (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        reason -> Nullable<Text>,
        banned -> Nullable<Bool>,
        expires -> Nullable<Timestamp>,
        when_ -> Timestamp,
    }
}

table! {
    mod_ban_from_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        other_person_id -> Uuid,
        community_id -> Uuid,
        reason -> Nullable<Text>,
        banned -> Nullable<Bool>,
        expires -> Nullable<Timestamp>,
        when_ -> Timestamp,
    }
}

table! {
    mod_lock_post (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        post_id -> Uuid,
        locked -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_remove_comment (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        comment_id -> Uuid,
        reason -> Nullable<Text>,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_remove_community (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        community_id -> Uuid,
        reason -> Nullable<Text>,
        removed -> Nullable<Bool>,
        expires -> Nullable<Timestamp>,
        when_ -> Timestamp,
    }
}

table! {
    mod_remove_post (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        post_id -> Uuid,
        reason -> Nullable<Text>,
        removed -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    mod_sticky_post (id) {
        id -> Uuid,
        mod_person_id -> Uuid,
        post_id -> Uuid,
        stickied -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    password_reset_request (id) {
        id -> Uuid,
        token_encrypted -> Text,
        published -> Timestamp,
        local_user_id -> Uuid,
    }
}

table! {
    person (id) {
        id -> Uuid,
        name -> Varchar,
        display_name -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        banned -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        actor_id -> Varchar,
        bio -> Nullable<Text>,
        local -> Bool,
        private_key -> Nullable<Text>,
        public_key -> Text,
        last_refreshed_at -> Timestamp,
        banner -> Nullable<Varchar>,
        deleted -> Bool,
        inbox_url -> Varchar,
        shared_inbox_url -> Nullable<Varchar>,
        matrix_user_id -> Nullable<Text>,
        admin -> Bool,
        bot_account -> Bool,
        ban_expires -> Nullable<Timestamp>,		
		extra_user_id -> Nullable<Text>,
        verified -> Bool,
        private_seeds -> Nullable<Text>,
        pi_address -> Nullable<Text>,
        web3_address -> Nullable<Text>,
        sol_address -> Nullable<Text>,
        dap_address -> Nullable<Text>,
        cosmos_address -> Nullable<Text>,   
        cert -> Nullable<Text>,
        tx -> Nullable<Text>,        
    }
}

table! {
    person_aggregates (id) {
        id -> Uuid,
        person_id -> Uuid,
        post_count -> Int8,
        post_score -> Int8,
        comment_count -> Int8,
        comment_score -> Int8,
    }
}

table! {
    person_ban (id) {
        id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    person_mention (id) {
        id -> Uuid,
        recipient_id -> Uuid,
        comment_id -> Uuid,
        read -> Bool,
        published -> Timestamp,
    }
}

table! {
    comment_reply (id) {
        id -> Uuid,
        recipient_id -> Uuid,
        comment_id -> Uuid,
        read -> Bool,
        published -> Timestamp,
    }
}

table! {
    post (id) {
        id -> Uuid,
        name -> Varchar,
        url -> Nullable<Varchar>,
        body -> Nullable<Text>,
        creator_id -> Uuid,
        community_id -> Uuid,
        removed -> Bool,
        locked -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Bool,
        nsfw -> Bool,
        stickied -> Bool,
        embed_title -> Nullable<Text>,
        embed_description -> Nullable<Text>,
        embed_video_url -> Nullable<Text>,
        thumbnail_url -> Nullable<Text>,
        ap_id -> Varchar,
        local -> Bool,
        language_id -> Int4,

        //private_id -> Uuid,
        cert -> Nullable<Text>,
        tx -> Nullable<Text>,
    }
}

table! {
    post_aggregates (id) {
        id -> Uuid,
        post_id -> Uuid,
        comments -> Int8,
        score -> Int8,
        upvotes -> Int8,
        downvotes -> Int8,
        stickied -> Bool,
        published -> Timestamp,
        newest_comment_time_necro -> Timestamp,
        newest_comment_time -> Timestamp,
    }
}

table! {
    post_like (id) {
        id -> Uuid,
        post_id -> Uuid,
        person_id -> Uuid,
        score -> Int2,
        published -> Timestamp,
    }
}

table! {
    post_read (id) {
        id -> Uuid,
        post_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    post_report (id) {
        id -> Uuid,
        creator_id -> Uuid,
        post_id -> Uuid,
        original_post_name -> Varchar,
        original_post_url -> Nullable<Text>,
        original_post_body -> Nullable<Text>,
        reason -> Text,
        resolved -> Bool,
        resolver_id -> Nullable<Uuid>,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    post_saved (id) {
        id -> Uuid,
        post_id -> Uuid,
        person_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    private_message (id) {
        id -> Uuid,
        creator_id -> Uuid,
        recipient_id -> Uuid,
        content -> Text,
        deleted -> Bool,
        read -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        ap_id -> Varchar,
        local -> Bool,
        secured -> Nullable<Text>,
        cert -> Nullable<Text>,
        tx -> Nullable<Text>,
    }
}

table! {
    private_message_report (id) {
        id -> Uuid,
        creator_id -> Uuid,
        private_message_id -> Uuid,
        original_pm_text -> Text,
        reason -> Text,
        resolved -> Bool,
        resolver_id -> Nullable<Uuid>,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    site (id) {
        id -> Uuid,
        name -> Varchar,
        sidebar -> Nullable<Text>,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        enable_downvotes -> Bool,
        open_registration -> Bool,
        enable_nsfw -> Bool,
        icon -> Nullable<Varchar>,
        banner -> Nullable<Varchar>,
        description -> Nullable<Text>,
        community_creation_admin_only -> Bool,
        require_email_verification -> Bool,
        require_application -> Bool,
        application_question -> Nullable<Text>,
        private_instance -> Bool,
        actor_id -> Text,
        last_refreshed_at -> Timestamp,
        inbox_url -> Text,
        private_key -> Nullable<Text>,
        public_key -> Text,
        default_theme -> Text,
        default_post_listing_type -> Text,
        legal_information -> Nullable<Text>,
        hide_modlog_mod_names -> Bool,
    }
}

table! {
    site_aggregates (id) {
        id -> Uuid,
        site_id -> Uuid,
        users -> Int8,
        posts -> Int8,
        comments -> Int8,
        communities -> Int8,
        users_active_day -> Int8,
        users_active_week -> Int8,
        users_active_month -> Int8,
        users_active_half_year -> Int8,
    }
}

table! {
    person_block (id) {
        id -> Uuid,
        person_id -> Uuid,
        target_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
    community_block (id) {
        id -> Uuid,
        person_id -> Uuid,
        community_id -> Uuid,
        published -> Timestamp,
    }
}

table! {
  secret(id) {
    id -> Uuid,
    jwt_secret -> Varchar,
  }
}

table! {
  admin_purge_comment (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    post_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

table! {
  email_verification (id) {
    id -> Uuid,
    local_user_id -> Uuid,
    email -> Text,
    verification_token -> Varchar,
    published -> Timestamp,
  }
}

table! {
  admin_purge_community (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

table! {
  admin_purge_person (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

table! {
  admin_purge_post (id) {
    id -> Uuid,
    admin_person_id -> Uuid,
    community_id -> Uuid,
    reason -> Nullable<Text>,
    when_ -> Timestamp,
  }
}

table! {
    registration_application (id) {
        id -> Uuid,
        local_user_id -> Uuid,
        answer -> Text,
        admin_id -> Nullable<Uuid>,
        deny_reason -> Nullable<Text>,
        published -> Timestamp,
    }
}

table! {
    mod_hide_community (id) {
        id -> Uuid,
        community_id -> Uuid,
        mod_person_id -> Uuid,
        reason -> Nullable<Text>,
        hidden -> Nullable<Bool>,
        when_ -> Timestamp,
    }
}

table! {
    pipayment (id) {
        id -> Uuid,
        person_id -> Nullable<Uuid>, // WePi user's id
        ref_id -> Nullable<Uuid>,    // Captchar id
        testnet -> Bool,
        finished -> Bool,
        published -> Timestamp,
        updated -> Nullable<Timestamp>,
        comment -> Nullable<Text>,

        pi_uid -> Nullable<Uuid>,
        pi_username -> Text,
        identifier -> Text,
        user_uid -> Text, //  PaymentDto
        amount -> Double,
        memo -> Text,
        to_address -> Text,
        created_at -> Nullable<Timestamp>,

        approved -> Bool,
        verified -> Bool,
        completed -> Bool,
        cancelled -> Bool,
        user_cancelled -> Bool,
        tx_verified -> Bool,
        tx_link -> Text,
        tx_id -> Text,
        metadata -> Nullable<Jsonb>,
        extras -> Nullable<Jsonb>,
    }
}

//joinable!(pipayment -> person (person_id));

table! {
    language (id) {
        id -> Int4,
        code -> Text,
        name -> Text,
    }
}


table! {
    local_user_language(id) {
        id -> Uuid,
        local_user_id -> Uuid,
        language_id -> Int4,
    }
}

joinable!(person_block -> person (person_id));

joinable!(comment -> person (creator_id));
joinable!(comment -> post (post_id));
joinable!(comment_aggregates -> comment (comment_id));
joinable!(comment_like -> comment (comment_id));
joinable!(comment_like -> person (person_id));
joinable!(comment_like -> post (post_id));
joinable!(comment_report -> comment (comment_id));
joinable!(comment_saved -> comment (comment_id));
joinable!(comment_saved -> person (person_id));
joinable!(community_aggregates -> community (community_id));
joinable!(community_block -> community (community_id));
joinable!(community_block -> person (person_id));
joinable!(community_follower -> community (community_id));
joinable!(community_follower -> person (person_id));
joinable!(community_moderator -> community (community_id));
joinable!(community_moderator -> person (person_id));
joinable!(community_person_ban -> community (community_id));
joinable!(community_person_ban -> person (person_id));
joinable!(local_user -> person (person_id));
joinable!(mod_add_community -> community (community_id));
joinable!(mod_transfer_community -> community (community_id));
joinable!(mod_ban_from_community -> community (community_id));
joinable!(mod_lock_post -> person (mod_person_id));
joinable!(mod_lock_post -> post (post_id));
joinable!(mod_remove_comment -> comment (comment_id));
joinable!(mod_remove_comment -> person (mod_person_id));
joinable!(mod_remove_community -> community (community_id));
joinable!(mod_remove_community -> person (mod_person_id));
joinable!(mod_remove_post -> person (mod_person_id));
joinable!(mod_remove_post -> post (post_id));
joinable!(mod_sticky_post -> person (mod_person_id));
joinable!(mod_sticky_post -> post (post_id));
joinable!(password_reset_request -> local_user (local_user_id));
joinable!(person_aggregates -> person (person_id));
joinable!(person_ban -> person (person_id));
joinable!(person_mention -> comment (comment_id));
joinable!(person_mention -> person (recipient_id));
joinable!(comment_reply -> comment (comment_id));
joinable!(comment_reply -> person (recipient_id));
joinable!(post -> community (community_id));
joinable!(post -> person (creator_id));
joinable!(post_aggregates -> post (post_id));
joinable!(post_like -> person (person_id));
joinable!(post_like -> post (post_id));
joinable!(post_read -> person (person_id));
joinable!(post_read -> post (post_id));
joinable!(post_report -> post (post_id));
joinable!(post_saved -> person (person_id));
joinable!(post_saved -> post (post_id));
joinable!(site_aggregates -> site (site_id));
joinable!(email_verification -> local_user (local_user_id));
joinable!(registration_application -> local_user (local_user_id));
joinable!(registration_application -> person (admin_id));
joinable!(mod_hide_community -> person (mod_person_id));
joinable!(mod_hide_community -> community (community_id));
joinable!(post -> language (language_id));
joinable!(comment -> language (language_id));
joinable!(local_user_language -> language (language_id));
joinable!(local_user_language -> local_user (local_user_id));
joinable!(private_message_report -> private_message (private_message_id));

joinable!(admin_purge_comment -> person (admin_person_id));
joinable!(admin_purge_comment -> post (post_id));
joinable!(admin_purge_community -> person (admin_person_id));
joinable!(admin_purge_person -> person (admin_person_id));
joinable!(admin_purge_post -> community (community_id));
joinable!(admin_purge_post -> person (admin_person_id));

allow_tables_to_appear_in_same_query!(
  activity,
  comment,
  comment_aggregates,
  community_block,
  comment_like,
  comment_report,
  comment_saved,
  community,
  community_aggregates,
  community_follower,
  community_moderator,
  community_person_ban,
  local_user,
  mod_add,
  mod_add_community,
  mod_transfer_community,
  mod_ban,
  mod_ban_from_community,
  mod_lock_post,
  mod_remove_comment,
  mod_remove_community,
  mod_remove_post,
  mod_sticky_post,
  mod_hide_community,
  password_reset_request,
  person,
  person_aggregates,
  person_ban,
  person_block,
  person_mention,
  comment_reply,
  post,
  post_aggregates,
  post_like,
  post_read,
  post_report,
  post_saved,
  private_message,
  private_message_report,
  site,
  site_aggregates,
  admin_purge_comment,
  admin_purge_community,
  admin_purge_person,
  admin_purge_post,
  email_verification,
  registration_application,
  language,
  local_user_language,
  pipayment
);
