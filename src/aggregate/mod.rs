mod data_access;
mod dedup;
mod structs;
mod ftw;

use sqlx::{Pool, Postgres};
use crate::AppError;
use log::info;




pub async fn identify_linked_studies(pool: &Pool<Postgres>) -> Result<(), AppError> {
 
    // First get the list of source tables

    let tables = data_access::fetch_table_list(pool).await?;
        
    // Then, collect the secondary ids into two temporary tables
    // One for the secondary ids that are trial registry ids, 
    // the second for 'other' ids, mostly from sponsors and funders.
        
    let mut tr_ids_total = 0;
    let mut oth_ids_total = 0;
    dedup::set_up_sec_id_tables(pool).await?;
    for entry in &tables {
        let (tr, oth) = dedup::process_sec_ids(entry, pool).await?;
        tr_ids_total += tr;
        oth_ids_total += oth;
    }
    info!("{} trial registry secondary ids extracted", tr_ids_total);
    info!("{} other (sponsor) secondary ids extracted", oth_ids_total);

    // Some of the 'registry ids' are in fact WHO UTN numbers.
    // Extract these into a separate table.
    
    let n = dedup::separate_who_utn_secids(pool).await?;
    info!("{} secondary ids that are WHO UTNs separated out", n);

    // See if common WHO UTN ids suggests links between studies. 
    // When this is the case add the new links to the link table.

    let links = dedup::setup_utn_processing(pool).await?;
    let n = dedup::process_links(links, pool).await?;
    info!("{} link records generated from shared UTN links", n);
    let r = dedup::add_new_recs(pool).await?;
    info!("{} new UTN based link records added to listing of links", r);
    dedup::complete_utn_processing(pool).await?;
    
    // Get possible other matches using sponsor name and 'sponsor id'(when longer than 4 characters
    // and discounting some obviously problematic ids). 'Sponsor id' refers to any non registry id.
    // Only get groupings where the number of studies per sponsor / sponsor id pair is > 1,
    // and where (the number of sources) = (the number of grouped sponsor ids) - i.e. each grouping should
    // consist of studies from different registries. Otherwise the common sponsor / sponsor id may in
    // fact relate to a funding or research programme id.
    // N.B. At this stage unambiguous sponsor identification is incomplete, so some links may be missing.

    let links = dedup::setup_sponsor_id_processing(pool).await?;
    let n = dedup::process_links(links, pool).await?;
    info!("{} new link records generated from shared sponsor id links", n);
    let r = dedup::add_new_recs(pool).await?;
    info!("{} new sponsor id based link records added to listing of links", r);
    dedup::complete_sponsor_id_processing(pool).await?;

    // Extract the secondary ids that have the same source TR into separate table.
    // These refer to studies that are related rather than equivalences, though
    // the nature of the relationshipo is unclear.

    let n = dedup::separate_same_registry_secids(pool).await?;
    info!("{} secondary ids from same registry separated out", n);

    // In many case the secondary id links will have been provided in
    // both directions, i.e. A<-B and B<-A. To identify these duplicates the
    // links must be presented in a consistent order. This is done by assigning
    // a preference to each source, and ensuring that the link is presented
    // most preferred source <- less preferred source.
    // Once this is done the duplicates can be removed by selecting distinct records.

    ftw::set_up_schema("mon.src", pool).await?;
    dedup::assign_prefs_and_rearrange(pool).await?;
    dedup::retain_distinct(pool).await?;
    
    // In some cases the relationships between studies is 1:n rather than 1:1, 
    // i.e. a study registered in one trial registry is equivalemt to 2 or more 
    // studies in another registry. In these situations the '1' study can be 
    // marked as a (partial) duplicate of the 'n' studies, and all the 1:n records should be 
    // removed from the list of links - i.e. they should not be processed further.



    // n:n links may also occur, though these are relatively rare. 
    // For further exploration.





    // The final list of secondary links is fine when just a pair of registrations 
    // are involved, but if a study is registered in three or more registries then there 
    // is no guarantee thast all links are present (e.g. A<-B and A<-C may be present 
    // but not B<-C). The links that are ***necessary*** to identify duplicate studies,
    // and 

    
    
    
    //Full links are necessary to support the cascade process described below.
    
    //ftw::drop_schema("mon_src", pool).await?;

    Ok(())
}




pub async fn aggregate_who_data(pool: &Pool<Postgres>) -> Result<(), AppError> {

    
    // Then set up tables to hold data,
    // then read list of soures / tables into a vector of structs

    data_access::set_up_data_tables(pool).await?;

    /* 
    let mut total = 0;
    for entry in &tables {
        total += data_access::store_reg_numbers(entry, pool).await?;
    }
    info!("{} numbers by reg year stored", total);

    total = 0;
    for entry in &tables {
        total += data_access::store_enrol_numbers(entry, pool).await?;
    }
    info!("{} numbers by enrol year stored", total);

    total = 0;
    for entry in &tables {
        total += data_access::store_type_numbers(entry, pool).await?;
    }
    info!("{} numbers by type and reg year stored", total);

    total = 0;
    for entry in &tables {
        total += data_access::store_status_numbers(entry, pool).await?;
    }
    info!("{} numbers by status and reg year stored", total);

    total = 0;
    for entry in &tables {
        data_access::unnest_country_lists(entry, pool).await?;
        total += data_access::store_country_numbers(&entry, pool).await?;
    }
    info!("{} numbers by country and reg year stored", total);
*/
    

    ftw::set_up_schema("cxt.lups", pool).await?;
    ftw::set_up_schema("cxt.locs", pool).await?;
    
    data_access::set_up_data_grids(pool).await?;

  //  data_access::insert_grid_reg_year_data(pool).await?;
  //  data_access::insert_grid_enrol_year_data(pool).await?;
  //  data_access::insert_grid_type_year_data(pool).await?;
  //  data_access::insert_grid_status_year_data(pool).await?;



    // summarise number of studies registered for each continent, for each source / year


    // summarise number of studies starting in each continent, 2004 onwards


    // summarise differences between start dates and registration dates


    // summarise number of studies with results, 2000 onwards

    ftw::drop_schema("ctx_lups", pool).await?;
    ftw::drop_schema("ctx_locs", pool).await?;

    Ok(())
}


