use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde::{
    Serialize,
    Deserialize,
};
use crate::schema::logs;
use chrono::NaiveDateTime;
use crate::server::errors::{
    Result,
    ApiError,
};
use std::str::FromStr;

#[derive(DbEnum, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[PgType = "logging_level"]
#[DieselType = "Logging_level"]
pub enum LoggingLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl FromStr for LoggingLevel {
    type Err = ApiError;

    fn from_str(input: &str) -> std::result::Result<LoggingLevel, Self::Err> {
        match input {
            "Debug"   => Ok(Self::Debug),
            "Info"    => Ok(Self::Info),
            "Warning" => Ok(Self::Warning),
            "Error"   => Ok(Self::Error),
            _ => Err(ApiError {
                code: 500,
                message: "Wrong value of logging level".to_string(),
                error_type: crate::server::errors::ErrorType::ValueError,
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Queryable, Identifiable)]
#[table_name = "logs"]
#[primary_key(id)]
pub struct LogNote {
    pub id: i32,
    pub message: String,
    pub level: LoggingLevel,
    pub datetime: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Queryable, Insertable)]
#[table_name = "logs"]
pub struct NewLogNote {
    pub message: String,
    pub level: LoggingLevel,
}

impl LogNote {
    pub async fn new(
        instance: &NewLogNote,
        conn: &PgConnection,
    ) -> Result<usize> {
        Ok(diesel::insert_into(logs::table)
            .values(instance)
            .execute(conn)?
        )
    }

    pub async fn get(
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        logs::table
            .order_by(logs::datetime.desc())
            .load(conn)
            .map_err(|e| e.into())
    }

    pub async fn get_range(
        from_time: NaiveDateTime,
        to_time: NaiveDateTime,
        logging_level: LoggingLevel,
        conn: &PgConnection,
    ) -> Result<Vec<Self>> {
        logs::table
            .filter(logs::level.eq(logging_level))
            .filter(logs::datetime.between(from_time, to_time))
            .order_by(logs::datetime.desc())
            .load(conn)
            .map_err(|e| e.into())
    }
}