use sqlx::{Pool, Postgres};
use crate::AppError;

use super::structs::BasTable;


async fn execute_sql(sql: &str, pool: &Pool<Postgres>) -> Result<u64, AppError> {

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

    let sql = r#"Insert into der.grid_reg_numbers (source_id, source_name)
        values(800000, 'column_total');"#;
    execute_sql(sql, pool).await?;


    set_up_data_grid(pool, "enrol_numbers").await?;
   
    let sql = r#"Insert into der.grid_enrol_numbers (source_id, source_name)
        select distinct source_id, source_name
        from met.tables 
        order by source_id;"#;
    execute_sql(sql, pool).await?;

    let sql = r#"Insert into der.grid_enrol_numbers (source_id, source_name)
        values(800000, 'column_total');"#;
    execute_sql(sql, pool).await?;


    set_up_categorised_data_grid(pool, "type_numbers").await?;

    let sql = r#"Insert into der.grid_type_numbers (source_id, source_name, category_id, category)
        select distinct t.source_id, t.source_name, st.id, st.name 
        from met.tables t
        cross join cxt_lups.study_types st
        order by t.source_id, st.id;"#;
    execute_sql(sql, pool).await?;

    let sql = r#"Insert into der.grid_type_numbers (source_id, source_name, category_id, category)
        select 800000, 'column_total', st.id, st.name 
        from cxt_lups.study_types st
        order by st.id;"#;
    execute_sql(sql, pool).await?;


    set_up_categorised_data_grid(pool, "status_numbers").await?;

    let sql = r#"Insert into der.grid_status_numbers (source_id, source_name, category_id, category)
        select distinct t.source_id, t.source_name, ss.id, ss.name 
        from met.tables t
        cross join cxt_lups.study_statuses ss
        order by t.source_id, ss.id;"#;
    execute_sql(sql, pool).await?;

    let sql = r#"Insert into der.grid_status_numbers (source_id, source_name, category_id, category)
        select 800000, 'column_total', ss.id, ss.name 
        from cxt_lups.study_statuses ss
        order by ss.id;"#;
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



// Get the possible other matches using sponsor name and sponsor id
// Only get pairings - groups of more than 2 very likely to be funding grant ids
// (as, very probably, are some of the pairs)






/* 
pub async fn store_reg_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_reg_numbers (source_id, source_name, reg_year, num)
        select {}, '{}', reg_year, count(id)
        from dat.{}
        where is_a_duplicate is null
        group by reg_year"#, entry.source_id, entry.source_name, entry.table_name);

    execute_sql(&sql, pool).await
}


pub async fn store_enrol_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_enrol_numbers (source_id, source_name, enrol_year, num)
        select {}, '{}', enrol_year, count(id)
        from dat.{}
        where is_a_duplicate is null
        group by enrol_year"#, entry.source_id, entry.source_name, entry.table_name);

    execute_sql(&sql, pool).await
}


pub async fn store_type_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_types (source_id, source_name, reg_year, study_type_id, num)
        select {}, '{}', reg_year, study_type_id, count(id)
        from dat.{}
        where is_a_duplicate is null
        group by reg_year, study_type_id "#, entry.source_id, entry.source_name, entry.table_name);

    execute_sql(&sql, pool).await
}


pub async fn store_status_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_statuses (source_id, source_name, reg_year, study_status_id, num)
        select {}, '{}', reg_year, study_status_id, count(id)
        from dat.{}
        where is_a_duplicate is null
        group by reg_year, study_status_id "#, entry.source_id, entry.source_name, entry.table_name);

    execute_sql(&sql, pool).await
}


pub async fn unnest_country_lists(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {
    let sql = format!(r#"Truncate table der.temp_unnested_countries;
        Insert into der.temp_unnested_countries (source_id, source_name, reg_year, country)
        select {}, '{}', reg_year, unnest(country_list)
        from dat.{}
        where is_a_duplicate is null
        and country_list is not null"#, entry.source_id, entry.source_name, entry.table_name);

      execute_sql(&sql, pool).await
}


pub async fn store_country_numbers(entry: &BasTable, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let sql = format!(r#"Insert into der.study_countries (source_id, source_name, reg_year, country, num)
        select {}, '{}', reg_year, country, count(source_id)
        from der.temp_unnested_countries 
        group by reg_year, country "#, entry.source_id, entry.source_name);

    execute_sql(&sql, pool).await
}


pub async fn insert_grid_reg_year_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    // Do the 'not given' column

    let sql = r#"update der.grid_reg_numbers g
        set not_given = n.s
        from (
            select source_id, sum(num) as s
            from der.study_reg_numbers
            where reg_year = 0 or reg_year > 2030
            group by source_id) n
        where g.source_id = n.source_id"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_reg_numbers g
        set not_given = t.sum
        from (
            select sum(not_given) as sum
            from der.grid_reg_numbers) t
        where g.source_id = 800000"#;

    // Do the pre-2000 data, if any.

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_reg_numbers g
        set pre_2000 = n.s
        from (
            select source_id, sum(num) as s
            from der.study_reg_numbers
            where reg_year > 0 and reg_year < 2000 
            group by source_id) n
        where g.source_id = n.source_id"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_reg_numbers g
        set pre_2000 = t.sum
        from (
            select sum(pre_2000) as sum
            from der.grid_reg_numbers) t
        where g.source_id = 800000"#;

    execute_sql(&sql, pool).await?;

    // Do each year from 2000 to 2030.

    for yi in 2000..2031 {
        let y = yi.to_string();
        let sql = format!(r#"update der.grid_reg_numbers g
            set y{} = n.s
            from (
                select source_id, sum(num) as s
                from der.study_reg_numbers
                where reg_year = {}
                group by source_id) n
            where g.source_id = n.source_id"#, y, y);
  
        execute_sql(&sql, pool).await?;

        let sql = format!(r#"update der.grid_reg_numbers g
        set y{} = t.sum
        from (
            select sum(y{}) as sum
            from der.grid_reg_numbers) t
        where g.source_id = 800000"#, y, y);

        execute_sql(&sql, pool).await?;
    }

    // Do the totals at the end of each line

    let sql = r#"update der.grid_reg_numbers g
        set line_total = not_given + pre_2000 
            + y2000 + y2001 + y2002 + y2003 + y2004 + y2005 + y2006 + y2007 + y2008 + y2009
            + y2010 + y2011 + y2012 + y2013 + y2014 + y2015 + y2016 + y2017 + y2018 + y2019
            + y2020 + y2021 + y2022 + y2023 + y2024 + y2025 + y2026 + y2027 + y2028 + y2029
            + y2030"#;

    execute_sql(&sql, pool).await?;

    Ok(())
}


pub async fn insert_grid_enrol_year_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    // Do the 'not given' column

    let sql = r#"update der.grid_enrol_numbers g
        set not_given = n.s
        from (
            select source_id, sum(num) as s
            from der.study_enrol_numbers
            where enrol_year < 1970 or enrol_year > 2030
            group by source_id) n
        where g.source_id = n.source_id"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_enrol_numbers g
        set not_given = t.sum
        from (
            select sum(not_given) as sum
            from der.grid_enrol_numbers) t
        where g.source_id = 800000"#;

    // Do the pre-2000 data, if any.

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_enrol_numbers g
        set pre_2000 = n.s
        from (
            select source_id, sum(num) as s
            from der.study_enrol_numbers
            where enrol_year >= 1970 and enrol_year < 2000 
            group by source_id) n
        where g.source_id = n.source_id"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_enrol_numbers g
        set pre_2000 = t.sum
        from (
            select sum(pre_2000) as sum
            from der.grid_enrol_numbers) t
        where g.source_id = 800000"#;

    execute_sql(&sql, pool).await?;

    // Do each year from 2000 to 2030.

    for yi in 2000..2031 {
        let y = yi.to_string();
        let sql = format!(r#"update der.grid_enrol_numbers g
            set y{} = n.s
            from (
                select source_id, sum(num) as s
                from der.study_enrol_numbers
                where enrol_year = {}
                group by source_id) n
            where g.source_id = n.source_id"#, y, y);
  
        execute_sql(&sql, pool).await?;

        let sql = format!(r#"update der.grid_enrol_numbers g
        set y{} = t.sum
        from (
            select sum(y{}) as sum
            from der.grid_enrol_numbers) t
        where g.source_id = 800000"#, y, y);

        execute_sql(&sql, pool).await?;
    }

    // Do the totals at the end of each line

    let sql = r#"update der.grid_enrol_numbers g
        set line_total = not_given + pre_2000 
            + y2000 + y2001 + y2002 + y2003 + y2004 + y2005 + y2006 + y2007 + y2008 + y2009
            + y2010 + y2011 + y2012 + y2013 + y2014 + y2015 + y2016 + y2017 + y2018 + y2019
            + y2020 + y2021 + y2022 + y2023 + y2024 + y2025 + y2026 + y2027 + y2028 + y2029
            + y2030"#;

    execute_sql(&sql, pool).await?;

    Ok(())
}


pub async fn insert_grid_type_year_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    // Do the 'not given' column

    let sql = r#"update der.grid_type_numbers g
        set not_given = n.s
        from (
            select source_id, study_type_id as category_id, sum(num) as s
            from der.study_types
            where reg_year = 0 or reg_year > 2030
            group by source_id, study_type_id) n
        where g.source_id = n.source_id
        and g.category_id = n.category_id"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_type_numbers g
        set not_given = t.sum
        from (
            select category_id, sum(not_given) as sum
            from der.grid_type_numbers
            group by category_id) t
        where g.source_id = 800000
        and g.category_id = t.category_id"#;

    // Do the pre-2000 data, if any.

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_type_numbers g
        set pre_2000 = n.s
        from (
            select source_id, study_type_id as category_id, sum(num) as s
            from der.study_types
            where reg_year > 0 and reg_year < 2000 
            group by source_id, study_type_id) n
        where g.source_id = n.source_id
        and g.category_id = n.category_id"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_type_numbers g
        set pre_2000 = t.sum
        from (
            select category_id, sum(pre_2000) as sum
            from der.grid_type_numbers
            group by category_id) t
        where g.source_id = 800000
        and g.category_id = t.category_id"#;

    execute_sql(&sql, pool).await?;

    // Do each year from 2000 to 2030.

    for yi in 2000..2031 {
        let y = yi.to_string();
        let sql = format!(r#"update der.grid_type_numbers g
            set y{} = n.s
            from (
                select source_id, study_type_id as category_id, sum(num) as s
                from der.study_types
                where reg_year = {}
                group by source_id, study_type_id) n
            where g.source_id = n.source_id
            and g.category_id = n.category_id"#, y, y);
  
        execute_sql(&sql, pool).await?;

        let sql = format!(r#"update der.grid_type_numbers g
        set y{} = t.sum
        from (
            select category_id, sum(y{}) as sum
            from der.grid_type_numbers
            group by category_id) t
        where g.source_id = 800000
        and g.category_id = t.category_id"#, y, y);

        execute_sql(&sql, pool).await?;
    }

    // Do the totals at the end of each line

    let sql = r#"update der.grid_type_numbers g
        set line_total = not_given + pre_2000 
            + y2000 + y2001 + y2002 + y2003 + y2004 + y2005 + y2006 + y2007 + y2008 + y2009
            + y2010 + y2011 + y2012 + y2013 + y2014 + y2015 + y2016 + y2017 + y2018 + y2019
            + y2020 + y2021 + y2022 + y2023 + y2024 + y2025 + y2026 + y2027 + y2028 + y2029
            + y2030"#;

    execute_sql(&sql, pool).await?;

    // Remove registry categories with 0 totals

    let sql = r#"delete from der.grid_type_numbers g
        where line_total = 0
        and g.source_id <> 800000"#;

    execute_sql(&sql, pool).await?;


    Ok(())
}


pub async fn insert_grid_status_year_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    // Do the 'not given' column

    let sql = r#"update der.grid_status_numbers g
        set not_given = n.s
        from (
            select source_id, study_status_id as category_id, sum(num) as s
            from der.study_statuses
            where reg_year = 0 or reg_year > 2030
            group by source_id, study_status_id) n
        where g.source_id = n.source_id
        and g.category_id = n.category_id"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_status_numbers g
        set not_given = t.sum
        from (
            select category_id, sum(not_given) as sum
            from der.grid_status_numbers
            group by category_id) t
        where g.source_id = 800000
        and g.category_id = t.category_id"#;

    // Do the pre-2000 data, if any.

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_status_numbers g
        set pre_2000 = n.s
        from (
            select source_id, study_status_id as category_id, sum(num) as s
            from der.study_statuses
            where reg_year > 0 and reg_year < 2000 
            group by source_id, study_status_id) n
        where g.source_id = n.source_id
        and g.category_id = n.category_id"#;

    execute_sql(&sql, pool).await?;

    let sql = r#"update der.grid_status_numbers g
        set pre_2000 = t.sum
        from (
            select category_id, sum(pre_2000) as sum
            from der.grid_status_numbers
            group by category_id) t
        where g.source_id = 800000
        and g.category_id = t.category_id"#;

    execute_sql(&sql, pool).await?;

    // Do each year from 2000 to 2030.

    for yi in 2000..2031 {
        let y = yi.to_string();
        let sql = format!(r#"update der.grid_status_numbers g
            set y{} = n.s
            from (
                select source_id, study_status_id as category_id, sum(num) as s
                from der.study_statuses
                where reg_year = {}
                group by source_id, study_status_id) n
            where g.source_id = n.source_id
            and g.category_id = n.category_id"#, y, y);
  
        execute_sql(&sql, pool).await?;

        let sql = format!(r#"update der.grid_status_numbers g
        set y{} = t.sum
        from (
            select category_id, sum(y{}) as sum
            from der.grid_status_numbers
            group by category_id) t
        where g.source_id = 800000
        and g.category_id = t.category_id"#, y, y);

        execute_sql(&sql, pool).await?;
    }

    // Do the totals at the end of each line

    let sql = r#"update der.grid_status_numbers g
        set line_total = not_given + pre_2000 
            + y2000 + y2001 + y2002 + y2003 + y2004 + y2005 + y2006 + y2007 + y2008 + y2009
            + y2010 + y2011 + y2012 + y2013 + y2014 + y2015 + y2016 + y2017 + y2018 + y2019
            + y2020 + y2021 + y2022 + y2023 + y2024 + y2025 + y2026 + y2027 + y2028 + y2029
            + y2030"#;

    execute_sql(&sql, pool).await?;

    // Remove registry categories with 0 totals

    let sql = r#"delete from der.grid_status_numbers g
        where line_total = 0
        and g.source_id <> 800000"#;

    execute_sql(&sql, pool).await?;


    Ok(())
}

*/
