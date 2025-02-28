//use crate::AppError;
//use std::path::PathBuf;
//use crate::DownloadResult;

use regex::Regex;
use log::error;
use super::who_helper::{get_db_name, get_source_id, get_status, 
    get_conditions, split_and_dedup_countries, 
    add_int_study_features, add_obs_study_features, 
    add_masking_features, add_phase_features, split_and_add_ids};
use super::gen_helper::{StringExtensions, DateExtensions};
use super::file_model::{WHOLine, WHORecord, WhoStudyFeature, SecondaryId};


pub fn process_line(w: WHOLine, i: i32) -> Option<i32>  {

    let sid = w.trial_id.replace("/", "-").replace("\\", "-").replace(".", "-");
    let sd_sid = sid.trim();
    
    if sd_sid == "" || sd_sid == "null" || sd_sid == "NULL" {        // Seems to happen, or has happened in the past, with one Dutch trial.
        error!("Well that's weird - no study id on line {}!", i);
        return None;
    }

    let source_id = get_source_id(sd_sid);
    if source_id == 100120 || source_id == 100126  // no need to process these - details input directly from registry (for CGT, ISRCTN).
    {
        return None;
    }
            
    if source_id == 0
    {
        error!("Well that's weird - can't match the study id's {} source on line {}!", sd_sid, i);
        return None;
       
    }

    println!("{}, {}\n", sd_sid, source_id);

    
    let mut study_type = w.study_type.tidy();
    if study_type.is_some()
    {
        let stype = study_type.clone().unwrap().to_lowercase();
        if stype.starts_with("intervention")
        {
            study_type = Some("Interventional".to_string());
        }
        else if stype.starts_with("observation")
              || stype.starts_with("epidem")
        {
            study_type = Some("Observational".to_string());
        }
        else
        {
            study_type = Some(format!("Other ({})", study_type.unwrap()));
        }
    }
   
    let mut study_status = w.recruitment_status.tidy();
    if study_status.is_some() {
        let status = study_status.clone().unwrap().to_lowercase();
        if status.len() > 5
        {
            study_status = get_status(&status);
        }
        else {
            study_status = None;
        }
    }

    let design_list = w.study_design.tidy();
    let design_orig = design_list.clone();
    let phase_statement = w.phase.tidy();
    let phase_orig = phase_statement.clone();


    let condition_list = w.conditions.replace_unicodes();
    let conditions: Option<Vec<String>>;
    if condition_list.is_some()
    {
        // Need a specific routine to deal with EUCTR and the MedDRA listings

        conditions = get_conditions(&condition_list.unwrap());
    }
    else {
        conditions = None;
    }
   


    let secondary_ids = Vec::<SecondaryId>::new();
    let sec_ids = w.sec_ids.tidy();

    if sec_ids.is_some()
    {
        secondary_ids = split_and_add_ids(secondary_ids, sd_sid, sec_ids, "secondary ids");
    }
        
    let bridging_flag = w.bridging_flag.tidy();
    if bridging_flag.is_some() && bridging_flag.unwrap() != sd_sid
    {
        secondary_ids = split_and_add_ids(secondary_ids, r.sd_sid, r.bridging_flag, "bridging flag");
    }

    let bridged_type = w.bridged_type.tidy();
    let childs = w.childs.tidy();
    if childs.is_some()
    {
        secondary_ids = split_and_add_ids(secondary_ids, sd_sid, childs, "bridged child recs");
    }
  

    let mut features = Vec::<WhoStudyFeature>::new();

    if design_list.is_some()  {
        let des_list = &design_list.unwrap();

        // Needs separate routines for eu-ctr

        if study_type == Some("Observational".to_string())
        {
            let mut fs = add_obs_study_features(des_list);
            features.append(&mut fs);
        }
        else
        {               
            let mut fs = add_int_study_features(des_list);
            features.append(&mut fs);
            let mut fs = add_masking_features(des_list);
            features.append(&mut fs);
        }
    }


    if phase_statement.is_some() {

        // Needs separate routine for eu-ctr

        let mut fs = add_phase_features(&phase_statement.unwrap());
        features.append(&mut fs);
    }

    let study_features = match features.len() {
        0 => None,
        _ => Some(features)
    };


    let country_list = w.countries.tidy();
    let countries: Option<Vec<String>>;
    if country_list.is_some()
    {
        countries = split_and_dedup_countries(&country_list.unwrap());
    }
    else {
        countries = None;
    }


    let mut study_type = w.study_type.tidy();
    if study_type.is_some()
    {
        let stype = study_type.clone().unwrap().to_lowercase();
        if stype.starts_with("intervention")
        {
            study_type = Some("Interventional".to_string());
        }
        else if stype.starts_with("observation")
              || stype.starts_with("epidem")
        {
            study_type = Some("Observational".to_string());
        }
        else
        {
            study_type = Some(format!("Other ({})", study_type.unwrap()));
        }
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
   
    
    let rec = 
        WHORecord 
        {
        source_id: source_id, 
        record_date: w.last_updated.as_iso_date(),
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
        study_type: study_type,
        date_registration: w.date_registration.as_iso_date(),
        date_enrolment: w.date_enrollement.as_iso_date(),
        target_size: w.target_size.tidy(),
        study_status: study_status,
        primary_sponsor: w.primary_sponsor.tidy(),
        secondary_sponsors: w.secondary_sponsors.tidy(),
        source_support: w.source_support.tidy(),
        interventions: w.interventions.tidy(),
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
        results_date_first_pubation: w.results_date_first_pubation.as_iso_date(),
        results_url_protocol: w.results_url_protocol.tidy(),
        ipd_plan: ipd_plan,
        ipd_description:ipd_description,
        results_date_completed: w.results_date_completed.as_iso_date(),
        results_yes_no: w.results_yes_no.tidy(),
        db_name: get_db_name(source_id),

        design_string: design_orig,
        phase_string: phase_orig,

        country_list: countries,
        secondary_ids: None,
        study_features: study_features,
        condition_list: conditions,
    };

    print!("{:#?}\n", rec );

    Some(42)

}


/* 
         
            List<Secondary_Id> secondary_ids = new List<Secondary_Id>();
            string? sec_ids = sr.SecondaryIDs.Tidy();
            if (!string.IsNullOrEmpty(sec_ids))
            {
                secondary_ids = _wh.SplitAndAddIds(secondary_ids, sd_sid, sec_ids, "secondary ids");
            }
            
            
            r.bridging_flag = sr.Bridging_flag.Tidy();
            if (!string.IsNullOrEmpty(r.bridging_flag) && r.bridging_flag != r.sd_sid)
            {
                secondary_ids = _wh.SplitAndAddIds(secondary_ids, r.sd_sid, r.bridging_flag, "bridging flag");
            }

            r.bridged_type = sr.Bridged_type.Tidy();

            r.childs = sr.Childs.Tidy();
            if (!string.IsNullOrEmpty(r.childs))
            {
                secondary_ids = _wh.SplitAndAddIds(secondary_ids, r.sd_sid, r.childs, "bridged child recs");
            }
          
            r.secondary_ids = secondary_ids;




            // Need to check if still required...

            if (source_id is 100132 && sd_sid.StartsWith("NTR"))
            {
                // For the Dutch trials there is confusion over the sd_sid, which
                // should all be NL numbers, but older trials are presented by WHO as 
                // NTR trials, with the NL as a secondary id. In fact it is the other way round!
                // Though all about to be superseded anyway by the new dutch trial registry...

                if (secondary_ids.Any())
                {
                    foreach (Secondary_Id sec_id in secondary_ids)
                    {
                        if (sec_id.processed_id is not null 
                            &&  Regex.Match(sec_id.processed_id!, @"^NL\d{1,4}$").Success)
                        {
                            string new_sd_sid = sec_id.processed_id;
                            sec_id.processed_id = sd_sid;  // change the secondary id to the old sd_sid
                            sec_id.sec_id = sd_sid;
                            sec_id.sec_id_type_id = 45;
                            sec_id.sec_id_type = "Obsolete NTR number";
                            sd_sid = new_sd_sid;
                            r.sd_sid = sd_sid;
                            break;
                        }
                    }
                }
            }
                        


        }
    }
}


*/