use crate::{errors::AppError, schema::groups, AppState};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Local, NaiveDateTime};
use diesel::{prelude::*, result::Error as DieselError, sql_query, sql_types::*};
use serde::{Deserialize, Serialize};

pub async fn get_all(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let school_id = path.into_inner();
    let groups = web::block(move || Group::get_all(&mut conn, school_id)).await??;
    Ok(HttpResponse::Ok().json(GetAllResponse { groups }))
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<UpsertRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let id = web::block(move || Group::create(&mut conn, &form.group)).await??;
    Ok(HttpResponse::Ok().json(CreateResponse { id }))
}

// pub async fn update(
//     state: web::Data<AppState>,
//     req: HttpRequest,
//     form: web::Json<UpsertRequest>,
//     path: web::Path<(i32, i32)>,
// ) -> Result<HttpResponse, AppError> {
//     let mut conn = state.get_conn()?;
//     let (school_id, id) = path.into_inner();
//     let id = web::block(move || Stream::update(&mut conn, school_id, id, &form.stream)).await??;
//     Ok(HttpResponse::Ok().json(CreateResponse { id }))
// }

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let (_, id) = path.into_inner();
    web::block(move || Group::delete(&mut conn, id)).await??;
    Ok(HttpResponse::Ok().json(()))
}

#[derive(Debug, Serialize, QueryableByName)]
struct Group {
    #[diesel(sql_type = Int4)]
    id: i32,
    #[diesel(sql_type = Nullable<Int4>)]
    parent_id: Option<i32>,
    #[diesel(sql_type = Nullable<Int4>)]
    year_id: Option<i32>,
    #[diesel(sql_type = Int4)]
    stream_id: i32,
    #[diesel(sql_type = Text)]
    name: String,
}

impl Group {
    fn get_all(conn: &mut PgConnection, school_id: i32) -> Result<Vec<Self>, AppError> {
        let res = sql_query(
            r#"
            select
                g.*
            from
                groups g
            join streams s on
                s.id = g.stream_id
            where
                s.school_id = $1
                and s.valid_from <= now()
                and s.valid_to > now()
                and g.valid_from <= now()
                and g.valid_to > now()
            order by
                g.name
            "#,
        )
        .bind::<Integer, _>(school_id)
        .load::<Self>(conn)?;

        Ok(res)
    }

    fn create(conn: &mut PgConnection, form: &NewGroup) -> Result<i32, AppError> {
        use groups::dsl;
        let valid_from = Local::now().naive_utc();
        let id: i32 = diesel::insert_into(groups::table)
            .values((
                dsl::parent_id.eq(&form.parent_id),
                dsl::stream_id.eq(&form.stream_id),
                dsl::year_id.eq(&form.year_id),
                dsl::name.eq(&form.name),
                dsl::valid_from.eq(valid_from),
            ))
            .returning(dsl::id)
            .get_result(conn)?;

        Ok(id)
    }

    // fn update(
    //     conn: &mut PgConnection,
    //     school_id: i32,
    //     id: i32,
    //     form: &NewStream,
    // ) -> Result<i32, AppError> {
    //     use streams::dsl;
    //     let now: NaiveDateTime = Local::now().naive_utc();

    //     conn.transaction(|conn| {
    //         Self::expire(conn, id, Some(now))?;

    //         diesel::insert_into(streams::table)
    //             .values((
    //                 dsl::id.eq(id),
    //                 dsl::school_id.eq(school_id),
    //                 dsl::name.eq(&form.name),
    //                 dsl::start_year.eq(&form.start_year),
    //                 dsl::valid_from.eq(&now),
    //             ))
    //             .execute(conn)
    //     })?;

    //     Ok(id)
    // }

    fn delete(conn: &mut PgConnection, id: i32) -> Result<(), AppError> {
        let row_cnt = Self::expire(conn, id, None)?;
        // TODO: delete should not leave children dangling.
        //      Either should expire them or make them children
        //      of the same entity the one being deleted is a child of

        // TODO: do the same in the rest of the modules
        if row_cnt == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(())
        }
    }

    fn expire(
        conn: &mut PgConnection,
        id: i32,
        valid_to: Option<NaiveDateTime>,
    ) -> Result<usize, DieselError> {
        use groups::dsl;
        let valid_to = valid_to.unwrap_or_else(|| Local::now().naive_utc());

        let res = diesel::update(groups::table)
            .filter(
                dsl::id
                    .eq(id)
                    .and(dsl::valid_from.le(&valid_to))
                    .and(dsl::valid_to.gt(&valid_to)),
            )
            .set(dsl::valid_to.eq(&valid_to))
            .execute(conn)?;

        Ok(res)
    }
}

#[derive(Debug, Serialize)]
struct GetAllResponse {
    groups: Vec<Group>,
}

#[derive(Debug, Serialize)]
struct CreateResponse {
    id: i32,
}

#[derive(Debug, Deserialize)]
pub struct NewGroup {
    parent_id: Option<i32>,
    year_id: Option<i32>,
    stream_id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpsertRequest {
    group: NewGroup,
}
