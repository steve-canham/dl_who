use sqlx::{Pool, Postgres};
use crate::AppError;

use super::BasTable;


pub async fn execute_sql(sql: &str, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(res.rows_affected())
}


pub async fn set_up_data_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {
  
    let sql = r#"drop table if exists der.study_reg_numbers;
    create table der.study_reg_numbers (
          source_id  int4
        , source_name varchar
        , reg_year int4
        , num int4
    );"#;

    execute_sql(sql, pool).await?;

    let sql = r#"drop table if exists der.study_enrol_numbers;
    create table der.study_enrol_numbers (
          source_id  int4
        , source_name varchar
        , enrol_year int4
        , num int4
    );"#;

    execute_sql(sql, pool).await?;

    let sql = r#"drop table if exists der.study_types;
    create table der.study_types (
          source_id  int4
        , source_name varchar
        , reg_year int4
        , study_type_id int4
        , num int4
    );"#;

    execute_sql(sql, pool).await?;

    let sql = r#"drop table if exists der.study_statuses;
    create table der.study_statuses (
          source_id  int4
        , source_name varchar
        , reg_year int4
        , study_status_id int4
        , num int4
    );"#;

    execute_sql(sql, pool).await?;

    let sql = r#"drop table if exists der.temp_unnested_countries;
    create table der.temp_unnested_countries (
          source_id  int4
        , source_name varchar
        , reg_year int4
        , country varchar
    );"#;

    execute_sql(sql, pool).await?;

    let sql = r#"drop table if exists der.study_countries;
    create table der.study_countries (
          source_id  int4
        , source_name varchar
        , reg_year int4
        , country varchar
        , num int4
    );"#;

    execute_sql(sql, pool).await?;

    Ok(())
}


pub async fn fetch_table_list(pool: &Pool<Postgres>) -> Result<Vec<BasTable>, AppError> {
  
  let sql = r#"select table_name, source_id, source_name
        from met.tables
        order by source_id;"#;

  sqlx::query_as(&sql).fetch_all(pool).await
             .map_err(|e| AppError::SqlxError(e, sql.to_string()))

}


pub async fn store_reg_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_reg_numbers (source_id, source_name, reg_year, num)
        select {}, '{}', reg_year, count(id)
        from bas.{}
        group by reg_year"#, entry.source_id, entry.source_name, entry.table_name);

    execute_sql(&sql, pool).await
}


pub async fn store_enrol_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_enrol_numbers (source_id, source_name, enrol_year, num)
        select {}, '{}', enrol_year, count(id)
        from bas.{}
        group by enrol_year"#, entry.source_id, entry.source_name, entry.table_name);

    execute_sql(&sql, pool).await
}


pub async fn store_type_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_types (source_id, source_name, reg_year, study_type_id, num)
        select {}, '{}', reg_year, study_type, count(id)
        from bas.{}
        group by reg_year, study_type "#, entry.source_id, entry.source_name, entry.table_name);

    execute_sql(&sql, pool).await
}


pub async fn store_status_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_statuses (source_id, source_name, reg_year, study_status_id, num)
        select {}, '{}', reg_year, study_status, count(id)
        from bas.{}
        group by reg_year, study_status "#, entry.source_id, entry.source_name, entry.table_name);

    execute_sql(&sql, pool).await
}


pub async fn unnest_country_lists(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {
    let sql = format!(r#"Truncate table der.temp_unnested_countries;
        Insert into der.temp_unnested_countries (source_id, source_name, reg_year, country)
        select {}, '{}', reg_year, unnest(country_list)
        from bas.{}
        where country_list is not null"#, entry.source_id, entry.source_name, entry.table_name);

      execute_sql(&sql, pool).await
}


pub async fn store_country_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_countries (source_id, source_name, reg_year, country, num)
        select {}, '{}', reg_year, country, count(source_id)
        from der.temp_unnested_countries 
        group by reg_year, country "#, entry.source_id, entry.source_name);

    execute_sql(&sql, pool).await
}

