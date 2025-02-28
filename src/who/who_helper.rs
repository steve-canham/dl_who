use super::file_model::{SecondaryId, SecIdBase, WhoStudyFeature};

pub fn get_source_id(sd_sid: &str) -> usize {
    match sd_sid {
        _ if sd_sid.starts_with("NCT") => 100120,
        _ if sd_sid.starts_with("CHICTR") => 100118,
        _ if sd_sid.starts_with("CTRI") => 100121,
        _ if sd_sid.starts_with("JPRN") =>  100127,
        _ if sd_sid.starts_with("EUCTR") => 100123,
        _ if sd_sid.starts_with("ISRCTN") => 100126,
        _ if sd_sid.starts_with("ACTRN") => 100116,
        _ if sd_sid.starts_with("DRKS") => 100124,
        _ if sd_sid.starts_with("IRCT") => 100125,
        _ if sd_sid.starts_with("KCT") =>  100119,
        _ if sd_sid.starts_with("NL") || sd_sid.starts_with("NTR") => 100132,
        _ if sd_sid.starts_with("CTIS") => 110428,
        _ if sd_sid.starts_with("RBR") => 100117, 
        _ if sd_sid.starts_with("RPCEC") => 100122,
        _ if sd_sid.starts_with("PACTR") => 100128,
        _ if sd_sid.starts_with("PER") =>  100129,
        _ if sd_sid.starts_with("SLCTR") => 100130,
        _ if sd_sid.starts_with("TCTR") => 100131,
        _ if sd_sid.starts_with("LBCTR") => 101989,
        _ if sd_sid.starts_with("ITMCTR") => 109108,
        _ => 0
    }
}


pub fn get_db_name (source_id: usize) -> String {
    let db_name = match source_id {
        100116 => "anzctr",
        100117 => "rebec",
        100118 => "chictr",
        100119 => "cris",
        100121 => "ctri",
        100122 => "rpcec",
        100123 => "euctr",
        100124 => "drks",
        100125 => "irct",
        100127 => "jprn",
        100128 => "pactr",
        100129 => "rpuec",
        100130 => "slctr",
        100131 => "thctr",
        100132 => "nntr",
        110428 => "ctis",
        101989 => "lebctr",
        109108 => "itmctr",
        _ => ""
    };
    db_name.to_string()
}


pub fn get_status(status: &String) -> Option<String> {

    if status == "complete" || status == "completed" 
    || status == "complete: follow-up complete" || status == "complete: follow up complete" 
    || status == "data analysis completed" || status == "main results already published"
    {
        Some("Completed".to_string())
    }
    else if status == "complete: follow-up continuing" 
             || status == "complete: follow up continuing" || status == "active, not recruiting" 
             || status == "closed to recruitment of participants" || status == "no longer recruiting" 
             || status == "not recruiting" || status == "recruitment completed"
    {
        Some("Active, not recruiting".to_string())
    }
    else if status == "recruiting" || status =="open public recruiting" 
    || status == "open to recruitment"
    {
        Some("Recruiting".to_string())
    }
    else if status.contains("pending")
          || status == "not yet recruiting"
    {
        Some("Not yet recruiting".to_string())
    }
    else if status.contains("suspended")
          || status.contains("temporarily closed")
    {
        Some("Suspended".to_string())
    }
    else if status.contains("terminated")
          || status.contains("stopped early")
    {
        Some("Terminated".to_string())
    }
    else if status.contains("withdrawn")
    {
        Some("Withdrawn".to_string())
    }
    else if status.contains("enrolling by invitation")
    {
        Some("Enrolling by invitation".to_string())
    }
    else
    {
        Some(format!("Other ({})", status))
    }
}

