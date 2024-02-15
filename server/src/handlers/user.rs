use actix_web::{web, HttpResponse};
use chrono::Utc;
use diesel::{prelude::*, sql_query, sql_types::*};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    schema::persons,
    utils::{hasher, token},
    AppState,
};

type Token = String;

pub async fn login(
    state: web::Data<AppState>,
    form: web::Json<Signin>,
) -> Result<HttpResponse, AppError> {
    let mut conn = state.get_conn()?;
    let (user, token) =
        web::block(move || User::login(&mut conn, &form.email, &form.password)).await??;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

enum FindFilter<'a> {
    Id(i32),
    Email(&'a str),
}

#[derive(Debug, QueryableByName, PartialEq, Clone)]
#[diesel(table_name = persons)]
pub struct User {
    pub id: i32,
    name: String,
    email: String,
    password_hash: Option<String>,
    #[diesel(sql_type = Array<Text>)]
    privileges: Vec<String>,
}

impl User {
    pub fn get(conn: &mut PgConnection, id: i32) -> Result<Self, AppError> {
        Self::find(conn, FindFilter::Id(id))
    }

    fn find(conn: &mut PgConnection, filter: FindFilter) -> Result<Self, AppError> {
        let filter_column = match filter {
            FindFilter::Id(_) => "id",
            FindFilter::Email(_) => "email",
        };

        let sql_text = format!(
            r#"
            select p.id,
                p.name,
                p.email,
                p.password_hash,
                array_agg(p2."name") as privileges
            from persons p
                left join person_roles pr ON p.id = pr.person_id
                left join role_privileges rp on rp.role_id = pr.role_id
                left join "privileges" p2 on p2.id = rp.privilege_id
            where p.{filter_column} = $1
                and p.valid_from <= now()
                and p.valid_to > now()
                and p."locked" = false
            group by p.id,
                p.name,
                p.email,
                p.password_hash
        "#
        );

        let user = match filter {
            FindFilter::Id(id) => sql_query(sql_text)
                .bind::<Integer, _>(id)
                .get_result::<User>(conn)?,
            FindFilter::Email(email) => sql_query(sql_text)
                .bind::<Text, _>(&email)
                .get_result::<User>(conn)?,
        };

        Ok(user)
    }

    fn login(
        conn: &mut PgConnection,
        email: &str,
        naive_password: &str,
    ) -> Result<(User, Token), AppError> {
        let user = Self::find(conn, FindFilter::Email(email))?;

        let password_matches = if let Some(hash) = user.password_hash.as_ref() {
            hasher::verify(naive_password, hash)?
        } else {
            return Err(AppError::Unauthorized("Empty password".into()));
        };

        if !password_matches {
            return Err(AppError::Unauthorized("Invalid password".into()));
        }

        let token = user.generate_token()?;
        Ok((user, token))
    }

    fn generate_token(&self) -> Result<Token, AppError> {
        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second
        let token = token::generate(self.id, now)?;
        Ok(token)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Signin {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct AuthedUser {
    id: i32,
    name: String,
    email: String,
    privileges: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserResponse {
    user: AuthedUser,
    token: String,
}

impl From<(User, Token)> for UserResponse {
    fn from(
        (
            User {
                id,
                name,
                email,
                password_hash: _,
                privileges,
            },
            token,
        ): (User, Token),
    ) -> Self {
        Self {
            user: AuthedUser {
                id,
                name,
                email,
                privileges,
            },
            token,
        }
    }
}
