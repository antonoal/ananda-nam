use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Local;
use diesel::deserialize::QueryableByName;
use diesel::{prelude::*, sql_query, sql_types::*};
use serde::{Deserialize, Serialize};

use crate::schema::persons;
use crate::{errors::AppError, AppState};

#[derive(Debug, Serialize, Deserialize, QueryableByName)]
pub struct Person {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub email: String,
    #[diesel(sql_type = Text)]
    pub name: String,
}

impl Person {
    fn get_all(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let res = sql_query(
            r#"
            select p.id,
                p.email,
                p.name
            from persons p
            where p.valid_from <= now()
                and p.valid_to > now()
            order by p.name
            "#,
        )
        .load::<Self>(conn)?;

        Ok(res)
    }

    fn update(&self, conn: &mut PgConnection, id: i32) -> Result<(), AppError> {
        use persons::dsl;
        let now = Local::now().naive_utc();
        conn.transaction(|conn| {
            diesel::update(persons::table)
                .filter(
                    dsl::id
                        .eq(id)
                        .and(dsl::valid_from.ge(&now))
                        .and(dsl::valid_to.lt(&now)),
                )
                .set(dsl::valid_to.eq(&now))
                .execute(conn)?;

            diesel::insert_into(persons::table)
                .values((
                    dsl::id.eq(id),
                    dsl::email.eq(&self.email),
                    dsl::name.eq(&self.name),
                    dsl::valid_from.eq(&now),
                ))
                .execute(conn)
        })?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct NewPerson {
    name: String,
    email: String,
    password: String,
}

impl NewPerson {
    fn add(&self, conn: &mut PgConnection) -> Result<i32, AppError> {
        use persons::dsl;
        let id = diesel::insert_into(persons::table)
            .values((
                dsl::email.eq(&self.email),
                dsl::name.eq(&self.name),
                dsl::password_hash.eq(&self.password), //TODO: change to hash
                dsl::valid_from.eq(Local::now().naive_utc()),
            ))
            .returning(dsl::id)
            .get_result(conn)?;

        Ok(id)
    }
}

pub async fn test(req: HttpRequest) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(()))
}

pub async fn all_persons(
    state: web::Data<AppState>,
    _req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    // let user = auth::get_current_user(&req)?;
    // let token = user.generate_token()?;
    // let res = UserResponse::from((user, token));
    // Ok(HttpResponse::Ok().json(res))
    let persons = web::block(move || Person::get_all(&mut conn)).await??;

    Ok(HttpResponse::Ok().json(PersonsResponse { persons }))
}

pub async fn add_person(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<AddPersonRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let id = web::block(move || form.person.add(&mut conn)).await??;
    Ok(HttpResponse::Ok().json(AddPersonsResponse { id }))
}

pub async fn update_person(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<UpdatePersonRequest>,
    path: web::Path<IdSlug>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let id = path.into_inner();
    web::block(move || form.person.update(&mut conn, id)).await??;
    Ok(HttpResponse::Ok().json(()))
}

type IdSlug = i32;

#[derive(Debug, Serialize)]
pub struct PersonsResponse {
    pub persons: Vec<Person>,
}

#[derive(Debug, Serialize)]
pub struct AddPersonsResponse {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub struct AddPersonRequest {
    pub person: NewPerson,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePersonRequest {
    pub person: Person,
}
