use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use crate::AppError;


#[derive(sqlx::FromRow)]
pub struct BasTable {
    pub table_name: String, 
    pub source_id: i32, 
    pub source_name: String,
}


#[derive(sqlx::FromRow)]
pub struct LinkedRec {
    pub count: i32, 
    pub array_agg: Vec<String>,
}


pub struct OutputRec {
    pub pri_source: i32,
    pub pri_sid: String, 
    pub sec_source: i32,
    pub sec_sid: String,
}

pub struct OutputRecs {
    pub pri_sources: Vec<i32>,
    pub pri_sids: Vec<String>, 
    pub sec_sources: Vec<i32>,
    pub sec_sids: Vec<String>,
}

impl OutputRecs{
    pub fn new(vsize: usize) -> Self {
        OutputRecs { 
            pri_sources: Vec::with_capacity(vsize),
            pri_sids: Vec::with_capacity(vsize),
            sec_sources: Vec::with_capacity(vsize),
            sec_sids: Vec::with_capacity(vsize),
        }
    }


    pub fn add_rec(&mut self, r: &OutputRec) 
    {
        self.pri_sources.push(r.pri_source);
        self.pri_sids.push(r.pri_sid.clone());
        self.sec_sources.push(r.sec_source);
        self.sec_sids.push(r.sec_sid.clone());
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {
    
        let sql = r#"INSERT INTO sec.new_recs (pri_sid_type, pri_sid, sec_sid_type, sec_sid)
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::int[], $4::text[])"#;

        sqlx::query(sql)
            .bind(&self.pri_sources)
            .bind(&self.pri_sids)
            .bind(&self.sec_sources)
            .bind(&self.sec_sids)
            .execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}

