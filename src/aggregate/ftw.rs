use sqlx::{Pool, Postgres};
use crate::AppError;
use crate::setup::fetch_db_pars;


async fn execute_sql(sql: &str, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let res = sqlx::raw_sql(sql).execute(pool)
            .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(res.rows_affected())
}

pub async fn set_up_schema(data_source: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {
    
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

    let sql = format!(r#"SET client_min_messages TO WARNING;
            CREATE EXTENSION IF NOT EXISTS postgres_fdw WITH SCHEMA met;
            CREATE SERVER IF NOT EXISTS {}
            FOREIGN DATA WRAPPER postgres_fdw
            OPTIONS (host '{}', dbname '{}', port '{}');"#,
            source_db_name, dbp.db_host, source_db_name, dbp.db_port);
        
    execute_sql(&sql, pool).await?;   

    let sql = format!(r#"SET client_min_messages TO WARNING;
            CREATE USER MAPPING IF NOT EXISTS FOR CURRENT_USER
            SERVER {}
            OPTIONS (user '{}', password '{}');"#, 
            source_db_name, dbp.db_user, dbp.db_password);

    execute_sql(&sql, pool).await?;   
    
    let sql = format!(r#"DROP SCHEMA IF EXISTS {} cascade;
            CREATE SCHEMA {};
            IMPORT FOREIGN SCHEMA {}
            FROM SERVER {}
            INTO {};
            SET client_min_messages TO NOTICE;"#, dest_schema, dest_schema, source_schema, source_db_name, dest_schema);

    execute_sql(&sql, pool).await?;
  
    Ok(())
}


pub async fn drop_schema(dest_schema: &str, pool: &Pool<Postgres>) -> Result<(), AppError> {

    // Derive database name a - 
    // dest_schema is provided as <source_db>_<source_schema>, e.g. "cxt_lups".
    // Split into two constituent parts...
    
    let source_parts: Vec<&str> = dest_schema.split('_').collect();
    let source_db_name = source_parts[0];
    let sql = format!(r#"SET client_min_messages TO WARNING;
                    DROP SCHEMA IF EXISTS {} cascade;
                    DROP USER MAPPING IF EXISTS FOR CURRENT_USER SERVER {} ;
                    DROP SERVER IF EXISTS {};
                    SET client_min_messages TO NOTICE;"#, dest_schema, source_db_name, source_db_name);

    execute_sql(&sql, pool).await?;

    Ok(())
}