pub fn get_conditions(condition_list: &String) -> Option<Vec<String>> {

    // replace line breaks and hashes with semi-colons, and split

    let mut clist = condition_list.replace("<br>", ";").replace("<br/>", ";");
    clist = clist.replace("#", ";");

    let sep_conds: Vec<&str> = clist.split(";").collect();
    let mut conds = Vec::<String>::new();

    for s in sep_conds
    {
        let complex_trim = |c| c == ' ' || c == '('|| c == '.' || c == ';' || c == '-';
        let s1 = s.trim_matches(complex_trim);
        if s1 != "" && s1.len() >= 3
        {
            conds.push(s1.to_string());

            // Processing code for condition data now all moved to Harvester
            // module, as it is easier to correct and extend there (changes
            // do not require global WHO re-download!).
            // Conditions exported from here a a simple string array.
            
        }
    }
    
    match conds.len() {
        0 => None,
        _ => Some(conds),
    }

}



pub fn split_and_dedup_countries(country_list: &String) -> Option<Vec<String>> {

    // country list known to be non-null and already 'tidied'.

    let in_strings: Vec<&str> = country_list.split(';').collect();
    let mut out_strings = Vec::<String>::new();

    for c in in_strings
    {
        if out_strings.len() == 0
        {
            out_strings.push(c.to_string());
        }
        else
        {
            let mut add_string = true;
            for s in &out_strings
            {
                if s == c
                {
                    add_string = false;
                    break;
                }
            }
            if add_string {
                out_strings.push(c.to_string());
            }
        }
    }

    return Some(out_strings);
}



pub fn add_int_study_features(design_list: &String) -> Vec<WhoStudyFeature>
{
    let mut fs = Vec::<WhoStudyFeature>::new();
    let design = design_list.replace(" :", ":").to_lowercase(); // to make comparisons easier

    if design.contains("purpose: treatment")
    {
        fs.push(WhoStudyFeature::new(21, "Primary purpose", 400, "Treatment"));
    }
    if design.contains("purpose: diagnosis") || design.contains("diagnostic")
    {
        fs.push(WhoStudyFeature::new(21, "Primary purpose", 410, "Diagnostic"));
    }    
    if design.contains("supportive care") || design.contains("purpose: supportive")
    {
        fs.push(WhoStudyFeature::new(21, "Primary purpose", 415, "Supportive care"));
    }

    if design.contains("non-randomized")   
     || design.contains("nonrandomized")
     || design.contains("non-randomised")
     || design.contains("nonrandomised")
     || design.contains("non-rct")
    {
        fs.push(WhoStudyFeature::new(22, "Allocation type", 210, "Nonrandomised"));
    }
    else if design.contains("randomized")
         || design.contains("randomised")
         || design.contains(" rct")
    {
        fs.push(WhoStudyFeature::new(22, "Allocation type", 205, "Randomised"));
    }

    if design.contains("parallel")
    {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 305, "Parallel assignment"));
    }

    if design.contains("crossover")
    {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 310, "Crossover assignment"));
    }

    if design.contains("factorial")
    {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 315, "Factorial assignment"));
    }

    fs
}


pub fn add_obs_study_features(design_list: &String) -> Vec<WhoStudyFeature>
{
    let mut fs = Vec::<WhoStudyFeature>::new();
    let design = design_list.replace(" :", ":").to_lowercase(); // to make comparisons easier

    if design.contains("observational study model")
    {
        if design.contains("cohort")
        {
            fs.push(WhoStudyFeature::new(30, "Observational model", 600, "Cohort"));
        }
        if design.contains("case-control") || design.contains("case control")
        {
            fs.push(WhoStudyFeature::new(30, "Observational model", 605, "Case-control"));
        }
        if design.contains("case-crossover") || design.contains("case crossover")
        {
            fs.push(WhoStudyFeature::new(30, "Observational model", 615, "Case-crossover"));
        }

    }
    if design.contains("time perspective")
    {
        if design.contains("retrospective")
        {
            fs.push(WhoStudyFeature::new(31, "Time perspective", 700, "Retrospective"));
        }
        if design.contains("prospective")
        {
            fs.push(WhoStudyFeature::new(31, "Time perspective", 705, "Prospective"));
        }
        if design.contains("cross-sectional") || design.contains("crosssectional")
        {
            fs.push(WhoStudyFeature::new(31, "Time perspective", 710, "Cross-sectional"));
        }
        if design.contains("longitudinal")
        {
            fs.push(WhoStudyFeature::new(31, "Time perspective", 730, "longitudinal"));
        }
    }


    if design.contains("biospecimen retention")
    {
        if design.contains("not collect nor archive")
        {
            fs.push(WhoStudyFeature::new(32, "Biospecimens retained", 800, "None retained"));
        }
        if design.contains("collect & archive- sample with dns")
        {
            fs.push(WhoStudyFeature::new(32, "Biospecimens retained", 805, "Samples with DNA"));
        }
    }

    fs
}


