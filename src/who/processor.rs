//use crate::AppError;
//use std::path::PathBuf;
//use crate::DownloadResult;

use regex::Regex;
use std::sync::LazyLock;
use log::error;
use crate::who::file_models::MeddraCondition;
use chrono::NaiveDate;

use super::who_helper::{get_db_name, get_source_id, get_type, get_status, 
    get_conditions, split_and_dedup_countries, 
    add_int_study_features, add_obs_study_features, add_eu_design_features,
    add_masking_features, add_phase_features, add_eu_phase_features, split_and_add_ids};
use super::gen_helper::{StringExtensions, DateExtensions};
use super::file_models::{WHOLine, WHORecord, WhoStudyFeature, SecondaryId, WHOSummary};


pub fn process_line(w: WHOLine, i: i32) -> (i32, Option<WHORecord>)  {

    let sid = w.trial_id.replace("/", "-").replace("\\", "-").replace(".", "-");
    let mut sd_sid = sid.trim().to_string();
    
    if sd_sid == "" || sd_sid == "null" || sd_sid == "NULL" {        // Seems to happen, or has happened in the past, with one Dutch trial.
        error!("Well that's weird - no study id on line {}!", i);
        return (0, None);
    }

    let source_id = get_source_id(&sd_sid);
    if source_id == 100120 || source_id == 100126  // no need to process these - details input directly from registry (for CGT, ISRCTN).
    {
        return (source_id, None);
    }
            
    if source_id == 0
    {
        error!("Well that's weird - can't match the study id's {} source on line {}!", sd_sid, i);
        return (0, None);
       
    }

    if source_id == 100123 {
        sd_sid = sd_sid[0..19].to_string(); // lose country specific suffix
    }
    
    let study_type = w.study_type.tidy();
    let stype = get_type(&study_type);

    let study_status = w.recruitment_status.tidy();
    let status = get_status(&study_status);

    let design_list = w.study_design.tidy();
    let design_orig = design_list.clone();
    let phase_statement = w.phase.tidy();
    let phase_orig = phase_statement.clone();

    let condition_list = w.conditions.replace_unicodes();
    let conditions_option: Option<Vec<String>>;
    let meddraconds_option: Option<Vec<MeddraCondition>>;
    if condition_list.is_some()
    {
        // Need a specific routine to deal with EUCTR and the MedDRA listings
        let conditions: Vec<String>;
        let meddraconds: Vec<MeddraCondition>;

        (conditions, meddraconds) = get_conditions(&condition_list.unwrap(), source_id);
        conditions_option = match conditions.len() {
            0 => None,
            _ => Some(conditions)
        };
        meddraconds_option = match meddraconds.len() {
            0 => None,
            _ => Some(meddraconds)
        };
    }
    else {
        conditions_option = None;
        meddraconds_option = None;
    }
   

    let mut secondary_ids = Vec::<SecondaryId>::new();
    let sec_ids = w.sec_ids.tidy();

    if sec_ids.is_some()
    {
        let mut initial_ids = split_and_add_ids(&secondary_ids, &sd_sid, &sec_ids.unwrap(), "secondary ids");
        secondary_ids.append(&mut initial_ids);
    }
        
    let bridging_flag = w.bridging_flag.tidy();
    if bridging_flag.is_some() {
        let mut br_flag = bridging_flag.unwrap();
        if source_id == 100123 {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{4}-[0-9]{6}-[0-9]{2}-").unwrap());
            if RE.is_match(&br_flag) {
                br_flag = br_flag[0..19].to_string() // lose country specific suffix
            }
        }
        if br_flag != sd_sid
        {
            let mut bridge_ids = split_and_add_ids(&secondary_ids, &sd_sid, &br_flag, "bridging flag");
            secondary_ids.append(&mut bridge_ids);
        }
    }

    let childs = w.childs.tidy();
    if childs.is_some()
    {
        let mut child_ids = split_and_add_ids(&secondary_ids, &sd_sid, &childs.unwrap(), "bridged child recs");
        secondary_ids.append(&mut child_ids);
    }

    let secids = match secondary_ids.len() {
        0 => None,
        _ => Some(secondary_ids)
    };
   

    let mut features = Vec::<WhoStudyFeature>::new();

    if design_list.is_some()  {
        let des_list = &design_list.unwrap().to_lowercase();

        // Needs separate routines for eu-ctr

        if study_type == Some("Observational".to_string())
        {
            let mut fs = add_obs_study_features(des_list);
            features.append(&mut fs);
        }
        else
        {      
            if source_id == 100123 {
                let mut fs = add_eu_design_features(des_list);
                features.append(&mut fs);
            }
            else {         
                let mut fs = add_int_study_features(des_list);
                features.append(&mut fs);
                let mut fs = add_masking_features(des_list);
                features.append(&mut fs);
            }
        }
    }


    if phase_statement.is_some() {

        // Needs separate routine for eu-ctr.

        let phase_statement = &phase_statement.unwrap().to_lowercase();
        if source_id == 100123 {
            let mut fs = add_eu_phase_features(phase_statement);
            features.append(&mut fs);
        }
        else {
            let mut fs = add_phase_features(phase_statement);
            features.append(&mut fs);
        }
        
    }

    let study_features = match features.len() {
        0 => None,
        _ => Some(features)
    };


    let country_list = w.countries.tidy();
    let countries: Option<Vec<String>>;
    if country_list.is_some()
    {
        countries = split_and_dedup_countries(source_id, &country_list.unwrap());
    }
    else {
        countries = None;
    }


    let mut agemin = w.age_min.tidy();
    let mut agemin_units: Option<String> = None;
    if agemin.is_some()
    {
        let pat = r#"\d+"#;
        let re = Regex::new(pat).unwrap();
        let amin = agemin.unwrap();
        if re.is_match(&amin) {
            let caps = re.captures(&amin).unwrap();
            let min = &caps[0];
            agemin = Some(min.to_string());

            let min_units = amin.get_time_units();
            if min_units != "".to_string() {
                agemin_units = Some(min_units);
            }
        }
        else {
            agemin = None;
        }
    }


    let mut agemax = w.age_max.tidy();
    let mut agemax_units: Option<String> = None;
    if agemax.is_some()
    {
        let pat = r#"\d+"#;
        let re = Regex::new(pat).unwrap();
        let amax = agemax.unwrap();
        if re.is_match(&amax) {
            let caps = re.captures(&amax).unwrap();
            let max = &caps[0];
            agemax = Some(max.to_string());

            let max_units = amax.get_time_units();
            if max_units != "".to_string() {
                agemax_units = Some(max_units);
            }
        }
        else {
            agemax = None;
        }
    }

 
    let mut gender = w.gender.tidy();
    if gender.is_some()
    {
        let gen = gender.unwrap().to_lowercase();
        if gen.contains("both")
        {
            gender = Some("Both".to_string());
        }
        else
        {
            let f = gen.contains("female") || gen.contains("women") || gen == "f";
            let mut gen2 = gen.clone();
            if f {
                gen2 = gen.replace("female", "").replace("women", "")
            }
            let m = gen2.contains("male") || gen.contains("men") || gen == "m";
            
            if m && f
            {
                gender = Some("Both".to_string());
            }
            else if m {
                gender = Some("Male".to_string());
            }
            else if f {
                gender = Some("Female".to_string()); 
            }
            else if gen == "-" {
                gender = None;
            }
            else // still no match...
            {
                gender = Some(format!("?? Unable to classify ({})", gen));
            }
        }
    }


    let mut inc_crit = w.inclusion_criteria.tidy();
    if inc_crit.is_some() {
        let mut crit = inc_crit.unwrap();
        if crit.to_lowercase().starts_with("inclusion criteria")
        {
            let complex_trim = |c| c == ' ' || c == ':' || c == ',';
            crit = crit[18..].trim_matches(complex_trim).to_string();
        }
        inc_crit = crit.replace_tags_and_unicodes();
    }

    let mut exc_crit = w.exclusion_criteria.tidy();
    if exc_crit.is_some() {
        let mut crit = exc_crit.unwrap();
        if crit.to_lowercase().starts_with("exclusion criteria")
        {
            let complex_trim = |c| c == ' ' || c == ':' || c == ',';
            crit = crit[18..].trim_matches(complex_trim).to_string();
        }
        exc_crit = crit.replace_tags_and_unicodes();
    }

   
    let mut ipd_plan = w.results_ipd_plan.replace_tags_and_unicodes();
    if ipd_plan.is_some() {
         let plan = ipd_plan.clone().unwrap().to_lowercase();
         if plan.len() < 11 || plan == "not available" || plan == "not avavilable" 
            || plan == "not applicable" || plan.starts_with("justification or reason for")  
        {
            ipd_plan = None;
        }
    }

    let mut ipd_description = w.results_ipd_description.replace_tags_and_unicodes();
    if ipd_description.is_some() {
         let desc = ipd_description.clone().unwrap().to_lowercase();
         if desc.len() < 11 || desc == "not available" || desc == "not avavilable" 
            || desc == "not applicable" || desc.starts_with("justification or reason for")  
        {
            ipd_description = None;
        }
    }
   
    
    (source_id, Some(WHORecord  {
        source_id: source_id, 
        record_date: w.last_updated.as_iso_date().unwrap(),    // assumed to be always present
        sd_sid: sd_sid.to_string(), 
        pub_title: w.pub_title.replace_unicodes(),
        scientific_title: w.scientific_title.replace_unicodes(),
        remote_url: w.url.tidy(),
        pub_contact_givenname: w.pub_contact_first_name.tidy(),
        pub_contact_familyname: w.pub_contact_last_name.tidy(),
        pub_contact_email: w.pub_contact_email.tidy(),
        pub_contact_affiliation: w.pub_contact_affiliation.tidy(),
        scientific_contact_givenname: w.sci_contact_first_name.tidy(),
        scientific_contact_familyname: w.sci_contact_last_name.tidy(),
        scientific_contact_email: w.sci_contact_email.tidy(),
        scientific_contact_affiliation: w.sci_contact_affiliation.tidy(),
        study_type: stype,
        date_registration: w.date_registration.as_iso_date(),
        date_enrolment: w.date_enrollement.as_iso_date(),
        target_size: w.target_size.tidy(),
        study_status: status,
        primary_sponsor: w.primary_sponsor.tidy(),
        secondary_sponsors: w.secondary_sponsors.tidy(),
        source_support: w.source_support.tidy(),
        interventions: w.interventions.replace_tags_and_unicodes(),
        agemin: agemin,
        agemin_units:agemin_units,
        agemax: agemax,
        agemax_units: agemax_units,
        gender: gender,
        inclusion_criteria: inc_crit,
        exclusion_criteria: exc_crit,
        primary_outcome: w.primary_outcome.replace_tags_and_unicodes(),
        secondary_outcomes: w.secondary_outcomes.replace_tags_and_unicodes(),
        bridging_flag: w.bridging_flag.tidy(),
        bridged_type: w.bridged_type.tidy(),
        childs: w.childs.tidy(),
        type_enrolment: w.type_enrolment.tidy(),
        retrospective_flag: w.retrospective_flag.tidy(),
        results_actual_enrollment: w.results_actual_enrollment.tidy(),
        results_url_link: w.results_url_link.tidy(),
        results_summary: w.results_summary.tidy(),
        results_date_posted: w.results_date_posted.as_iso_date(),
        results_date_first_pub: w.results_date_first_pub.as_iso_date(),
        results_url_protocol: w.results_url_protocol.tidy(),
        ipd_plan: ipd_plan,
        ipd_description:ipd_description,
        results_date_completed: w.results_date_completed.as_iso_date(),
        results_yes_no: w.results_yes_no.tidy(),
        db_name: get_db_name(source_id),

        design_string: design_orig,
        phase_string: phase_orig,

        country_list: countries,
        secondary_ids: secids,
        study_features: study_features,
        condition_list: conditions_option,
        meddra_condition_list: meddraconds_option,
    }))
}


