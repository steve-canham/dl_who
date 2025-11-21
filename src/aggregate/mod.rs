mod data_access;
mod initial;

use sqlx::{Pool, Postgres};
use crate::AppError;
use log::info;

#[derive(sqlx::FromRow)]
pub struct BasTable {
    pub table_name: String, 
    pub source_id: i32, 
    pub source_name: String,
}


pub async fn aggregate_who_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    // Aggregation function has several phases
    // First set up tables to hold data,
    // then read list of soures / tables into a vector of structs

    data_access::set_up_data_tables(pool).await?;
    let tables: Vec<BasTable> = data_access::fetch_table_list(pool).await?;

    let mut total = 0;
    for entry in &tables {
        total += data_access::store_reg_numbers(&entry, pool).await?;
    }
    info!("{} numbers by reg year stored", total);

    total = 0;
    for entry in &tables {
        total += data_access::store_enrol_numbers(&entry, pool).await?;
    }
    info!("{} numbers by enrol year stored", total);

    total = 0;
    for entry in &tables {
        total += data_access::store_type_numbers(&entry, pool).await?;
    }
    info!("{} numbers by type and reg year stored", total);

    total = 0;
    for entry in &tables {
        total += data_access::store_status_numbers(&entry, pool).await?;
    }
    info!("{} numbers by status and reg year stored", total);

    total = 0;
    for entry in &tables {
        data_access::unnest_country_lists(&entry, pool).await?;
        total += data_access::store_country_numbers(&entry, pool).await?;
    }
    info!("{} numbers by country and reg year stored", total);




    // check study types - dbase any not currently covered
    // summarise current distribution

    
    // check study statuses - dbase any not currently covered
    // summarise current distribution


    // get study countries and continents and update records - dbase any not currently covered
    // summarise current distribution


    // summarise number of registrations in each month for each registry, 2000 onwards


    // identify multi-registration


    // summarise multi-registration
    // identify level of WHO registration numbers (?)


    // flag duplicated studies so that they are not counted in the statistics below.


    // summarise number of studies registered for each continent, 2004 onwards


    // summarise number of studies starting in each continent, 2004 onwards


    // summarise number of studies where start date provided


    // summarise differences between start dates and registration dates


    // summarise number of studies starting in each continent, 2000 onwards


    // summarise number of studies with results, 2000 onwards


    // Summarise % of different study types, for each month


    Ok(())
}