pub fn add_masking_features(design_list: &String) -> Vec<WhoStudyFeature>
{
    let mut fs = Vec::<WhoStudyFeature>::new();
    let design = design_list.replace(" :", ":").to_lowercase(); // to make comparisons easier

    if design.contains("open label")
       || design.contains("open-label")
       || design.contains("no mask")
       || design.contains("masking not used")
       || design.contains("not blinded")
       || design.contains("non-blinded")
       || design.contains("no blinding")
       || design.contains("no masking")
       || design.contains("masking: none")
       || design.contains("masking: open")
       || design.contains("blinding: open")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 500, "None (Open Label)"));
    }
    else if design.contains("single blind")
     || design.contains("single-blind")
     || design.contains("single - blind")
     || design.contains("masking: single")
     || design.contains("outcome assessor blinded")
     || design.contains("participant blinded")
     || design.contains("investigator blinded")
     || design.contains("blinded (patient/subject)")
     || design.contains("blinded (investigator/therapist)")
     || design.contains("blinded (assessor)")
     || design.contains("blinded (data analyst)")
     || design.contains("uni-blind")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 505, "Single"));
    }
    else if design.contains("double blind")
     || design.contains("double-blind")
     || design.contains("doble-blind")
     || design.contains("double - blind")
     || design.contains("double-masked")
     || design.contains("masking: double")
     || design.contains("blinded (assessor, data analyst)")
     || design.contains("blinded (patient/subject, investigator/therapist")
     || design.contains("masking:participant, investigator, outcome assessor")
     || design.contains("participant and investigator blinded")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 510, "Double"));
    }
    else if design.contains("triple blind")
     || design.contains("triple-blind")
     || design.contains("blinded (patient/subject, caregiver, investigator/therapist, assessor")
     || design.contains("masking:participant, investigator, outcome assessor")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 515, "Triple"));
    }
    else if design.contains("quadruple blind") || design.contains("quadruple-blind")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 520, "Quadruple"));
    }
    else if design.contains("masking used") || design.contains("blinding used")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 502, "Blinded (no details)"));
    }
    else if design.contains("masking:not applicable")
     || design.contains("blinding:not applicable")
     || design.contains("masking not applicable")
     || design.contains("blinding not applicable")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 599, "Not applicable"));
    }
    else if design.contains("masking: unknown")
    {
        fs.push(WhoStudyFeature::new(24, "Masking", 525, "Not provided"));
    }

    fs
}


