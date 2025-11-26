use sqlx::{Pool, Postgres};
use crate::AppError;
use crate::setup::fetch_db_pars;

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


pub async fn set_up_data_grid (pool: &Pool<Postgres>, grid_name: &str) -> Result<(), AppError> {

let sql = format!(r#"drop table if exists der.grid_{};
    create table der.grid_{}
    (
          source_id   int4    not null
        , source_name varchar not null
        , not_given   int4 not null default(0)
        , pre_2000    int4 not null default(0)
        , y2000       int4 not null default(0)
        , y2001       int4 not null default(0)
        , y2002       int4 not null default(0)
        , y2003       int4 not null default(0)
        , y2004       int4 not null default(0)
        , y2005       int4 not null default(0)
        , y2006       int4 not null default(0)
        , y2007       int4 not null default(0)
        , y2008       int4 not null default(0)
        , y2009       int4 not null default(0)
        , y2010       int4 not null default(0)
        , y2011       int4 not null default(0)
        , y2012       int4 not null default(0)
        , y2013       int4 not null default(0)
        , y2014       int4 not null default(0)
        , y2015       int4 not null default(0)
        , y2016       int4 not null default(0)
        , y2017       int4 not null default(0)
        , y2018       int4 not null default(0)
        , y2019       int4 not null default(0)
        , y2020       int4 not null default(0)
        , y2021       int4 not null default(0)
        , y2022       int4 not null default(0)
        , y2023       int4 not null default(0)
        , y2024       int4 not null default(0)
        , y2025       int4 not null default(0)
        , y2026       int4 not null default(0)
        , y2027       int4 not null default(0)
        , y2028       int4 not null default(0)
        , y2029       int4 not null default(0)
        , y2030       int4 not null default(0)
        , line_total  int4 not null default(0)
    );
    create index grid_{}_src_id on der.grid_{}(source_id);"#, 
    grid_name, grid_name, grid_name, grid_name);

    execute_sql(&sql, pool).await?;

    Ok(())
}


pub async fn set_up_categorised_data_grid (pool: &Pool<Postgres>, grid_name: &str) -> Result<(), AppError> {

let sql = format!(r#"drop table if exists der.grid_{};
    create table der.grid_{}
    (
          source_id   int4    not null
        , source_name varchar not null
        , category_id int4    not null
        , category    varchar not null
        , not_given   int4 not null default(0)
        , pre_2000    int4 not null default(0)
        , y2000       int4 not null default(0)
        , y2001       int4 not null default(0)
        , y2002       int4 not null default(0)
        , y2003       int4 not null default(0)
        , y2004       int4 not null default(0)
        , y2005       int4 not null default(0)
        , y2006       int4 not null default(0)
        , y2007       int4 not null default(0)
        , y2008       int4 not null default(0)
        , y2009       int4 not null default(0)
        , y2010       int4 not null default(0)
        , y2011       int4 not null default(0)
        , y2012       int4 not null default(0)
        , y2013       int4 not null default(0)
        , y2014       int4 not null default(0)
        , y2015       int4 not null default(0)
        , y2016       int4 not null default(0)
        , y2017       int4 not null default(0)
        , y2018       int4 not null default(0)
        , y2019       int4 not null default(0)
        , y2020       int4 not null default(0)
        , y2021       int4 not null default(0)
        , y2022       int4 not null default(0)
        , y2023       int4 not null default(0)
        , y2024       int4 not null default(0)
        , y2025       int4 not null default(0)
        , y2026       int4 not null default(0)
        , y2027       int4 not null default(0)
        , y2028       int4 not null default(0)
        , y2029       int4 not null default(0)
        , y2030       int4 not null default(0)
        , line_total  int4 not null default(0)
    );
    create index grid_{}_src_id on der.grid_{}(source_id);"#, 
    grid_name, grid_name, grid_name, grid_name);

    execute_sql(&sql, pool).await?;

    Ok(())
}


