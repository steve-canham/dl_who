use std::sync::LazyLock;
use regex::Regex;
use log::error;
use chrono::NaiveDate;
use std::collections::HashSet;

use super::who_helper::{get_db_name, get_sid_type_id, get_type, get_status, 
    get_conditions, split_and_dedup_countries, add_study_purpose,
    add_int_study_features, add_obs_study_features, add_eu_design_features,
    add_masking, add_phase, add_eu_phase, split_ids, split_secids, process_sponsor_name};
use super::gen_helper::{StringExtensions, DateExtensions};
use super::file_models::{WHOLine, WHORecord, WhoStudyFeature, SecondaryId, WHOSummary};



pub fn summarise_line(w: &WHOLine, dl_id: i32, line_number: i32) -> Option<WHOSummary>  {

    let sid = w.trial_id.replace("/", "-").replace("\\", "-").replace(".", "-");
    let mut sd_sid = sid.trim().to_string();
    
    if sd_sid == "" || sd_sid == "null" || sd_sid == "NULL" {        // Seems to happen, or has happened in the past, with one Dutch trial.
        error!("Well that's weird - no study id on line {}!", line_number);
        return None;
    }

    let sid_type_id = get_sid_type_id(&sd_sid);
    if sid_type_id == 0
    {
        error!("Well that's weird - can't match the study id's {} source on line {}!", sd_sid, line_number);
        return None;
    }

    if sid_type_id == 123 {
        sd_sid = sd_sid[0..19].to_string(); // lose country specific suffix
    }
   
    let mut title = w.pub_title.replace_unicodes();
    if title.is_none() {
        title = w.scientific_title.replace_unicodes();
    }
    
    let study_type = w.study_type.tidy();
    let study_type_id = match &study_type {
         Some(t) => get_type(&t),
         None => 0
    };

    let study_status = w.recruitment_status.tidy();

    //Opportunity for a let-chain!
    
    let study_status_id = match &study_status {
         Some(s) => {
            if let Some(yn) = w.results_yes_no.tidy() && yn.to_lowercase() == "yes" {
                30   // study completed
            }
            else {
                get_status(&s, sid_type_id)
            }
         },
         None => 0
    };


    let sponsor_name = w.primary_sponsor.tidy();
    let sponsor_processed = process_sponsor_name(&sponsor_name);

    let mut secondary_ids: Vec<SecondaryId> = Vec::new();

    if let Some(s) = w.sec_ids.tidy()  {
        let initial_ids = Some(split_ids(&sd_sid, &s, "secondary ids"));
        if let Some(mut ids) = initial_ids {
            secondary_ids.append(&mut ids);
        }
    }
    
    if let Some(s) = w.bridging_flag.tidy() {
        let bridge_ids = Some(split_ids(&sd_sid, &s, "bridging flag"));
        if let Some(mut ids) = bridge_ids {
            secondary_ids.append(&mut ids);
        }
    }

    if let Some(s) = w.childs.tidy()  {
        let child_ids = Some(split_ids(&sd_sid, &s, "bridged child recs"));
        if let Some(mut ids) = child_ids {
            secondary_ids.append(&mut ids);
        }
    }

    // Secondary ids are often duplicated, need to be de-duplicated
    // using the .processed_id field

    let secids = match secondary_ids.len() {
        0 => None, 
        _ =>  {  
                let mut revised_list: Vec<SecondaryId> = Vec::new();
                if secondary_ids.len() == 1 {
                    revised_list = secondary_ids;
                }
                else {
                    let mut uniques:HashSet<String> = HashSet::new();
                    for secid in  secondary_ids {
                        if uniques.insert(secid.processed_id.clone()) {
                            revised_list.push(secid);
                        }
                    }
                }
                Some(revised_list)
            },
    };

    // Finally split the ids into two types - either trial registry Id or others;
    // and construct string vector from each
    
    let reg_ids: Option<Vec::<String>>;
    let oth_ids: Option<Vec::<String>>;
    (reg_ids, oth_ids) = split_secids(&secids);
    
   
    let date_last_rev = get_naive_date (&w.last_updated);
    
    let reg_year: i32;
    let date_reg = w.date_registration.as_iso_date();
    (reg_year, _ , _) = split_iso_date(&date_reg);

    let enrol_year: i32;
    let date_enrolment = w.date_enrollement.as_iso_date();
    (enrol_year, _, _) = split_iso_date(&date_enrolment);

    
    let mut table_name = get_db_name(sid_type_id);
    let mut suffix: &str;

    if sid_type_id == 120 {
        if reg_year < 2010 {
            suffix = "_lt_2010";
        }
        else if reg_year < 2015 {
            suffix = "_2010_14";
        }
        else if reg_year < 2020 {
            suffix = "_2015_19";
        }
        else if reg_year < 2025 {
            suffix = "_2020_24";
        }
        else {
            suffix = "_2025_29";
        }
        table_name = table_name + suffix;
    }

    if sid_type_id == 118 || sid_type_id == 121 
        || sid_type_id == 127 {
        if reg_year < 2020 {
            suffix = "_lt_2020";
        }
        else {
            suffix = "_ge_2020";
        }
        table_name = table_name + suffix;
    }
    

    let country_list = w.countries.tidy();
    let countries = match country_list {
        Some(c) => {split_and_dedup_countries(sid_type_id, &c)}
        None => None,
    };

    Some(WHOSummary {
        sid_type_id: sid_type_id, 
        sd_sid: sd_sid, 
        title: title,
        remote_url: w.url.tidy(),
        study_type: study_type,
        study_type_id: study_type_id,
        study_status: study_status,
        study_status_id: study_status_id,
        sponsor_name: sponsor_name,
        sponsor_processed: sponsor_processed,
        sec_ids: secids,
        reg_sec_ids: reg_ids,
        oth_sec_ids: oth_ids,
        reg_year: reg_year,
        enrol_year: enrol_year,
        results_yes_no: w.results_yes_no.tidy(),
        table_name: table_name,
        country_list: countries,
        date_last_rev_in_who: date_last_rev,  // assumed to be always present
        dl_id: dl_id,
    })
}


