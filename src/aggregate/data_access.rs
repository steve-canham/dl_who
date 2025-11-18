use sqlx::{Pool, Postgres};
use crate::AppError;

pub async fn create_table_list(pool: &Pool<Postgres>) -> Result<(), AppError> {
  
    let sql = r#"drop table if exists met.tables;
            create table met.tables
            (
                table_name varchar
                , source_id int4
                , source_name varchar
            );"#;

    sqlx::raw_sql(&sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    let sql = r#"insert into met.tables (table_name, source_id, source_name)
            values 
            ('anzctr', 100116, 'anzctr'),
            ('chictr_ge_2020', 100118, 'chictr'),
            ('chictr_lt_2020', 100118, 'chictr'),
            ('cris', 100119, 'cris'),
            ('ctg_2010_14', 100120, 'ctg'),
            ('ctg_2015_19', 100120, 'ctg'),
            ('ctg_2020_24', 100120, 'ctg'),
            ('ctg_2025_29', 100120, 'ctg'),
            ('ctg_lt_2010', 100120, 'ctg'),
            ('ctis', 110428, 'ctis'),
            ('ctri_ge_2020', 100121, 'ctri'),
            ('ctri_lt_2020', 100121, 'ctri'),
            ('drks', 100124, 'drks'),
            ('euctr', 100123, 'euctr'),
            ('irct', 100125, 'irct'),
            ('isrctn',100126, 'isrctn'),
            ('itmctr', 109108, 'itmctr'),
            ('jprn_ge_2020', 100127, 'jprn'),
            ('jprn_lt_2020', 100127, 'jprn'),
            ('lebctr', 101989, 'lebctr'),
            ('nntr', 100132, 'nntr'),
            ('pactr', 100128, 'pactr'),
            ('rebec', 100117, 'rebec'),
            ('rpcec', 100122, 'rpcec'),
            ('rpuec', 100129, 'rpuec'),
            ('slctr', 100130, 'slctr'),
            ('thctr', 100131, 'thctr');"#;

    sqlx::raw_sql(&sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}


pub async fn set_up_reg_summ_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {
  
    let sql = r#"drop table if exists der.study_types;
    create table der.study_types (
        source varchar
        , study_type varchar
        , num int4
    );

    drop table if exists der.study_statuses;
    create table der.study_statuses (
        source varchar
        , study_status varchar
        , num int4
    );


    drop table if exists der.countries;
    create table der.countries (
        source varchar
        , country varchar
        , num int4
    );"#;

    sqlx::raw_sql(&sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
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

