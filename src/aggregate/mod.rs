mod data_access;
mod initial;

use sqlx::{Pool, Postgres};
use crate::AppError;


pub async fn aggregate_who_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    // aggregation function has several phases

    // set up the table which has the ;ist of WHO data tables

    data_access::create_table_list(pool).await?;

    // check study types - dbase any not currently covered
    // summarise current distribution

    // check study statuses - dbase any not currently covered
    // summarise current distribution

    // get study countries and continents and update records - dbase any not currently covered
    // summarise current distribution

    // summarise number of registrations in each month for each registry, 2004 onwards

    // identify multi-registration

    // summarise multi-registration
    // identify level of WHO registration numbers (?)

    // flag duplicated studies so that they are not counted in the statistics below.

    // summarise number of studies registered for each continent, 2004 onwards
    // summarise number of studies starting in each continent, 2004 onwards

    // summarise number of studies where start date provided
    // summarise differences between start dates and regiustration dates

    // summarise number of studies starting in each continent, 2004 onwards

    // summarise number of studies with results, 2004 onwards

    // Summarise % of different study types, for each month


    Ok(())
}