pub fn summarise_line(w: WHOLine, i: i32) -> Option<WHOSummary>  {

    let sid = w.trial_id.replace("/", "-").replace("\\", "-").replace(".", "-");
    let mut sd_sid = sid.trim().to_string();
    
    if sd_sid == "" || sd_sid == "null" || sd_sid == "NULL" {        // Seems to happen, or has happened in the past, with one Dutch trial.
        error!("Well that's weird - no study id on line {}!", i);
        return None;
    }

    let source_id = get_source_id(&sd_sid);
    if source_id == 0
    {
        error!("Well that's weird - can't match the study id's {} source on line {}!", sd_sid, i);
        return None;
       
    }

    if source_id == 100123 {
        sd_sid = sd_sid[0..19].to_string(); // lose country specific suffix
    }

    let mut title = w.pub_title.replace_unicodes();
    if title.is_none() {
        title = w.scientific_title.replace_unicodes();
    }
    

    let reg_year: Option<i32>;
    let reg_month: Option<i32>;
    let reg_day: Option<i32>;
    (reg_year, reg_month, reg_day) = split_iso_date(w.date_registration);

    let enrol_year: Option<i32>;
    let enrol_month: Option<i32>;
    let enrol_day: Option<i32>;
    (enrol_year, enrol_month, enrol_day) = split_iso_date(w.date_enrollement);
    
    let study_type = w.study_type.tidy();
    let stype = get_type(&study_type);

    let study_status = w.recruitment_status.tidy();
    let status = get_status(&study_status);
   
    let mut table_name = get_db_name(source_id);
    if source_id == 100120 {
        let suffix = match reg_year {
            Some(y) => if y < 2010 {
                    "_lt_2010"
                }
                else if y < 2015 {
                    "_2010_14"
                }
                else if y < 2020 {
                    "_2015_19"
                }
                else if y < 2025 {
                    "_2020_24"
                }
                else {  
                    "_2025_29"
                },
            None => "_LT_2010",
        };
        table_name = table_name + suffix;
    }

    if source_id == 100118 || source_id == 100121 || source_id == 100127 {
        let suffix = match reg_year {
            Some(y) => if y < 2020 {
                "_lt_2020"
            }
            else {
                "_ge_2020"
            }
            ,
            None => "_lt_2020",
        };
        table_name = table_name + suffix;
    }
    

    let country_list = w.countries.tidy();
    let countries: Option<Vec<String>>;
    if country_list.is_some()
    {
        countries = split_and_dedup_countries(source_id, &country_list.unwrap());
    }
    else {
        countries = None;
    }


    let res_posted = get_naive_date (w.results_date_posted);
    let res_first_pub = get_naive_date (w.results_date_first_pub);
    let res_completed = get_naive_date (w.results_date_completed);
    let date_last_rev = get_naive_date (w.last_updated);
  
    Some(WHOSummary {
            source_id: source_id, 
            sd_sid: sd_sid, 
            title: title,
            remote_url: w.url.tidy(),
            study_type: stype,
            study_status: status,
            reg_year: reg_year,
            reg_month: reg_month,
            reg_day: reg_day,
            enrol_year: enrol_year,
            enrol_month: enrol_month,
            enrol_day: enrol_day,
            results_yes_no: w.results_yes_no.tidy(),
            results_url_link: w.results_url_link.tidy(),
            results_url_protocol: w.results_url_protocol.tidy(),
            results_date_posted: res_posted,
            results_date_first_pub: res_first_pub,
            results_date_completed: res_completed,
            table_name: table_name,
            country_list: countries,
            date_last_rev: date_last_rev,    // assumed to be always present
})

}

fn split_iso_date (dt: String) -> (Option<i32>, Option<i32>, Option<i32>) {

    match dt.as_iso_date() {
        Some(d) => {
            if d.len() != 10 {
                println!("Odd iso date: {}", dt);
            }
            let year: i32 = d[0..4].parse().unwrap_or(0);
            let month: i32 = d[5..7].parse().unwrap_or(0);
            let day: i32 = d[8..].parse().unwrap_or(0);
            if year != 0 && month != 0 && day != 0 {
                (Some(year), Some(month), Some(day))           
            }
            else {
                (None, None, None)      
            }
         },
         None => (None, None, None),     
    }
}

fn get_naive_date (dt: String) -> Option<NaiveDate> {

   match dt.as_iso_date()
   {
        Some(s) => {
            let base_date = NaiveDate::parse_from_str("1900-01-01", "%Y-%m-%d").unwrap();
            let d = match NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => base_date,
            };
            
            if d != base_date {
                Some(d)
            }
            else {
                None
            }
        },
        None => None,
   }
}