pub fn process_line(w: WHOLine, summ: &WHOSummary) -> Option<WHORecord>  {

    let sid_type_id = summ.sid_type_id;
    let study_type_id = summ.study_type_id;

    let design_orig = w.study_design.tidy();
    let phase_orig = w.phase.tidy();

    let condition_list = w.conditions.replace_unicodes();
    let (conditions, meddraconds) = match condition_list  {
        Some(cl) => {
            get_conditions(&cl, sid_type_id)
        }, 
        None => {
            (None, None)
        },
    };

   
    let mut features = Vec::<WhoStudyFeature>::new();
    
    if let Some(dl) = w.phase.tidy() {

        let phase_statement = &dl.to_lowercase();
        if sid_type_id == 123 || sid_type_id == 135 {
            if let Some(sf) = add_eu_phase(phase_statement)
            {
                features.push(sf);
            };
        }
        else {
            if let Some(sf) = add_phase(phase_statement)
            {
                features.push(sf);
            };
        }
    }
    
    if let Some(dl) =  w.study_design.tidy() {
    
        let des_list = &dl.to_lowercase();
        if sid_type_id == 123 || sid_type_id == 135 {
                let mut sfs = add_eu_design_features(des_list);
                features.append(&mut sfs);
        }
        else
        {
            if let Some(sf) = add_study_purpose(des_list)
            {
                features.push(sf);
            };

            if study_type_id == 12
            {
                let mut sfs = add_obs_study_features(des_list);
                features.append(&mut sfs);
            }
            else
            {      
                let mut sfs = add_int_study_features(des_list);
                features.append(&mut sfs);
                
                if let Some(sf) = add_masking(des_list)
                {
                    features.push(sf);
                };
            }
        }
    }
       
    let study_features = match features.len() {
        0 => None,
        _ => Some(features)
    };


    let mut agemin = w.age_min.tidy();
    let mut agemin_units: Option<String> = None;
    let mut agemax = w.age_max.tidy();
    let mut agemax_units: Option<String> = None;


    if sid_type_id != 123 {

        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

        (agemin, agemin_units)  =  match w.age_min.tidy() {

            Some(am) => {
                if RE.is_match(&am) {
                    let caps = RE.captures(&am).unwrap();
                    let min = &caps[0];
                    (Some(min.to_string()), am.get_time_units())
                }
                else {
                    (None, None)
                }
            }, 

            None => (None, None), 
        };

        (agemax, agemax_units)  =  match w.age_max.tidy() {

            Some(am) => {
                if RE.is_match(&am) {
                    let caps = RE.captures(&am).unwrap();
                    let max = &caps[0];
                    (Some(max.to_string()), am.get_time_units())
                }
                else {
                    (None, None)
                }
            }, 
            None => (None, None), 
        };
    }
    

    let gender = match w.gender.tidy() {

       Some(g) => {
            let glow = g.to_lowercase();
            if glow.contains("both")
            {
                Some("Both".to_string())
            }
            else
            {
                let f = glow.contains("female") || glow.contains("women") || glow == "f";
                let mut gen2 = glow.clone();
                if f {
                    gen2 = glow.replace("female", "").replace("women", "")
                }
                let m = gen2.contains("male") || glow.contains("men") || glow == "m";
                
                if m && f
                {
                    Some("Both".to_string())
                }
                else if m {
                    Some("Male".to_string())
                }
                else if f {
                    Some("Female".to_string())
                }
                else if glow == "-" {
                    None
                }
                else // still no match...
                {
                    Some(format!("?? Unable to classify ({})", g))
                }
        }
    },
        None => None,
    };


    let inc_crit = match  w.inclusion_criteria.tidy() {
        Some (mut s) => {
            if s.to_lowercase().starts_with("inclusion criteria")
            {
                let complex_trim = |c| c == ' ' || c == ':' || c == ',';
                s = s[18..].trim_matches(complex_trim).to_string();
            }

            let mut crit = s.clone();
            if sid_type_id == 123  {
               if let Some(pos) = s.find("Are the trial subjects under 18?")
               {
                    (agemin, agemin_units, agemax, agemax_units) = get_euro_ages(&s[pos..].to_string());
                     crit = s[..pos].to_string();
               }
            }
            crit.replace_tags_and_unicodes() 
        },
        None => None,
    };


    let exc_crit = match  w.exclusion_criteria.tidy() {
        Some (mut s) => {
            if s.to_lowercase().starts_with("exclusion criteria")
            {
                let complex_trim = |c| c == ' ' || c == ':' || c == ',';
                s = s[18..].trim_matches(complex_trim).to_string();
            }
            s.replace_tags_and_unicodes()
        },
        None => None,
    };
    

     let ipd_plan = match w.results_ipd_plan.replace_tags_and_unicodes() {
        Some (s) => {
            let plan = s.to_lowercase();
            if plan.len() < 11 || plan == "not available" || plan == "not avavilable" 
                || plan == "not applicable" || plan.starts_with("justification or reason for")  
            {
                None
            }
            else {
                Some(plan)
            }
        },
        None => None,
     };
   

     let ipd_description = match w.results_ipd_description.replace_tags_and_unicodes() {
        Some (s) => {
            let desc = s.to_lowercase();
            if desc.len() < 11 || desc == "not available" || desc == "not avavilable" 
                || desc == "not applicable" || desc.starts_with("justification or reason for")  
            {
                None
            }
            else {
                Some(desc)
            }
        },
        None => None,
     };
    
    Some(WHORecord  {
        sid_type_id: sid_type_id, 
        record_date: w.last_updated.as_iso_date(),
        sd_sid: summ.sd_sid.clone(), 
        pub_title: w.pub_title.replace_unicodes(),
        scientific_title: w.scientific_title.replace_unicodes(),
        remote_url: summ.remote_url.clone(),
        pub_contact_givenname: w.pub_contact_first_name.tidy(),
        pub_contact_familyname: w.pub_contact_last_name.tidy(),
        pub_contact_email: w.pub_contact_email.tidy(),
        pub_contact_affiliation: w.pub_contact_affiliation.tidy(),
        scientific_contact_givenname: w.sci_contact_first_name.tidy(),
        scientific_contact_familyname: w.sci_contact_last_name.tidy(),
        scientific_contact_email: w.sci_contact_email.tidy(),
        scientific_contact_affiliation: w.sci_contact_affiliation.tidy(),
        study_type_orig: summ.study_type.to_owned(),
        study_type_id: summ.study_type_id,

        date_registration: w.date_registration.as_iso_date(),
        date_enrolment: w.date_enrollement.as_iso_date(),
        target_size: w.target_size.tidy(),
        study_status_orig: summ.study_status.to_owned(),
        study_status_id: summ.study_status_id,
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
        
        results_yes_no: w.results_yes_no.tidy(),       
        results_url_link: w.results_url_link.tidy(),
        results_summary: w.results_summary.tidy(),
        results_date_posted: w.results_date_posted.as_iso_date(),
        results_date_first_pub: w.results_date_first_pub.as_iso_date(),
        results_url_protocol: w.results_url_protocol.tidy(),
        results_date_completed: w.results_date_completed.as_iso_date(),

        ipd_plan: ipd_plan,
        ipd_description:ipd_description,
        design_string: design_orig,
        phase_string: phase_orig,
        country_list: summ.country_list.to_owned(),
        secondary_ids: summ.sec_ids.to_owned(),
        study_features: study_features,
        condition_list: conditions,
        meddra_condition_list: meddraconds,
    })
}


