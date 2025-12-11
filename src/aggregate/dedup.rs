use sqlx::{Pool, Postgres};
use crate::AppError;

use super::structs::{BasTable, LinkedRec, OutputRec, OutputRecs};


async fn execute_sql(sql: &str, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(res.rows_affected())
}


pub async fn set_up_sec_id_tables (pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"SET client_min_messages TO WARNING;
                    create schema if not exists sec;"#;
    execute_sql(sql, pool).await?;

    let sql = r#"SET client_min_messages TO WARNING;
        drop table if exists sec.temp_tr_sec_ids;
        create table sec.temp_tr_sec_ids (
          pri_source     int4
        , pri_sid        varchar 
        , sec_source     int4
        , sec_sid        varchar
    );"#;
    execute_sql(sql, pool).await?;

    let sql = r#"SET client_min_messages TO WARNING;
        drop table if exists sec.temp_oth_sec_ids;
        create table sec.temp_oth_sec_ids (
          pri_source     int4
        , pri_sid        varchar 
        , sponsor        varchar
        , sec_id         varchar
    );"#;
    execute_sql(sql, pool).await?;

    Ok(())
}



pub async fn process_sec_ids(entry: &BasTable, pool: &Pool<Postgres>) -> Result<(u64, u64), AppError> {

    let _ = entry.source_id;
    let _ = entry.source_name; // to avoid warnings for now

    let sql = format!(r#"insert into sec.temp_tr_sec_ids (pri_source, pri_sid, sec_source, sec_sid)
        select source_id, sd_sid, SPLIT_PART(unnest(reg_sec_ids), '::', 1)::int4, SPLIT_PART(unnest(reg_sec_ids), '::', 2)
        from dat.{}
        where reg_sec_ids is not null
        order by sd_sid;"#, entry.table_name);

    let tr = execute_sql(&sql, pool).await?;
        
    let sql = format!(r#"insert into sec.temp_oth_sec_ids (pri_source, pri_sid, sponsor, sec_id)
        select source_id, sd_sid, sponsor_processed, unnest(oth_sec_ids) 
        from dat.{}
        where oth_sec_ids is not null
        order by sd_sid;"#, entry.table_name);

    let oth = execute_sql(&sql, pool).await?;

    Ok((tr, oth))
}


// Extract the secondary ids that have the same source TR into separate table

pub async fn separate_same_registry_secids(pool: &Pool<Postgres>) -> Result<u64, AppError> {
    
   let sql = format!(r#"drop table if exists sec.same_tr_sec_ids;
        create table sec.same_tr_sec_ids as 
        select * from sec.temp_tr_sec_ids
        where pri_source = sec_source
        order by pri_sid;"#);
    
    execute_sql(&sql, pool).await?;

    let sql = format!(r#"delete from sec.temp_tr_sec_ids
        where pri_source = sec_source"#);

    let n = execute_sql(&sql, pool).await?;

    Ok(n)
}
    

// Extract the secondary ids that are WHO U numbers into separate table
    
pub async fn separate_who_utn_secids(pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"drop table if exists sec.who_tr_sec_ids;
        create table sec.who_tr_sec_ids as 
        select * from sec.temp_tr_sec_ids
        where sec_source = 100115
        order by pri_sid;"#);
    
    execute_sql(&sql, pool).await?;

    let sql = format!(r#"delete from sec.temp_tr_sec_ids
        where sec_source = 100115"#);

    let n = execute_sql(&sql, pool).await?;

    Ok(n)
}

// Get the possible other matches using sponsor name and sponsor id
// Only get pairings - groups of more than 2 very likely to be funding grant ids
// (as, very probably, are some of the pairs)



pub async fn setup_utn_processing(pool: &Pool<Postgres>) -> Result<Vec<LinkedRec>, AppError> {

    let sql = r#"drop table if exists sec.new_recs;
        create table sec.new_recs (
            pri_source  int4,
            pri_sid     varchar,
            sec_source  int4,
            sec_sid     varchar
        );
        
        drop table if exists sec.temp_mult_utn;
        create table sec.temp_mult_utn (
            sec_sid     varchar,
            count       int4
        );

        drop table if exists sec.temp_mult_utn_dets;
        create table sec.temp_mult_utn_dets (
            sec_sid     varchar,
            count       int4,
            array_agg   varchar[]
        );"#;

    execute_sql(&sql, pool).await?;
    
    let sql = r#"insert into sec.temp_mult_utn(sec_sid, count)
        select sec_sid, count(pri_sid)
        from sec.who_tr_sec_ids
        group by sec_sid
        having count(pri_sid) > 1 and count(pri_sid) < 6
        order by count(pri_sid) desc, sec_sid;"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"insert into sec.temp_mult_utn_dets(sec_sid, count, array_agg)
        select m.sec_sid, count, array_agg(w.pri_source||'::'||w.pri_sid) 
        from sec.temp_mult_utn m
        inner join sec.who_tr_sec_ids w
        on m.sec_sid = w.sec_sid
        group by m.sec_sid, count
        order by m.sec_sid;"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"select count, array_agg 
    from sec.temp_mult_utn_dets;"#;

    sqlx::query_as(&sql).fetch_all(pool).await
             .map_err(|e| AppError::SqlxError(e, sql.to_string()))

}


pub async fn complete_utn_processing(pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = r#"insert into sec.temp_tr_sec_ids (pri_source, pri_sid, sec_source, sec_sid)
    select n.pri_source, n.pri_sid, n.sec_source, n.sec_sid
    from sec.new_recs n
    left join sec.temp_tr_sec_ids s
    on n.pri_source = s.pri_source and n.pri_sid = s.pri_sid
    and n.sec_source = s.sec_source and n.sec_sid = s.sec_sid
    where s.pri_source is null;"#;
        
    let n = execute_sql(&sql, pool).await?;
    
    let sql = r#"drop table if exists sec.temp_mult_utn;
        drop table if exists sec.temp_mult_utn_dets;
        truncate table sec.new_recs;"#;
        
    execute_sql(&sql, pool).await?;
    
    Ok(n)
}


pub async fn setup_sponsor_id_processing(pool: &Pool<Postgres>) -> Result<Vec<LinkedRec>, AppError> {

    let sql = r#"drop table if exists sec.temp_mult_sponsor_source_id;
    create table sec.temp_mult_sponsor_source_id (
                sponsor     varchar,
                sec_id      varchar,
                count_sids  int4,
                count_sources int4
    );

    drop table if exists sec.temp_mult_sponsor_id_dets;
    create table sec.temp_mult_sponsor_id_dets (
                sponsor     varchar,
                sec_id      varchar,
                count       int4,
                array_agg   varchar[]
    );"#;

    execute_sql(&sql, pool).await?;


    let sql = r#"insert into sec.temp_mult_sponsor_source_id(sponsor, sec_id, count_sids, count_sources)
        select sponsor, sec_id, count(pri_sid), count(distinct pri_source)
        from sec.temp_oth_sec_ids
        where length(sec_id) > 4
        and sec_id not in ('000000', '0000-00000', '11111')
        group by sponsor, sec_id
        having count(pri_sid) > 1  
        and count(distinct pri_source) = count(pri_sid)
        order by count(pri_sid) desc, sec_id;"#;

    execute_sql(&sql, pool).await?;


    let sql = r#"insert into sec.temp_mult_sponsor_id_dets(sec_id, sponsor, count, array_agg)
        select m.sec_id, m.sponsor, m.count_sids, array_agg(w.pri_source||'::'||w.pri_sid) 
        from sec.temp_mult_sponsor_source_id m
        inner join sec.temp_oth_sec_ids w
        on m.sec_id = w.sec_id
        and m.sponsor = w.sponsor
        group by m.sec_id, m.sponsor, m.count_sids
        order by m.sec_id;"#;

    execute_sql(&sql, pool).await?;


    let sql = r#"select count, array_agg 
    from sec.temp_mult_sponsor_id_dets;"#;

    sqlx::query_as(&sql).fetch_all(pool).await
             .map_err(|e| AppError::SqlxError(e, sql.to_string()))

}


pub async fn complete_sponsor_id_processing(pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = r#"insert into sec.temp_tr_sec_ids (pri_source, pri_sid, sec_source, sec_sid)
    select n.pri_source, n.pri_sid, n.sec_source, n.sec_sid
    from sec.new_recs n
    left join sec.temp_tr_sec_ids s
    on n.pri_source = s.pri_source and n.pri_sid = s.pri_sid
    and n.sec_source = s.sec_source and n.sec_sid = s.sec_sid
    where s.pri_source is null;"#;
        
    let n = execute_sql(&sql, pool).await?;
    
    let sql = r#"drop table if exists sec.temp_mult_sponsor_source_id;
        drop table if exists sec.temp_mult_sponsor_id_dets;
        truncate table sec.new_recs;"#;
        
    execute_sql(&sql, pool).await?;
    
    Ok(n)
}



pub async fn process_links(recs: Vec<LinkedRec>, pool: &Pool<Postgres>) -> Result<usize, AppError> {
   
    let vector_size = 250;
    let mut ors: OutputRecs = OutputRecs::new(vector_size);
    let mut n = 0;
    for rec in recs {
       
        // Construct each output record required.

        for i in 0..=rec.count-2 {
            for j in i+1..=rec.count-1 {
               
                let pri_rec: Vec<&str> = rec.array_agg[i as usize].split("::").collect();
                let sec_rec: Vec<&str> = rec.array_agg[j as usize].split("::").collect();
                
                let new_rec = OutputRec {
                    pri_source: pri_rec[0].parse().unwrap(),
                    pri_sid: pri_rec[1].to_string(),
                    sec_source: sec_rec[0].parse().unwrap(),
                    sec_sid: sec_rec[1].to_string(),
                };
                ors.add_rec(&new_rec);

                n += 1;

                if (n) % vector_size == 0 {    // Store records to DB and create new empty vectors.
                    ors.store_data(pool).await?;
                    ors = OutputRecs::new(vector_size);
                }
            }
        }
    }

    // At end unnest vectors to store in database
    ors.store_data(pool).await?;

    Ok(n)

}


pub async fn add_new_recs(pool: &Pool<Postgres>) -> Result<u64, AppError> {
   
    let sql = r#"insert into sec.temp_tr_sec_ids (pri_source, pri_sid, sec_source, sec_sid)
        select n.pri_source, n.pri_sid, n.sec_source, n.sec_sid
        from sec.new_recs n
        left join sec.temp_tr_sec_ids s
        on n.pri_source = s.pri_source and n.pri_sid = s.pri_sid
        and n.sec_source = s.sec_source and n.sec_sid = s.sec_sid
        where s.pri_source is null;"#;
        
    execute_sql(&sql, pool).await
}



pub async fn assign_prefs_and_rearrange(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"SET client_min_messages TO WARNING;
        drop table if exists sec.tr_sec_ids;
        create table sec.tr_sec_ids (
          pri_pref       int4
        , pri_source     int4
        , pri_sid        varchar 
        , sec_pref       int4
        , sec_source     int4
        , sec_sid        varchar
    );

    drop table if exists sec.tr_ids;
        create table sec.tr_ids (
          p_pref       int4
        , p_source     int4
        , p_sid        varchar 
        , n_pref       int4
        , n_source     int4
        , n_sid        varchar
    );"#;
    
    execute_sql(sql, pool).await?;

    let sql = r#"insert into sec.tr_sec_ids(pri_pref, pri_source, pri_sid, sec_pref, sec_source, sec_sid)
        select p1.preference_rating, s.pri_source, s.pri_sid, p2.preference_rating, s.sec_source, s.sec_sid
        from sec.temp_tr_sec_ids s
        inner join mon_src.parameters p1
        on pri_source = p1.id
        inner join mon_src.parameters p2
        on sec_source = p2.id;"#;

    execute_sql(sql, pool).await?;


    let sql = r#"insert into sec.tr_ids (p_pref, p_source, p_sid, n_pref, n_source, n_sid)
        select pri_pref, pri_source, pri_sid, sec_pref, sec_source, sec_sid
        from sec.tr_sec_ids
        where pri_pref > sec_pref;"#;
   
    execute_sql(sql, pool).await?;


    let sql = r#"insert into sec.tr_ids (p_pref, p_source, p_sid, n_pref, n_source, n_sid)
        select sec_pref, sec_source, sec_sid, pri_pref, pri_source, pri_sid
        from sec.tr_sec_ids
        where pri_pref < sec_pref;"#;
   
    execute_sql(sql, pool).await?;
        
    Ok(())
}



pub async fn retain_distinct(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = r#"drop table if exists sec.distinct_tr_ids;
               create table sec.distinct_tr_ids
               as select distinct * from sec.tr_ids;"#;
    
    execute_sql(sql, pool).await?;

    Ok(())
}
