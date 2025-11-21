use std::sync::LazyLock;
use regex::Regex;
use log::error;
use chrono::NaiveDate;
use std::collections::HashSet;

use super::who_helper::{get_db_name, get_source_id, get_type, get_status, 
    get_conditions, split_and_dedup_countries,
    add_int_study_features, add_obs_study_features, add_eu_design_features,
    add_masking_features, add_phase_features, add_eu_phase_features, split_ids};
use super::gen_helper::{StringExtensions, DateExtensions};
use super::file_models::{WHOLine, WHORecord, WhoStudyFeature, SecondaryId, WHOSummary};


pub fn process_line(w: WHOLine, summ: &WHOSummary) -> Option<WHORecord>  {

    let source_id = summ.source_id;
    let study_type = summ.study_type;

    let design_orig = w.study_design.tidy();
    let phase_orig = w.phase.tidy();

    let condition_list = w.conditions.replace_unicodes();
    let (conditions, meddraconds) = match condition_list  {
        Some(cl) => {
            get_conditions(&cl, source_id)
        }, 
        None => {
            (None, None)
        },
    };

   
    let mut features = Vec::<WhoStudyFeature>::new();

    match w.study_design.tidy() {
       Some(dl) => {
            let des_list = &dl.to_lowercase();
            if study_type == 11
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
       },
       None => {},
    }


    if let Some(dl) = w.phase.tidy() {
        let phase_statement = &dl.to_lowercase();
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


    let mut agemin = w.age_min.tidy();
    let mut agemin_units: Option<String> = None;
    let mut agemax = w.age_max.tidy();
    let mut agemax_units: Option<String> = None;


    if source_id != 100123 {

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
            let gen = g.to_lowercase();
            if gen.contains("both")
            {
                Some("Both".to_string())
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
                    Some("Both".to_string())
                }
                else if m {
                    Some("Male".to_string())
                }
                else if f {
                    Some("Female".to_string())
                }
                else if gen == "-" {
                    None
                }
                else // still no match...
                {
                    Some(format!("?? Unable to classify ({})", gen))
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
            if source_id == 100123  {
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
        source_id: source_id, 
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
        study_type_orig: w.study_type.tidy(),
        study_type: summ.study_type,
        date_registration: w.date_registration.as_iso_date(),
        date_enrolment: w.date_enrollement.as_iso_date(),
        target_size: w.target_size.tidy(),
        study_status_orig: w.recruitment_status.tidy(),
        study_status: summ.study_status,
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
        design_string: design_orig,
        phase_string: phase_orig,
        country_list: summ.country_list.to_owned(),
        secondary_ids: summ.secondary_ids.to_owned(),
        study_features: study_features,
        condition_list: conditions,
        meddra_condition_list: meddraconds,
    })
}


pub fn summarise_line(w: &WHOLine, dl_id: i32, line_number: i32) -> Option<WHOSummary>  {

    let sid = w.trial_id.replace("/", "-").replace("\\", "-").replace(".", "-");
    let mut sd_sid = sid.trim().to_string();
    
    if sd_sid == "" || sd_sid == "null" || sd_sid == "NULL" {        // Seems to happen, or has happened in the past, with one Dutch trial.
        error!("Well that's weird - no study id on line {}!", line_number);
        return None;
    }

    let source_id = get_source_id(&sd_sid);
    if source_id == 0
    {
        error!("Well that's weird - can't match the study id's {} source on line {}!", sd_sid, line_number);
        return None;
    }

    if source_id == 100123 {
        sd_sid = sd_sid[0..19].to_string(); // lose country specific suffix
    }
   
    let mut title = w.pub_title.replace_unicodes();
    if title.is_none() {
        title = w.scientific_title.replace_unicodes();
    }
    
    let stype = get_type(&w.study_type.tidy());

    let status = if w.results_yes_no.to_lowercase() == "yes" {
        30   // completed
    }
    else {
        get_status(&w.recruitment_status.tidy())
    };
 

    let sec_ids = w.sec_ids.tidy();
    let initial_ids = match sec_ids {
        Some(s) => {
            Some(split_ids(&sd_sid, &s, "secondary ids"))
            },
        None => None,
    };
  
    let bridging_flag = w.bridging_flag.tidy();
    let bridge_ids =  match bridging_flag {
        Some(s) => {
                Some(split_ids(&sd_sid, &s, "bridging flag"))
            },         
        None => None,
    };
       
    let childs = w.childs.tidy();
    let child_ids = match childs {
        Some(s) => {
            Some(split_ids(&sd_sid, &s, "bridged child recs"))
            },
        None => None,
    };
 

    let mut secondary_ids: Vec<SecondaryId> = Vec::new();

    if let Some(mut v) = initial_ids {
        secondary_ids.append(&mut v);
    }
    if let Some(mut v) = bridge_ids {
        secondary_ids.append(&mut v);
    }
    if let Some(mut v) = child_ids {
        secondary_ids.append(&mut v);
    }   

    let secids = match secondary_ids.len() {
        0 => None, 
        _ =>  {  
                let mut uniques = HashSet::new();    // Dedup here
                secondary_ids.retain(|e| uniques.insert(e.clone()));
                Some(secondary_ids)
            },
    };
   

    let res_posted = get_naive_date (&w.results_date_posted);
    let res_first_pub = get_naive_date (&w.results_date_first_pub);
    let res_completed = get_naive_date (&w.results_date_completed);
    let date_last_rev = get_naive_date (&w.last_updated);
    
    let date_reg = w.date_registration.as_iso_date();
    let reg_year: i32;
    let reg_month: i32;
    let reg_day: i32;
    (reg_year, reg_month, reg_day) = split_iso_date(&date_reg);

    let date_enrolment = w.date_enrollement.as_iso_date();
    let enrol_year: i32;
    let enrol_month: i32;
    let enrol_day: i32;
    (enrol_year, enrol_month, enrol_day) = split_iso_date(&date_enrolment);

    
    let mut table_name = get_db_name(source_id);
    let mut suffix: &str;

    if source_id == 100120 {
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

    if source_id == 100118 || source_id == 100121 
        || source_id == 100127 {
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
        Some(c) => {split_and_dedup_countries(source_id, &c)}
        None => None,
    };

    Some(WHOSummary {
        source_id: source_id, 
        sd_sid: sd_sid, 
        title: title,
        remote_url: w.url.tidy(),
        study_type: stype,
        study_status: status,
        secondary_ids: secids,
        date_registration: date_reg,
        reg_year: reg_year,
        reg_month: reg_month,
        reg_day: reg_day,
        date_enrolment: date_enrolment,
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
        date_last_rev: date_last_rev,  // assumed to be always present
        dl_id: dl_id,
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
