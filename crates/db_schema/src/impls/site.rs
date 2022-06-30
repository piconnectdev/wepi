use crate::{newtypes::DbUrl, source::site::*, traits::Crud};
use diesel::{dsl::*, result::Error, *};
use url::Url;

impl Crud for Site {
  type Form = SiteForm;
  type IdType = i64;
  fn read(conn: &PgConnection, _site_id: i64) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    site.first::<Self>(conn)
  }

  fn create(conn: &PgConnection, new_site: &SiteForm) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    insert_into(site).values(new_site).get_result::<Self>(conn)
  }

  fn update(conn: &PgConnection, site_id: i64, new_site: &SiteForm) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    diesel::update(site.find(site_id))
      .set(new_site)
      .get_result::<Self>(conn)
  }
  fn delete(conn: &PgConnection, site_id: i64) -> Result<usize, Error> {
    use crate::schema::site::dsl::*;
    diesel::delete(site.find(site_id)).execute(conn)
  }
}

impl Site {
  pub fn read_local_site(conn: &PgConnection) -> Result<Self, Error> {
    use crate::schema::site::dsl::*;
    site.order_by(id).first::<Self>(conn)
  }

  pub fn upsert(conn: &PgConnection, site_form: &SiteForm) -> Result<Site, Error> {
    use crate::schema::site::dsl::*;
    insert_into(site)
      .values(site_form)
      .on_conflict(actor_id)
      .do_update()
      .set(site_form)
      .get_result::<Self>(conn)
  }

  pub fn read_from_apub_id(conn: &PgConnection, object_id: Url) -> Result<Option<Self>, Error> {
    use crate::schema::site::dsl::*;
    let object_id: DbUrl = object_id.into();
    Ok(
      site
        .filter(actor_id.eq(object_id))
        .first::<Site>(conn)
        .ok()
        .map(Into::into),
    )
  }

  pub fn read_remote_sites(conn: &PgConnection) -> Result<Vec<Self>, Error> {
    use crate::schema::site::dsl::*;
    site.order_by(id).offset(1).get_results::<Self>(conn)
  }
}