fn get_naive_date (dt: &String) -> Option<NaiveDate> {

   match dt.as_iso_date()
   {
        Some(s) => {
            match NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                Ok(nd) => Some(nd),
                Err(_) => None
            }
        },
        None => None,
   }
}


fn split_iso_date (dt: &Option<String>) -> (i32, i32, i32) {

    match dt {
        Some(d) => {
            if d.len() != 10 {
                println!("Odd iso date: {}", d);
            }
            let year: i32 = d[0..4].parse().unwrap_or(0);
            let month: i32 = d[5..7].parse().unwrap_or(0);
            let day: i32 = d[8..].parse().unwrap_or(0);
            if year != 0 && month != 0 && day != 0 {
                (year, month, day)           
            }
            else {
                (0, 0, 0)      
            }
         },
         None => (0, 0, 0),     
    }
}


fn get_euro_ages(age_string: &String) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
            
    let mut agemin= None;
    let mut agemin_units: Option<String> = None;
    let mut agemax= None;
    let mut agemax_units: Option<String> = None;

    let children: bool = if age_string.contains("Are the trial subjects under 18? yes") {true} else {false};
    let adult: bool = if age_string.contains("F.1.2 Adults (18-64 years) yes") {true} else {false};
    let aged: bool = if age_string.contains("F.1.3 Elderly (>=65 years) yes") {true} else {false};
     
    if children {
        if !adult {
            agemax = Some("17".to_string());
            agemax_units = Some("Years".to_string());
        }
        else {   // adult 
            if !aged {
                agemax = Some("64".to_string());
                agemax_units = Some("Years".to_string());
            }
        }
    }
    else {     // no children
        if adult {
            agemin = Some("18".to_string());
            agemin_units = Some("Years".to_string());

            if !aged {
                agemax = Some("64".to_string());
                agemax_units = Some("Years".to_string());
            }
        }
        else {       // aged only
            agemin = Some("65".to_string());
            agemin_units = Some("Years".to_string());
        }
    }

    (agemin, agemin_units, agemax, agemax_units) 

}
