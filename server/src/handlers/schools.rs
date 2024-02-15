use crate::{errors::AppError, middleware::auth, schema::schools, AppState};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Local, NaiveDateTime};
use diesel::{prelude::*, result::Error as DieselError, sql_query, sql_types::*};
use serde::{Deserialize, Serialize};

use super::IdSlug;

pub async fn get_all(
    state: web::Data<AppState>,
    req: HttpRequest, //TODO: enforce access rights on the backend!
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let user_id = dbg!(auth::get_current_user_id(&req)?);
    let schools = web::block(move || School::get_all(&mut conn, &user_id)).await??;
    Ok(HttpResponse::Ok().json(GetAllResponse { schools }))
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<UpsertRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let id = web::block(move || School::create(&mut conn, &form.school)).await??;
    Ok(HttpResponse::Ok().json(CreateResponse { id }))
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<UpsertRequest>,
    path: web::Path<IdSlug>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let id = path.into_inner();
    let id = web::block(move || School::update(&mut conn, id, &form.school)).await??;
    Ok(HttpResponse::Ok().json(CreateResponse { id }))
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<IdSlug>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let id = path.into_inner();
    web::block(move || School::delete(&mut conn, id)).await??;
    Ok(HttpResponse::Ok().json(()))
}

#[derive(Debug, Serialize, QueryableByName)]
struct School {
    #[diesel(sql_type = Int4)]
    id: i32,
    #[diesel(sql_type = Text)]
    name: String,
}

impl School {
    fn get_all(conn: &mut PgConnection, user_id: &i32) -> Result<Vec<Self>, AppError> {
        let res = sql_query(
            r#"
            select s.id,
                s.name
            from schools s
            where s.valid_from <= now()
                and s.valid_to > now()
            order by s.name
            "#,
        )
        .load::<Self>(conn)?;

        Ok(res)
    }

    fn create(conn: &mut PgConnection, form: &NewSchool) -> Result<i32, AppError> {
        use schools::dsl;
        let valid_from = Local::now().naive_utc();
        let id: i32 = diesel::insert_into(schools::table)
            .values((dsl::name.eq(&form.name), dsl::valid_from.eq(&valid_from)))
            .returning(dsl::id)
            .get_result(conn)?;

        Ok(id)
    }

    fn update(conn: &mut PgConnection, id: i32, form: &NewSchool) -> Result<i32, AppError> {
        use schools::dsl;
        let now: NaiveDateTime = Local::now().naive_utc();

        conn.transaction(|conn| {
            Self::expire(conn, id, Some(now))?;

            diesel::insert_into(schools::table)
                .values((
                    dsl::id.eq(id),
                    dsl::name.eq(&form.name),
                    dsl::valid_from.eq(&now),
                ))
                .execute(conn)
        })?;

        Ok(id)
    }

    fn delete(conn: &mut PgConnection, id: i32) -> Result<(), AppError> {
        Self::expire(conn, id, None)?;

        Ok(())
    }

    fn expire(
        conn: &mut PgConnection,
        id: i32,
        valid_to: Option<NaiveDateTime>,
    ) -> Result<(), DieselError> {
        use schools::dsl;
        let valid_to = valid_to.unwrap_or_else(|| Local::now().naive_utc());

        diesel::update(schools::table)
            .filter(
                dsl::id
                    .eq(id)
                    .and(dsl::valid_from.le(&valid_to))
                    .and(dsl::valid_to.gt(&valid_to)),
            )
            .set(dsl::valid_to.eq(&valid_to))
            .execute(conn)?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct GetAllResponse {
    schools: Vec<School>,
}

#[derive(Debug, Serialize)]
struct CreateResponse {
    id: i32,
}

#[derive(Debug, Deserialize)]
pub struct NewSchool {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpsertRequest {
    school: NewSchool,
}
