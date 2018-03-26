#![feature(plugin, decl_macro, custom_derive)]

#[macro_use]
extern crate quicli;

#[macro_use]
extern crate diesel;

extern crate data_encoding;
extern crate ring;

mod schema;

use quicli::prelude::*;
use diesel::Connection;
use diesel::pg::PgConnection;
use std::env;

use ring::{digest, rand, pbkdf2};
static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

pub mod model {
    #[derive(Queryable)]
    pub struct Salt {
        pub id: i32,
        pub salt: String,
    }

    #[derive(Queryable)]
    pub struct User {
        pub id: i32,
        pub email: String,
        pub password: String,
    }
}

fn establish_connection() -> PgConnection {
    let connect = || -> Result<PgConnection> {
        let env_var = env::var("DATABASE_URL")
            .with_context(|e| format!("Failed to parse env variable DATABASE_URL\n => {}", e))?;

        let connection = PgConnection::establish(&env_var[..])
            .with_context(|e| format!("Failed to connect to database\n => {}", e))?;

        Ok(connection)
    };

    connect().unwrap_or_else(|e| {
        println!("{}", e);
        panic!();
    })
}

fn salt_component() -> String {
    use diesel::prelude::*;
    use schema::salts::dsl::*;

    let connection = establish_connection();

    let salt_string = || -> Result<String> {
        let result = salts
            .first::<model::Salt>(&connection)
            .with_context(|e| format!("Failed to read salt from database\n => {}", e))?;

        Ok(result.salt.to_string())
    };

    salt_string().unwrap_or_else(|e| {
        println!("{}", e);
        panic!();
    })
}

fn salt(username: &str) -> Vec<u8> {
    let salt = || -> Result<Vec<u8>> {
        let db_salt = salt_component();
        let mut res = Vec::with_capacity(username.as_bytes().len() + db_salt.as_bytes().len());

        res.extend(db_salt.as_bytes());
        res.extend(username.as_bytes());

        Ok(res)
    };

    salt().unwrap_or_else(|e| {
        println!("{}", e);
        panic!();
    })
}

fn save<'u>(e: &'u str, p: &'u str) {
    use diesel::prelude::*;
    use schema::users::dsl::*;

    let connection = establish_connection();

    let new_user = (email.eq(e), password.eq(p));

    let _ = diesel::insert_into(users)
        .values(&new_user)
        .execute(&connection);
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "user", short = "u", parse(from_str))]
    user: String,

    #[structopt(long = "password", short = "p", parse(from_str))]
    password: String,
}

main!(|args: Cli| {
    let salt = salt(&args.user[..]);
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        DIGEST_ALG,
        100_000,
        &salt,
        args.password.as_bytes(),
        &mut to_store,
    );

    save(&args.user, &data_encoding::HEXUPPER.encode(&to_store));
});