pub async fn set_up_data_grids (pool: &Pool<Postgres>) -> Result<(), AppError> {

    set_up_data_grid(pool, "reg_numbers").await?;

    let sql = r#"Insert into der.grid_reg_numbers (source_id, source_name)
        select distinct source_id, source_name
        from met.tables 
        order by source_id;"#;

    execute_sql(sql, pool).await?;

    set_up_data_grid(pool, "enrol_numbers").await?;
   
    let sql = r#"Insert into der.grid_enrol_numbers (source_id, source_name)
        select distinct source_id, source_name
        from met.tables 
        order by source_id;"#;

    execute_sql(sql, pool).await?;

    set_up_categorised_data_grid(pool, "type_numbers").await?;

    let sql = r#"Insert into der.grid_type_numbers (source_id, source_name, category_id, category)
        select distinct t.source_id, t.source_name, st.id, st.name 
        from met.tables t
        cross join cxt_lups.study_types st
        order by t.source_id, st.id;"#;
     
    execute_sql(sql, pool).await?;

    set_up_categorised_data_grid(pool, "status_numbers").await?;

    let sql = r#"Insert into der.grid_status_numbers (source_id, source_name, category_id, category)
        select distinct t.source_id, t.source_name, ss.id, ss.name 
        from met.tables t
        cross join cxt_lups.study_statuses ss
        order by t.source_id, ss.id;"#;

    execute_sql(sql, pool).await?;

    // Need to translate countries into continents before aggregation

    set_up_data_grid(pool, "continent_numbers").await?;

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
        select {}, '{}', reg_year, study_type_id, count(id)
        from bas.{}
        group by reg_year, study_type_id "#, entry.source_id, entry.source_name, entry.table_name);

    execute_sql(&sql, pool).await
}


pub async fn store_status_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_statuses (source_id, source_name, reg_year, study_status_id, num)
        select {}, '{}', reg_year, study_status_id, count(id)
        from bas.{}
        group by reg_year, study_status_id "#, entry.source_id, entry.source_name, entry.table_name);

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


pub async fn set_up_ftw_schema(data_source: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {
    
    // First get DB parameters - only proceed if they are available
    let dbp = fetch_db_pars()?;
  
    // Derive database name and source schema - 
    // data_source is provided as source_db.source_schema, e.g. "cxt.lups".
    // Split into two constituent parts...
    
    let source_parts: Vec<&str> = data_source.split('.').collect();
    let source_db_name = source_parts[0];
    let source_schema = source_parts[1];

    // source db name used as the server name 
    // *** N.B. local host assumed here  *** would nade changing if not the case
    // dest schema will be source db and schema, separated by an underscore

    let dest_schema = data_source.replace(".", "_");

    let sql = format!(r#"CREATE EXTENSION IF NOT EXISTS postgres_fdw WITH SCHEMA met;
            CREATE SERVER IF NOT EXISTS {}
            FOREIGN DATA WRAPPER postgres_fdw
            OPTIONS (host '{}', dbname '{}', port '{}');"#,
            source_db_name, dbp.db_host, source_db_name, dbp.db_port);
        
    execute_sql(&sql, pool).await?;   

    let sql = format!(r#"CREATE USER MAPPING IF NOT EXISTS FOR CURRENT_USER
            SERVER {}
            OPTIONS (user '{}', password '{}');"#, 
            source_db_name, dbp.db_user, dbp.db_password);

    execute_sql(&sql, pool).await?;   
    
    let sql = format!(r#"DROP SCHEMA IF EXISTS {} cascade;
            CREATE SCHEMA {};
            IMPORT FOREIGN SCHEMA {}
            FROM SERVER {}
            INTO {};"#, dest_schema, dest_schema, source_schema, source_db_name, dest_schema);

    execute_sql(&sql, pool).await?;
  
    Ok(())
}

/* 
pub async fn drop_ftw_schema(dest_schema: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {

    // Derive database name a - 
    // dest_schema is provided as <source_db>_<source_schema>, e.g. "cxt_lups".
    // Split into two constituent parts...
    
    let source_parts: Vec<&str> = dest_schema.split('_').collect();
    let source_db_name = source_parts[0];
    let sql = format!(r#"DROP SCHEMA IF EXISTS {} cascade;
                    DROP USER MAPPING IF EXISTS FOR CURRENT_USER SERVER {} ;
                    DROP SERVER IF EXISTS {} ;"#, dest_schema, source_db_name, source_db_name);

    execute_sql(&sql, pool).await?;

    Ok(())
}

*/