pub fn add_phase_features(phase_list: &String) -> Vec<WhoStudyFeature>
{
    let mut fs = Vec::<WhoStudyFeature>::new();
    let phase = phase_list.to_lowercase();

    if phase != "not selected" && phase != "not applicable"
        && phase != "na" && phase != "n/a"
    {
        if phase == "phase 0" || phase == "phase-0" || phase == "phase0" 
        || phase ==  "0" || phase ==  "0 (exploratory trials)" 
        || phase == "phase 0 (exploratory trials)" || phase ==  "0 (exploratory trials)"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 105, "Early phase 1"));
        }
        else if phase == "1" || phase ==  "i" || phase ==  "i (phase i study)" 
                 || phase == "phase-1" || phase ==  "phase 1" || phase ==  "phase i" || phase ==  "phase1"
        {
            fs.push(WhoStudyFeature::new(20, "phase", 110, "Phase 1"));
        }
        else if phase == "1-2" || phase ==  "1 to 2" || phase ==  "i-ii" 
        || phase ==  "i+ii (phase i+phase ii)" || phase ==  "phase 1-2" 
        || phase ==  "phase 1 / phase 2" || phase ==  "phase 1/ phase 2" 
        || phase == "phase 1/phase 2" || phase ==  "phase i,ii" || phase == "phase1/phase2"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 115, "Phase 1/Phase 2"));
        }
        else if phase == "2" || phase ==  "2a" || phase ==  "2b" 
        || phase ==  "ii" || phase ==  "ii (phase ii study)" || phase ==  "iia" 
        || phase ==  "iib" || phase ==  "phase-2" || phase ==  "phase 2" || phase ==  "phase ii" || phase ==  "phase2"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 120, "Phase 2"));
        }
        else if phase == "2-3" || phase == "ii-iii" || phase ==  "phase 2-3" 
        || phase == "phase 2 / phase 3" || phase == "phase 2/ phase 3" 
        || phase ==  "phase 2/phase 3" || phase == "phase2/phase3" || phase == "phase ii,iii"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 125, "Phase 2/Phase 3"));
        }
        else if phase == "3" || phase ==  "iii" || phase ==  "iii (phase iii study)" 
        || phase ==  "iiia" || phase ==  "iiib" || phase ==  "3-4" || phase ==  "phase-3" 
        || phase ==  "phase 3" || phase ==  "phase 3 / phase 4" 
        || phase ==  "phase 3/ phase 4" || phase ==  "phase3" || phase ==  "phase iii"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 130, "Phase 3"));
        }
        else if phase == "4" || phase ==  "iv" || phase ==  "iv (phase iv study)" 
                 || phase == "phase-4" || phase ==  "phase 4" || phase ==  "post-market" 
                 || phase ==  "post marketing surveillance" || phase ==  "phase4" || phase ==  "phase iv"
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 135, "Phase 4"));
        }
        else
        {
            fs.push(WhoStudyFeature::new(20, "Phase", 1500, phase_list));
        }
    }

    fs
}


    pub fn  split_and_add_ids(sd_sid: &String, in_string: &String, source_field: &String) -> Vec<SecondaryId>
    {
        // in_string already known to be non-null, non-empty.

        let ids: Vec<&str> = in_string.split(";").collect();
        let mut id_list = Vec::<SecondaryId>::new();

        for s in ids
        {
            let complex_trim = |c| c==' '|| c =='\''|| c ==';'|| c=='‘'|| c=='’';
            let secid = s.trim_matches(complex_trim);

            if secid.len() >= 3 && secid != sd_sid
            {
                let secid_low = secid.to_lowercase();
                if secid_low.chars().any(|c| c.is_digit(10))    // has to include at least 1 number
                    && !secid_low.starts_with("none")
                    && !secid_low.starts_with("nil")
                    && !secid_low.starts_with("not ")
                    && !secid_low.starts_with("date")
                    && !secid_low.starts_with("version")
                    && !secid_low.starts_with("??")
                {
                    let sec_id_base = get_sec_id_details(secid, sd_sid);

                    /*  
                    // for now, mmove the duplicate checking back to the calling routine
                    // once all the ids have been collected
                    // Has this id been added before?
                  
                    let mut add_id = true;
                   
                    if existing_ids.len() > 0
                    {
                        for secid in existing_ids
                        {
                            if (sec_id_base.processed_id == secid.processed_id)
                            {
                                add_id = false;
                                break;
                            }
                        }
                    }
                    if (add_id)
                    {*/

                    id_list.push(SecondaryId::new(source_field.to_string(), secid.to_string(),
                    sec_id_base.processed_id, sec_id_base.sec_id_source,
                    sec_id_base.sec_id_type_id, sec_id_base.sec_id_type));

    
                }
            }
        }

        id_list  // may overlap ids with the ids from the other source fields
    }
     

    pub fn get_sec_id_details(sec_id: &str, sd_sid: &String) -> SecIdBase 
    {
        let interim_id: &str;
        let processed_id: Option<String> = None;
        let sec_id_source: Option<usize> = None;
        let isec_id_type_id: Option<usize> = None;
        
        if sec_id.Contains("NCT")
        {
            let interim_id = sec_id.replace("NCT ", "NCT").replace("NCTNumber", "");
            let re = Regex::new(r"NCT[0-9]{8}").unwrap();

            if (Regex.Match(interim_id, @"NCT[0-9]{8}").Success && 
            processed_id != "NCT11111111" && processed_id != "NCT99999999" 
            && processed_id !=  "NCT12345678" && processed_id != "NCT87654321")
            {
                processed_id = Regex.Match(interim_id, @"NCT[0-9]{8}").Value;
                sec_id_source = 100120;
                sec_id_type_id = 11;
            }
        }

        else if Regex.Match(sec_id, @"[0-9]{4}-[0-9]{6}-[0-9]{2}").Success
        {
            processed_id = Regex.Match(sec_id, @"[0-9]{4}-[0-9]{6}-[0-9]{2}").Value;
            sec_id_source = 100123;
            sec_id_type_id = 11;
        }

        else if sec_id.Contains("ISRCTN")
        {
            interim_id = sec_id.Replace("ISRCTN ", "ISRCTN");
            interim_id = interim_id.Replace("(ISRCTN)", "");
            interim_id = interim_id.Replace("ISRCTN(International", "");
            interim_id = interim_id.Replace("ISRCTN: ", "ISRCTN");
            interim_id = interim_id.Replace("ISRCTNISRCTN", "ISRCTN");
            
            if (Regex.Match(interim_id, @"ISRCTN[0-9]{8}").Success)
            {
                processed_id = Regex.Match(interim_id, @"ISRCTN[0-9]{8}").Value;
                sec_id_source = 100126;
                sec_id_type_id = 11;
            }
        }

        else if Regex.Match(sec_id, @"ACTRN[0-9]{14}").Success
        {
            processed_id = Regex.Match(sec_id, @"ACTRN[0-9]{14}").Value;
            sec_id_source = 100116;
            sec_id_type_id = 11;
        }

        else if Regex.Match(sec_id, @"DRKS[0-9]{8}").Success
        {
            processed_id = Regex.Match(sec_id, @"DRKS[0-9]{8}").Value;
            sec_id_source = 100124;
            sec_id_type_id = 11;
        }

        else if Regex.Match(sec_id, @"CTRI/[0-9]{4}/[0-9]{2,3}/[0-9]{6}").Success
        {
            processed_id = Regex.Match(sec_id, @"CTRI/[0-9]{4}/[0-9]{2,3}/[0-9]{6}").Value;
            processed_id = processed_id.Replace('/', '-');  // internal representation for CTRI
            sec_id_source = 100121;
            sec_id_type_id = 11;
        }

        else if Regex.Match(sec_id, @"1111-[0-9]{4}-[0-9]{4}").Success
        {
            processed_id = "U" + Regex.Match(sec_id, @"1111-[0-9]{4}-[0-9]{4}").Value;
            sec_id_source = 100115;
            sec_id_type_id = 11;
        }

        else if Regex.Match(sec_id, @"UMIN[0-9]{9}").Success || Regex.Match(sec_id, @"UMIN-CTR[0-9]{9}").Success
        {
            processed_id = "JPRN-UMIN" + Regex.Match(sec_id, @"[0-9]{9}").Value;
            sec_id_source = 100127;
            sec_id_type_id = 11;
        }

        else if Regex.Match(sec_id, @"jRCTs[0-9]{9}").Success
        {
            processed_id = "JPRN-jRCTs" + Regex.Match(sec_id, @"[0-9]{9}").Value;
            sec_id_source = 100127;
            sec_id_type_id = 11;
        }

        else if Regex.Match(sec_id, @"jRCT[0-9]{10}").Success
        {
            processed_id = "JPRN-jRCT" + Regex.Match(sec_id, @"[0-9]{10}").Value;
            sec_id_source = 100127;
            sec_id_type_id = 11;
        }

        else if sec_id.StartsWith("JPRN")
        {
            if (Regex.Match(sec_id, @"^[0-9]{8}$").Success)
            {
                processed_id = "JPRN-UMIN" + Regex.Match(sec_id, @"[0-9]{8}").Value;
                sec_id_source = 100127;
                sec_id_type_id = 11;
            }
            else
            {
                processed_id = sec_id;
                sec_id_source = 100127;
                sec_id_type_id = 11;
            }
        }
        
        else if sec_id.StartsWith("RBR")
        {
            sec_id_source = 100117;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
        
        else if sec_id.StartsWith("ChiCTR")
        {
            sec_id_source = 100118;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }

        else if sec_id.StartsWith("ChiMCTR")
        {
            sec_id_source = 104545;   
            processed_id = sec_id;
            sec_id_type_id = 11;
        }

        else if sec_id.StartsWith("KCT")
        {
            sec_id_source = 100119;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
        
        else if sec_id.StartsWith("RPCEC")
        {
            sec_id_source = 100122;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
        
        else if sec_id.StartsWith("DRKS")
        {
            sec_id_source = 100124;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
        
        else if sec_id.StartsWith("IRCT")
        {
            sec_id_source = 100125;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
        
        else if sec_id.StartsWith("PACTR")
        {
            sec_id_source = 100128;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
        
        else if sec_id.StartsWith("PER")
        {
            sec_id_source = 100129;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
        
        else if sec_id.StartsWith("SLCTR")
        {
            sec_id_source = 100130;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
       
        else if sec_id.StartsWith("TCTR")
        {
            sec_id_source = 100131;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
        
        // Avoid Dutch CCMO numbers, which also start with NL, by regex tests
        
        else if sec_id.StartsWith("NL") && Regex.Match(sec_id, @"^NL\d{1,4}$").Success
        {
            sec_id_source = 100132;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }
        
        else if sec_id.StartsWith("NTR") && Regex.Match(sec_id, @"^NTR\d{1,4}$").Success
        {
            sec_id_source = 100132;
            processed_id = sec_id;
            sec_id_type_id = 45;      // obsolete dutch registry id
        }
        
        else if sec_id.StartsWith("LBCTR")
        {
            sec_id_source = 101989;
            processed_id = sec_id;
            sec_id_type_id = 11;
        }

        if sd_sid.StartsWith("RBR")
        {
            // Extract Brazilian ethics Ids
            
            if (Regex.Match(sec_id, @"[0-9]{8}.[0-9].[0-9]{4}.[0-9]{4}").Success)
            {
                sec_id_source = 102000;  // Brazilian regulatory authority, ANVISA
                processed_id = Regex.Match(sec_id, @"[0-9]{8}.[0-9].[0-9]{4}.[0-9]{4}").Value;
                sec_id_type_id = 41;
            }

            if (Regex.Match(sec_id, @"[0-9].[0-9]{3}.[0-9]{3}").Success)
            {
                sec_id_source = 102001;  // Brazilian ethics committee approval number
                processed_id = Regex.Match(sec_id, @"[0-9].[0-9]{3}.[0-9]{3}").Value;
                sec_id_type_id = 12;
            }
        }

        string? sec_id_type = sec_id_type_id switch
        {
            11 => "Trial Registry ID",
            45 => "Obsolete NTR number",
            41 => "Regulatory Body ID",
            12 => "Ethics Review ID",
            _ => null
        };
       
        // Return the source / processed id process if discovery successful,
        // otherwise return the original secondary id without any source.
        
        return processed_id is not null 
            ? new SecIdBase(processed_id, sec_id_source, sec_id_type_id, sec_id_type) 
            : new SecIdBase(sec_id, null, null, null) ;
    }


