use crate::structs::{ModLockPostView, ModlogListParams};
use diesel::{
  result::Error,
  BoolExpressionMethods,
  ExpressionMethods,
  IntoSql,
  JoinOnDsl,
  NullableExpressionMethods,
  QueryDsl,
};
use diesel_async::RunQueryDsl;
use lemmy_db_schema::{
  newtypes::PersonId,
  schema::{community, mod_lock_post, person, post},
  source::{
    community::{Community, CommunitySafe},
    moderator::ModLockPost,
    person::{Person, PersonSafe},
    post::Post,
  },
  traits::{ToSafe, ViewToVec},
  utils::{get_conn, limit_and_offset, DbPool},
};

type ModLockPostViewTuple = (ModLockPost, Option<PersonSafe>, Post, CommunitySafe);

impl ModLockPostView {
  pub async fn list(pool: &DbPool, params: ModlogListParams) -> Result<Vec<Self>, Error> {
    let conn = &mut get_conn(pool).await?;
    let uuid = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
    let person_alias_1 = diesel::alias!(person as person1);
    let admin_person_id_join = params.mod_person_id.unwrap_or(PersonId(uuid.clone()));
    let show_mod_names = !params.hide_modlog_names;
    let show_mod_names_expr = show_mod_names.as_sql::<diesel::sql_types::Bool>();

    let admin_names_join = mod_lock_post::mod_person_id
      .eq(person::id)
      .and(show_mod_names_expr.or(person::id.eq(admin_person_id_join)));
    let mut query = mod_lock_post::table
      .left_join(person::table.on(admin_names_join))
      .inner_join(post::table)
      .inner_join(community::table.on(post::community_id.eq(community::id)))
      .inner_join(person_alias_1.on(post::creator_id.eq(person_alias_1.field(person::id))))
      .select((
        mod_lock_post::all_columns,
        Person::safe_columns_tuple().nullable(),
        post::all_columns,
        Community::safe_columns_tuple(),
      ))
      .into_boxed();

    if let Some(community_id) = params.community_id {
      query = query.filter(post::community_id.eq(community_id));
    };

    if let Some(mod_person_id) = params.mod_person_id {
      query = query.filter(mod_lock_post::mod_person_id.eq(mod_person_id));
    };

    if let Some(other_person_id) = params.other_person_id {
      query = query.filter(person_alias_1.field(person::id).eq(other_person_id));
    };

    let (limit, offset) = limit_and_offset(params.page, params.limit)?;

    let res = query
      .limit(limit)
      .offset(offset)
      .order_by(mod_lock_post::when_.desc())
      .load::<ModLockPostViewTuple>(conn)
      .await?;

    let results = Self::from_tuple_to_vec(res);
    Ok(results)
  }
}

impl ViewToVec for ModLockPostView {
  type DbTuple = ModLockPostViewTuple;
  fn from_tuple_to_vec(items: Vec<Self::DbTuple>) -> Vec<Self> {
    items
      .into_iter()
      .map(|a| Self {
        mod_lock_post: a.0,
        moderator: a.1,
        post: a.2,
        community: a.3,
      })
      .collect::<Vec<Self>>()
  }
}
