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



/*
To DO


SELECT 'insert into der.study_types (source, study_type, num) select '''||source_name||''', study_type, count(id) as num from sd.'|| table_name||' group by study_type;'
		FROM der.db_tables 
order by table_name;


insert into der.study_types (source, study_type, num) select 'anzctr', study_type, count(id) as num from bas.anzctr group by study_type;  
insert into der.study_types (source, study_type, num) select 'chictr', study_type, count(id) as num from bas.chictr_ge_2020 group by study_type;
insert into der.study_types (source, study_type, num) select 'chictr', study_type, count(id) as num from bas.chictr_lt_2020 group by study_type;
insert into der.study_types (source, study_type, num) select 'cris', study_type, count(id) as num from bas.cris group by study_type;
insert into der.study_types (source, study_type, num) select 'ctg', study_type, count(id) as num from bas.ctg_2010_14 group by study_type;
insert into der.study_types (source, study_type, num) select 'ctg', study_type, count(id) as num from bas.ctg_2015_19 group by study_type;
insert into der.study_types (source, study_type, num) select 'ctg', study_type, count(id) as num from bas.ctg_2020_24 group by study_type;
insert into der.study_types (source, study_type, num) select 'ctg', study_type, count(id) as num from bas.ctg_2025_29 group by study_type;
insert into der.study_types (source, study_type, num) select 'ctg', study_type, count(id) as num from bas.ctg_lt_2010 group by study_type;
insert into der.study_types (source, study_type, num) select 'ctis', study_type, count(id) as num from bas.ctis group by study_type;
insert into der.study_types (source, study_type, num) select 'ctri', study_type, count(id) as num from bas.ctri_ge_2020 group by study_type;
insert into der.study_types (source, study_type, num) select 'ctri', study_type, count(id) as num from bas.ctri_lt_2020 group by study_type;
insert into der.study_types (source, study_type, num) select 'drks', study_type, count(id) as num from bas.drks group by study_type;
insert into der.study_types (source, study_type, num) select 'euctr', study_type, count(id) as num from bas.euctr group by study_type;
insert into der.study_types (source, study_type, num) select 'irct', study_type, count(id) as num from bas.irct group by study_type;
insert into der.study_types (source, study_type, num) select 'isrctn', study_type, count(id) as num from bas.isrctn group by study_type;
insert into der.study_types (source, study_type, num) select 'itmctr', study_type, count(id) as num from bas.itmctr group by study_type;
insert into der.study_types (source, study_type, num) select 'jprn', study_type, count(id) as num from bas.jprn_ge_2020 group by study_type;
insert into der.study_types (source, study_type, num) select 'jprn', study_type, count(id) as num from bas.jprn_lt_2020 group by study_type;
insert into der.study_types (source, study_type, num) select 'lebctr', study_type, count(id) as num from bas.lebctr group by study_type;
insert into der.study_types (source, study_type, num) select 'nntr', study_type, count(id) as num from bas.nntr group by study_type;
insert into der.study_types (source, study_type, num) select 'pactr', study_type, count(id) as num from bas.pactr group by study_type;
insert into der.study_types (source, study_type, num) select 'rebec', study_type, count(id) as num from bas.rebec group by study_type;
insert into der.study_types (source, study_type, num) select 'rpcec', study_type, count(id) as num from bas.rpcec group by study_type;
insert into der.study_types (source, study_type, num) select 'rpuec', study_type, count(id) as num from bas.rpuec group by study_type;
insert into der.study_types (source, study_type, num) select 'slctr', study_type, count(id) as num from bas.slctr group by study_type;
insert into der.study_types (source, study_type, num) select 'thctr', study_type, count(id) as num from bas.thctr group by study_type;


SELECT 'insert into der.study_statuses (source, study_status, num) select '''||source_name||''', study_status, count(id) as num from bas.'|| table_name||' group by study_status;'
		FROM der.db_tables 
order by table_name;

insert into der.study_statuses (source, study_status, num) select 'anzctr', study_status, count(id) as num from bas.anzctr group by study_status;
insert into der.study_statuses (source, study_status, num) select 'chictr', study_status, count(id) as num from bas.chictr_ge_2020 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'chictr', study_status, count(id) as num from bas.chictr_lt_2020 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'cris', study_status, count(id) as num from bas.cris group by study_status;
insert into der.study_statuses (source, study_status, num) select 'ctg', study_status, count(id) as num from bas.ctg_2010_14 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'ctg', study_status, count(id) as num from bas.ctg_2015_19 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'ctg', study_status, count(id) as num from bas.ctg_2020_24 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'ctg', study_status, count(id) as num from bas.ctg_2025_29 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'ctg', study_status, count(id) as num from bas.ctg_lt_2010 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'ctis', study_status, count(id) as num from bas.ctis group by study_status;
insert into der.study_statuses (source, study_status, num) select 'ctri', study_status, count(id) as num from bas.ctri_ge_2020 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'ctri', study_status, count(id) as num from bas.ctri_lt_2020 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'drks', study_status, count(id) as num from bas.drks group by study_status;
insert into der.study_statuses (source, study_status, num) select 'euctr', study_status, count(id) as num from bas.euctr group by study_status;
insert into der.study_statuses (source, study_status, num) select 'irct', study_status, count(id) as num from bas.irct group by study_status;
insert into der.study_statuses (source, study_status, num) select 'isrctn', study_status, count(id) as num from bas.isrctn group by study_status;
insert into der.study_statuses (source, study_status, num) select 'itmctr', study_status, count(id) as num from bas.itmctr group by study_status;
insert into der.study_statuses (source, study_status, num) select 'jprn', study_status, count(id) as num from bas.jprn_ge_2020 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'jprn', study_status, count(id) as num from bas.jprn_lt_2020 group by study_status;
insert into der.study_statuses (source, study_status, num) select 'lebctr', study_status, count(id) as num from bas.lebctr group by study_status;
insert into der.study_statuses (source, study_status, num) select 'nntr', study_status, count(id) as num from bas.nntr group by study_status;
insert into der.study_statuses (source, study_status, num) select 'pactr', study_status, count(id) as num from bas.pactr group by study_status;
insert into der.study_statuses (source, study_status, num) select 'rebec', study_status, count(id) as num from bas.rebec group by study_status;
insert into der.study_statuses (source, study_status, num) select 'rpcec', study_status, count(id) as num from bas.rpcec group by study_status;
insert into der.study_statuses (source, study_status, num) select 'rpuec', study_status, count(id) as num from bas.rpuec group by study_status;
insert into der.study_statuses (source, study_status, num) select 'slctr', study_status, count(id) as num from bas.slctr group by study_status;
insert into der.study_statuses (source, study_status, num) select 'thctr', study_status, count(id) as num from bas.thctr group by study_status;


select country, count(country) from (select unnest(country_list) as country from bas.anzctr) c group by country;

SELECT 'insert into der.countries (source, country, num) select '''||source_name||''', country, count(country) as num from (select unnest(country_list) as country from bas.'||table_name||') c group by country;'
		FROM der.db_tables 
order by table_name;

insert into der.countries (source, country, num) select 'anzctr', country, count(country) as num from (select unnest(country_list) as country from bas.anzctr) c group by country;
insert into der.countries (source, country, num) select 'chictr', country, count(country) as num from (select unnest(country_list) as country from bas.chictr_ge_2020) c group by country;
insert into der.countries (source, country, num) select 'chictr', country, count(country) as num from (select unnest(country_list) as country from bas.chictr_lt_2020) c group by country;
insert into der.countries (source, country, num) select 'cris', country, count(country) as num from (select unnest(country_list) as country from bas.cris) c group by country;
insert into der.countries (source, country, num) select 'ctg', country, count(country) as num from (select unnest(country_list) as country from bas.ctg_2010_14) c group by country;
insert into der.countries (source, country, num) select 'ctg', country, count(country) as num from (select unnest(country_list) as country from bas.ctg_2015_19) c group by country;
insert into der.countries (source, country, num) select 'ctg', country, count(country) as num from (select unnest(country_list) as country from bas.ctg_2020_24) c group by country;
insert into der.countries (source, country, num) select 'ctg', country, count(country) as num from (select unnest(country_list) as country from bas.ctg_2025_29) c group by country;
insert into der.countries (source, country, num) select 'ctg', country, count(country) as num from (select unnest(country_list) as country from bas.ctg_lt_2010) c group by country;
insert into der.countries (source, country, num) select 'ctis', country, count(country) as num from (select unnest(country_list) as country from bas.ctis) c group by country;
insert into der.countries (source, country, num) select 'ctri', country, count(country) as num from (select unnest(country_list) as country from bas.ctri_ge_2020) c group by country;
insert into der.countries (source, country, num) select 'ctri', country, count(country) as num from (select unnest(country_list) as country from bas.ctri_lt_2020) c group by country;
insert into der.countries (source, country, num) select 'drks', country, count(country) as num from (select unnest(country_list) as country from bas.drks) c group by country;
insert into der.countries (source, country, num) select 'euctr', country, count(country) as num from (select unnest(country_list) as country from bas.euctr) c group by country;
insert into der.countries (source, country, num) select 'irct', country, count(country) as num from (select unnest(country_list) as country from bas.irct) c group by country;
insert into der.countries (source, country, num) select 'isrctn', country, count(country) as num from (select unnest(country_list) as country from bas.isrctn) c group by country;
insert into der.countries (source, country, num) select 'itmctr', country, count(country) as num from (select unnest(country_list) as country from bas.itmctr) c group by country;
insert into der.countries (source, country, num) select 'jprn', country, count(country) as num from (select unnest(country_list) as country from bas.jprn_ge_2020) c group by country;
insert into der.countries (source, country, num) select 'jprn', country, count(country) as num from (select unnest(country_list) as country from bas.jprn_lt_2020) c group by country;
insert into der.countries (source, country, num) select 'lebctr', country, count(country) as num from (select unnest(country_list) as country from bas.lebctr) c group by country;
insert into der.countries (source, country, num) select 'nntr', country, count(country) as num from (select unnest(country_list) as country from bas.nntr) c group by country;
insert into der.countries (source, country, num) select 'pactr', country, count(country) as num from (select unnest(country_list) as country from bas.pactr) c group by country;
insert into der.countries (source, country, num) select 'rebec', country, count(country) as num from (select unnest(country_list) as country from bas.rebec) c group by country;
insert into der.countries (source, country, num) select 'rpcec', country, count(country) as num from (select unnest(country_list) as country from bas.rpcec) c group by country;
insert into der.countries (source, country, num) select 'rpuec', country, count(country) as num from (select unnest(country_list) as country from bas.rpuec) c group by country;
insert into der.countries (source, country, num) select 'slctr', country, count(country) as num from (select unnest(country_list) as country from bas.slctr) c group by country;
insert into der.countries (source, country, num) select 'thctr', country, count(country) as num from (select unnest(country_list) as country from bas.thctr) c group by country;


drop table if exists der.dist_countries;
create table der.dist_countries as select distinct country from der.countries c order by country

drop table if exists der.dist_types;
create table der.dist_types as 
select study_type, sum(num)  as num 
from der.study_types
group by study_type
order by  sum(num) desc;

drop table if exists der.dist_statuses;
create table der.dist_statuses as 
select study_status, sum(num)  as num 
from der.study_statuses
group by study_status
order by  sum(num) desc;

*/

