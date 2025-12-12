use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use crate::AppError;


#[derive(sqlx::FromRow)]
pub struct BasTable {
    pub table_name: String, 
}

#[derive(sqlx::FromRow)]
pub struct LinkedRec {
    pub count: i32, 
    pub array_agg: Vec<String>,
}


pub struct OutputRec {
    pub pri_sid_type: i32,
    pub pri_sid: String, 
    pub sec_sid_type: i32,
    pub sec_sid: String,
}

pub struct OutputRecs {
    pub pri_sid_types: Vec<i32>,
    pub pri_sids: Vec<String>, 
    pub sec_sid_types: Vec<i32>,
    pub sec_sids: Vec<String>,
}

impl OutputRecs{
    pub fn new(vsize: usize) -> Self {
        OutputRecs { 
            pri_sid_types: Vec::with_capacity(vsize),
            pri_sids: Vec::with_capacity(vsize),
            sec_sid_types: Vec::with_capacity(vsize),
            sec_sids: Vec::with_capacity(vsize),
        }
    }


    pub fn add_rec(&mut self, r: &OutputRec) 
    {
        self.pri_sid_types.push(r.pri_sid_type);
        self.pri_sids.push(r.pri_sid.clone());
        self.sec_sid_types.push(r.sec_sid_type);
        self.sec_sids.push(r.sec_sid.clone());
    }


    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {
    
        let sql = r#"INSERT INTO sec.new_recs (pri_sid_type, pri_sid, sec_sid_type, sec_sid)
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::int[], $4::text[])"#;

        sqlx::query(sql)
            .bind(&self.pri_sid_types)
            .bind(&self.pri_sids)
            .bind(&self.sec_sid_types)
            .bind(&self.sec_sids)
            .execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }

}

