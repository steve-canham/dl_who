use super::file_models::{MeddraCondition, SecIdBase, 
    SecondaryId, WhoStudyFeature};
use std::sync::LazyLock;
use regex::Regex;
use std::collections::HashSet;


pub fn get_sid_type_id(sd_sid: &String) -> i32 {
    let usid = sd_sid.to_uppercase();
    match usid {
        _ if usid.starts_with("NCT") => 120,
        _ if usid.starts_with("CHICTR") => 118,
        _ if usid.starts_with("CTRI") => 121,
        _ if usid.starts_with("JPRN") => 127,
        _ if usid.starts_with("EUCTR") => 123,
        _ if usid.starts_with("ISRCTN") => 126,
        _ if usid.starts_with("ACTRN") => 116,
        _ if usid.starts_with("DRKS") => 124,
        _ if usid.starts_with("IRCT") => 125,
        _ if usid.starts_with("KCT") =>  119,
        _ if usid.starts_with("NL-OMON") => 132,
        _ if usid.starts_with("CTIS") => 135,
        _ if usid.starts_with("RBR") => 117, 
        _ if usid.starts_with("RPCEC") => 122,
        _ if usid.starts_with("PACTR") => 128,
        _ if usid.starts_with("PER") => 129,
        _ if usid.starts_with("SLCTR") => 130,
        _ if usid.starts_with("TCTR") => 131,
        _ if usid.starts_with("LBCTR") => 133,
        _ if usid.starts_with("ITMCTR") => 134,
        _ => 0
    }
}


pub fn get_db_name (sid_type_id: i32) -> String {
    let db_name = match sid_type_id {
        120 => "ctg",
        116 => "anzctr",
        117 => "rebec",
        118 => "chictr",
        119 => "cris",
        121 => "ctri",
        122 => "rpcec",
        123 => "euctr",
        124 => "drks",
        125 => "irct",
        126 => "isrctn",
        127 => "jprn",
        128 => "pactr",
        129 => "rpuec",
        130 => "slctr",
        131 => "thctr",
        132 => "nntr",
        135 => "ctis",
        133 => "lebctr",
        134 => "itmctr",
        _ => ""
    };
    db_name.to_string()
}


pub fn split_by_year (sid_type_id: i32) -> bool {
    
    match sid_type_id {
        116 => true,
        117 => false,
        118 => true,
        119 => false,
        121 => true,
        122 => false,
        123 => true,
        124 => true,
        125 => true,
        127 => true,
        128 => false,
        129 => false,
        130 => false,
        131 => false,
        132 => true,
        135 => true,
        133 => false,
        134 => false,
        _ => false
    }

}


pub fn get_type(study_type: &String) -> i32 {

    let t = study_type.to_lowercase();
    if t.starts_with("intervention") || t == "ba/be"
    {
        11
    }
    else if t.starts_with("observation")
            || t.starts_with("epidem")
            || t == "pms"
            || t == "relative factors research"
            || t == "cause"
            || t == "cause/relative factors study"
            || t == "health services research"
            || t == "health services reaserch"
    {
        12
    }
    else if t == "patient registry" || t == "observational patient registry"  
    || t == "observational [patient registry]"
    {
        13
    }
    else if t == "expanded access"
    {
        14
    }
    else if t == "funded programme"
    {
        15
    }
    else if t == "diagnostic test"
    {
        16
    }
    else if t == "not applicable" || t == "n/a" {
        98
    }
    else if t == "not specified" || t == "unknown" || t == "not provided" {
        0
    }
    else if t  ==  "other" 
            || t  == "others,meta-analysis etc" 
            || t  == "basic science"
            || t  == "prevention"
            || t  == "screening"
            || t == "treatment study"
    {
        99
    }
    else {
        999
    }
}


pub fn get_status(status: &String, sid_type_id: i32) -> i32 {

    // to clarify, need to check exact meaning of terms as used by CGT and EMA in particuolar
    // to distinguish whether terms apply to recruitment or to the study as a whole
    // raise the issue of categorising the terms according to the source id...

    // For CTG (and ANZCTR) 'Completed' means the whole study is completed (as opposed to 'active, not recruiting')
    // For China and NNTR, not clear odf the meaning of 'Completed' - need tro investigate...
    // For EUDRACXT and CTIS ???
  
    let s = status.to_lowercase();
    if (s == "completed" && (sid_type_id == 120 || sid_type_id == 116)) 
        || s == "complete: follow-up complete" || s == "complete: follow up complete" 
        || s == "data analysis completed" || s == "main results already published"
        || s == "approved for marketing"
    {
        30    // Study Completed
    }
    else if s == "complete" || s == "completed" || s == "complete: follow-up continuing" 
        || s == "complete: follow up continuing" || s == "active, not recruiting" 
        || s == "closed to recruitment of participants" || s == "no longer recruiting" 
        || s == "not recruiting" || s == "recruitment completed"
        || s == "enrollment closed"
        || s == "recruiting stopped after recruiting started"
    {
        25    //  No longer recruiting - may be ongoing as a study
    }
    else if s == "recruiting"  || s =="open public recruiting" 
    || s == "open to recruitment" || s =="in enrollment"
    {
        15  // Recruiting
    }
    else if s.contains("pending")
        || s == "not yet recruiting"
        || s == "without startig enrollment"
        || s == "preinitiation"
    {
        10   // Not yet recruiting
    }
    else if s.contains("suspended") || s.contains("temporarily closed")
        || s == "temporary halt"
    {
        19      // Suspended
    }
    else if s.contains("terminated") || s.contains("stopped early")
        || s == "stopped"
    {
        28      // Terminated
    }
    else if s.contains("withdrawn")
    {
        12      // Withdrawn
    }
    else if s.contains("enrolling by invitation")
    {
        16      // Enrolling by invitation
    }
    else if s == "authorised-recruitment may be ongoing or finished"  || s == "available" || s == "ongoing"
    {
        22      // Ongoing, recruitment status unclear
    }
    else if s == "not applicable" || s== "na" || s== "n/a"{
        98      // Recorded as not applicable
    }
    else if s == "withheld" || s == "unknown" || s == "no longer available"
        || s == "deleted from source registry" || s == "unknown status"
        || s == "temporarily not available" {
            0       // Not provided
    }
    else {
        99   // Other
    }
}
         

pub fn get_conditions(condition_list: &String, sid_type_id: i32) -> (Option<Vec<String>>, Option<Vec<MeddraCondition>>) {

    // Replace line breaks and hashes with semi-colons, then split

    let mut clist = condition_list.replace("<br>", ";").replace("<br/>", ";");
    clist = clist.replace("#", ";");
    let sep_conds: Vec<&str> = clist.split(";").collect();
    
    // Set up vectors to receive possible conditions / meddra conditions

    let mut conds = Vec::<String>::new();
    let mut medra_conds = Vec::<MeddraCondition>::new();

    for s in sep_conds
    {
        let complex_trim = |c| c == ' ' || c == '('|| c == '.' || c == ';' || c == '-';
        let s1 = s.trim_matches(complex_trim);
        if s1 != "" && s1.len() >= 3
        {
            // For EMA data check for a structured MedDRA entry

            if sid_type_id == 123 || sid_type_id == 135 {

                // Of type (but without line breaks): 
                // MedDRA version: 20.0  // Level: PT  // Classification code 10005003  // Term: Bladder cancer
                // System Organ Class: 10029104 - Neoplasms benign, malignant and unspecified (incl cysts and polyps)",
                // MedDRA version: 21.1  //Level: LLT  //Classification code 10022877  //Term: Invasive bladder cancer
                // System Organ Class: 10029104 - Neoplasms benign, malignant and unspecified (incl cysts and polyps)",

                static RE_V: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"MedDRA version: (?<v>.+)Level").unwrap());
                static RE_LEV: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Level: (?<level>.+)Classific").unwrap());
                static RE_CLASS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Classification code (?<code>[0-9]+)").unwrap());
                static RE_TERM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Term: (?<term>.+)System").unwrap());
                static RE_SOCCODE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"System Organ Class: (?<soccode>[0-9]+)").unwrap());
                static RE_SOC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"System Organ Class: (.+) - (?<socterm>.+)$").unwrap());

                let version = match RE_V.captures(&s1)
                {
                    Some(c) => c["v"].to_string(),
                    None => "".to_string(),
                };

                let level = match RE_LEV.captures(&s1)
                {
                    Some(c) => c["level"].to_string(),
                    None => "".to_string(),
                };

                let code = match RE_CLASS.captures(&s1)
                {
                    Some(c) => c["code"].to_string(),
                    None => "".to_string(),
                };
                
                let term = match RE_TERM.captures(&s1)
                {
                    Some(c) => c["term"].to_string(),
                    None => "".to_string(),
                };

                let soc_code = match RE_SOCCODE.captures(&s1)
                {
                    Some(c) => c["soccode"].to_string(),
                    None => "".to_string(),
                };

                let soc_term = match RE_SOC.captures(&s1)
                {
                    Some(c) => c["socterm"].to_string(),
                    None => "".to_string(),
                };
                
                let mc = MeddraCondition::new(version, level, code, term, soc_code, soc_term);
                medra_conds.push(mc);
            }
            else {

                // Simple condition - push strring to vector.

                conds.push(s1.to_string());
            }

            // Most processing code for condition data now all moved to Harvester
            // module, as it is easier to correct and extend there (changes
            // do not require WHO re-download!).
            // Conditions exported from here as a simple string array.
        }
    }

    // Convert vectors to Option<vector>

    let conditions_option = match conds.len() {
            0 => None,
            _ => {  
                    if conds.len() > 1 {       // May need to de-duplicate
                        
                        let mut uniques = HashSet::new();
                        conds.retain(|e| uniques.insert(e.clone()));
                    }
                    Some(conds)
                },
    };

    let meddraconds_option: Option<Vec<MeddraCondition>> = match medra_conds.len() {
            0 => None,
            _ => {         
                    let mut revised_list: Vec<MeddraCondition> = Vec::new();
                    if medra_conds.len() == 1 {
                        revised_list = medra_conds;
                    }
                    else {          // May need to de-duplicate
                        
                        let mut uniques:HashSet<String> = HashSet::new();
                        for mc in medra_conds {
                            if uniques.insert(mc.term.clone()) {
                                revised_list.push(mc);
                            }
                        }
                    }
                    Some(revised_list)
                }
                  
    };

    (conditions_option, meddraconds_option)    

}


pub fn split_and_dedup_countries(sid_type_id: i32, country_list: &String) -> Option<Vec<String>> {

    // country list known to be non-null and already 'tidied'.

    let in_strings: Vec<&str> = country_list.split(';').collect();
    let mut out_strings = Vec::<String>::new();
   
    for c in in_strings
    {
        // Sri Lankan registry (in particular) uses commas to list countries
        // but commas appear legitimately in many versions of country names
        
        let mut this_c = c.trim().to_lowercase().replace(".", "");
        let mut this_c_consumed = false;

        if sid_type_id == 127 {// Some odd 'regional' countries used by the Japanese registries
            this_c = this_c.replace("asia except japan", "asia");
            this_c = this_c.replace("asia exept japan", "asia");
            this_c = this_c.replace("japan,asia(except japan)", "asia");
            this_c = this_c.replace("asia(except japan)", "asia");
            this_c = this_c.replace("none (japan only)", "japan");
            this_c = this_c.replace("none other than japan", "japan");
        }

        if this_c.contains(',') {

            // Sri Lankan registry (in particular) uses commas to list countries
            // but commas appear legitimately in many versions of country names

            let complex_trim = |c| c == ',';    // remove ending commas and double spaces
            this_c = this_c.trim_matches(complex_trim).replace("  ", " ");

            this_c = this_c.replace("aiwan, prov", "aiwan - prov");
            this_c = this_c.replace("aiwan, tai", "aiwan - tai");
            this_c = this_c.replace("congo,", "congo - ");
            this_c = this_c.replace("iran,", "iran - ");
            this_c = this_c.replace("sar,", "sar - ");
            this_c = this_c.replace("kong, chi", "kong - chi");
            this_c = this_c.replace("korea, dem", "korea - dem");
            this_c = this_c.replace("korea, nor", "korea - nor");
            this_c = this_c.replace("korea, sou", "korea - sou");
            this_c = this_c.replace("korea, rep", "korea - rep");
            this_c = this_c.replace("macedonia,", "macedonia - ");
            this_c = this_c.replace("moldova,", "moldova - ");
            this_c = this_c.replace("palastine,", "palastine - ");
            this_c = this_c.replace("palastinian,", "palastinian - ");
            this_c = this_c.replace("tanzania,", "tanzania - ");
            this_c = this_c.replace("islands,", "islands - ");

            if this_c.contains(',') {
                let added_strings: Vec<&str> = this_c.split(',').collect();
                for ac in added_strings {
                    let act = ac.trim();
                    if add_country_name(act, &out_strings) {
                        out_strings.push(act.to_string());
                    }
                }
                this_c_consumed = true;   // countries have been added from this string
            }
        }

        if !this_c_consumed && add_country_name(&this_c, &out_strings) {
            out_strings.push(this_c.to_string());
        }
    }

    return Some(out_strings);
}


fn add_country_name(new_name:&str, out_strings: &Vec::<String>) -> bool {

    if out_strings.len() == 0
    {
        true
    }
    else {
        let mut add_string = true;
        for s in out_strings
        {
            if s == new_name
            {
                add_string = false;
                break;
            }
        }
        add_string 
    }
}


pub fn add_eu_design_features(design: &String) -> Vec<WhoStudyFeature> {
    let mut fs = Vec::<WhoStudyFeature>::new();
    
    // design list in forms such as (without line breaks)
    // "Controlled: yes Randomised: yes Open: no Single blind: no Double blind: yes Parallel group: no 
    //Cross over: no Other: no 
    //If controlled, specify comparator, Other Medicinial Product: no Placebo: yes 
    // Other: no Number of treatment arms in the trial: 3",
    
    //Controlled: yes Randomised: yes Open: no Single blind: no Double blind: yes 
    //Parallel group: yes Cross over: no Other: yes 
    //Other trial design description: 2 part of the study - first double-blind, second part open label 
    //If controlled, specify comparator, 
    //Other Medicinial Product: no Placebo: yes Other: no Number of treatment arms in the trial: 2"
    
    if design.contains("randomised: yes") {
        fs.push(WhoStudyFeature::new(22, "Allocation type", 205, "Randomised"));
    }
    if design.contains("parallel group: yes") {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 305, "Parallel assignment"));
    }
    if design.contains("cross over: yes") {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 310, "Crossover assignment"));
    }
    if design.contains("open: yes") {
        fs.push(WhoStudyFeature::new(24, "Masking", 500, "None (Open Label)"));
    }
    if design.contains("single blind: yes") {
        fs.push(WhoStudyFeature::new(24, "Masking", 505, "Single"));
    }
    if design.contains("double blind: yes") {
        fs.push(WhoStudyFeature::new(24, "Masking", 510, "Double"));
    }

    fs
}


pub fn add_study_purpose(design_list: &String) -> Option<WhoStudyFeature> {
   
    let design = design_list.replace(" :", ":"); // to make comparisons easier
    let purpose: &str ;
    let code: usize;

    if design.contains("purpose: treatment") || design.ends_with(", treatment")
    {
        purpose = "Treatment";  code = 400;
    }
    else if design.contains("purpose: diagnosis") || design.contains("diagnostic")
    {
        purpose = "Diagnostic";  code = 410;
    }   
    else if design.contains("supportive care") || design.contains("purpose: supportive")
    {
        purpose = "Supportive care";    code = 415;
    }
    else if design.contains("screening")
    {   
        purpose = "Screening";     code = 420;
    }
    else if design.contains("prevention") || design.contains("preventative") 
    {
        purpose = "Prevention";    code = 405;
    }
    else if design.contains("basic science") || design.contains("natural history") 
    {   
        purpose = "Basic Science";    code = 430;
    }
    else if design.contains("health services") 
    {   
        purpose = "Health services research";   code = 425;
    }
    else if design.contains("device") 
    {   
        purpose = "Device feasibility";    code = 435;
    }
    else if design.contains("education") ||  design.contains("counselling") || design.contains("training") 
    {    
        purpose = "Educational / counselling / training";     code = 450;
    }
    else {
        purpose = "Not provided";   code = 0;
    }

    if purpose != "Not provided" {
        Some(WhoStudyFeature::new(21, "Primary purpose", code, purpose))
    }
    else {
        None
    }
     
}


pub fn add_int_study_features(design_list: &String) -> Vec<WhoStudyFeature> {
    let mut fs = Vec::<WhoStudyFeature>::new();
   
    if design_list.contains("non-randomized")   
     || design_list.contains("nonrandomized")
     || design_list.contains("non-randomised")
     || design_list.contains("nonrandomised")
     || design_list.contains("non-rct")
    {
        fs.push(WhoStudyFeature::new(22, "Allocation type", 210, "Nonrandomised"));
    }
    else if design_list.contains("randomized")
         || design_list.contains("randomised")
         || design_list.contains(" rct")
    {
        fs.push(WhoStudyFeature::new(22, "Allocation type", 205, "Randomised"));
    }

    if design_list.contains("parallel")
    {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 305, "Parallel assignment"));
    }

    if design_list.contains("crossover")
    {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 310, "Crossover assignment"));
    }

    if design_list.contains("factorial")
    {
        fs.push(WhoStudyFeature::new(23, "Intervention model", 315, "Factorial assignment"));
    }

    fs
}


pub fn add_obs_study_features(design: &String) -> Vec<WhoStudyFeature> {
    
   // "Purpose: Screening;Duration: Longitudinal;Selection: Defined population;Timing: Prospective"
    let mut fs = Vec::<WhoStudyFeature>::new();
    
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

    if design.contains("not collect nor archive")
    {
        fs.push(WhoStudyFeature::new(32, "Biospecimens retained", 800, "None retained"));
    }
    if design.contains("collect & archive- sample with dns")
    {
        fs.push(WhoStudyFeature::new(32, "Biospecimens retained", 805, "Samples with DNA"));
    }

    fs
}


pub fn add_masking(design_list: &String) -> Option<WhoStudyFeature> {

    let design = design_list.replace(" :", ":"); // to make comparisons easier
    let masking: &str ;
    let code: usize;

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
        masking = "None (Open Label";
        code = 500;
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
        masking = "Single";
        code = 505;
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
        masking = "Double";
        code = 510;
    }
    else if design.contains("triple blind")
     || design.contains("triple-blind")
     || design.contains("blinded (patient/subject, caregiver, investigator/therapist, assessor")
     || design.contains("masking:participant, investigator, outcome assessor")
    {
        masking = "Triple";
        code = 515;
    }
    else if design.contains("quadruple blind") || design.contains("quadruple-blind")
    {
        masking = "Quadruple";
        code = 520;
    }
    else if design.contains("masking used") || design.contains("blinding used")
    {
        masking = "Blinded (no details)";
        code = 502;
    }
    else if design.contains("masking:not applicable")
     || design.contains("blinding:not applicable")
     || design.contains("masking not applicable")
     || design.contains("blinding not applicable")
    {
        masking = "Not applicable";
        code = 599;
    }
    else if design.contains("masking: unknown")
    {
        masking = "Not provided";
        code = 525;
    }
    else {
        masking = "Not provided";
        code = 525;
    }

    if masking != "Not provided" {
        Some(WhoStudyFeature::new(24, "Masking", code, masking))
    }
    else {
        None
    }

}


pub fn add_eu_phase(phase_list: &String) -> Option<WhoStudyFeature> {
    
    // phase string in the form
    // (Eudract) "Human pharmacology (Phase I): noTherapeutic exploratory (Phase II): yesTherapeutic confirmatory - (Phase III): noTherapeutic use (Phase IV): no"
    // (CTIS) "phase_string": "Human pharmacology (Phase I): No Therapeutic exploratory (Phase II): No Therapeutic confirmatory - (Phase III): Yes Therapeutic use - (Phase IV): No",
    // split on the colon

    let mut p1 = false;
    let mut p2 = false;
    let mut p3 = false;
    let mut p4 = false;

    let ps: Vec<&str> = phase_list.split(':').into_iter().collect();

    // Presence of each phase signalled by the ebginning of the following text.

    if ps[1].trim().starts_with("yes") {
        p1 = true;
    }
    if ps[2].trim().starts_with("yes") {
        p2 = true;
    }
    if ps[3].trim().starts_with("yes") {
        p3 = true;
    }
    if ps[4].trim().starts_with("yes") {
        p4 = true;
    }

    let phase: &str;
    let code: usize;
    
    if p1 && p2 {
        phase = "Phase 1/Phase 2";
        code = 115;
    }
    else if p2 && p3 {
        phase = "Phase 2/Phase 3";
        code = 125;
    }
    else if p1 {
        phase = "Phase 1";
        code = 110;
    }
    else if p2 {
        phase = "Phase 2";
        code = 120;
    }
    else if p3 {
        phase = "Phase 3";
        code = 130;
    }
    else if p4 {
        phase = "Phase 4";
        code = 135;
    }
    else {
        phase = "Not provided";
        code = 0;
    }

    if phase != "Not provided" {
        Some(WhoStudyFeature::new(20, "Phase", code, phase))
    }
    else {
        None
    }
}


pub fn add_phase(phase: &String) -> Option<WhoStudyFeature> {
    
    let ph: &str;
    let code: usize;
    
    if phase == "phase 0" || phase == "phase-0" || phase == "phase0" 
    || phase ==  "0" || phase ==  "0 (exploratory trials)" 
    || phase == "phase 0 (exploratory trials)" || phase ==  "0 (exploratory trials)"
    {
        ph = "Early phase 1";
        code = 105;
    }
    else if phase == "1" || phase ==  "i" || phase ==  "i (phase i study)" 
                || phase == "phase-1" || phase ==  "phase 1" || phase ==  "phase i" || phase ==  "phase1"
    {
        ph = "Phase 1";
        code = 110;
    }
    else if phase == "1-2" || phase ==  "1 to 2" || phase ==  "i-ii" 
    || phase ==  "i+ii (phase i+phase ii)" || phase ==  "phase 1-2" 
    || phase ==  "phase 1 / phase 2" || phase ==  "phase 1/ phase 2" 
    || phase == "phase 1/phase 2" || phase ==  "phase i,ii" || phase == "phase1/phase2"
    {
        ph = "Phase 1/Phase 2";
        code = 115;
    }
    else if phase == "2" || phase ==  "2a" || phase ==  "2b" 
    || phase ==  "ii" || phase ==  "ii (phase ii study)" || phase ==  "iia" 
    || phase ==  "iib" || phase ==  "phase-2" || phase ==  "phase 2" || phase ==  "phase ii" || phase ==  "phase2"
    {
        ph = "Phase 2";
        code = 120;
    }
    else if phase == "2-3" || phase == "ii-iii" || phase ==  "phase 2-3" 
    || phase == "phase 2 / phase 3" || phase == "phase 2/ phase 3" 
    || phase ==  "phase 2/phase 3" || phase == "phase2/phase3" || phase == "phase ii,iii"
    {
        ph = "Phase 2/Phase 3";
        code = 125;
    }
    else if phase == "3" || phase ==  "iii" || phase ==  "iii (phase iii study)" 
    || phase ==  "iiia" || phase ==  "iiib" || phase ==  "3-4" || phase ==  "phase-3" 
    || phase ==  "phase 3" || phase ==  "phase 3 / phase 4" 
    || phase ==  "phase 3/ phase 4" || phase ==  "phase3" || phase ==  "phase iii"
    {
        ph = "Phase 3";
        code = 130;
    }
    else if phase == "4" || phase ==  "iv" || phase ==  "iv (phase iv study)" 
                || phase == "phase-4" || phase ==  "phase 4" || phase ==  "post-market" 
                || phase ==  "post marketing surveillance" || phase ==  "phase4" || phase ==  "phase iv"
    {
        ph = "Phase 4";
        code = 135;
    }
    else if phase == "not selected" || phase == "not applicable"
            || phase == "na" || phase == "n/a" {
        ph = "Not provided";
        code = 0;
    }
    else
    {
        ph = "Not provided";
        code = 0;
    }

    if ph != "Not provided" {
        Some(WhoStudyFeature::new(20, "Phase", code, ph))
    }
    else {
        None
    }
}


pub fn split_secids (ids: &Option<Vec<SecondaryId>>) -> (Option<Vec<String>>, Option<Vec<String>>) {
    
    let mut reg_ids = Vec::<String>::new();
    let mut oth_ids = Vec::<String>::new();

    match ids {
        Some(sids) => {
            if sids.len() > 0 {
                for secid in sids {
                   if secid.sec_id_type_id < 990 {
                       reg_ids.push(format!("{}::{}", secid.sec_id_type_id, secid.processed_id))
                   }
                   else {
                       oth_ids.push(secid.sec_id.clone())
                   }
                }

                let reg_sec_ids = match reg_ids.len() {
                    0 => None,
                  _ => Some(reg_ids)
                };
                let oth_sec_ids = match oth_ids.len() {
                    0 => None,
                    _ => Some(oth_ids)
                };
                (reg_sec_ids, oth_sec_ids)
           }
           else {
            (None, None)
           }
        },
        None => (None, None),
    }
}


pub fn process_sponsor_name(sponsor: &Option<String>) -> Option<String> {

    // put to lower case
    match sponsor {
        Some(s) => {
            let mut s2 = s.trim().to_lowercase();

            s2 = s2.replace(".", "");
            s2 = s2.replace(",", "");
            s2 = s2.replace("'", "");    
            s2 = s2.replace("’", "");  

            s2 = s2.replace(" of ", " ");
            s2 = s2.replace(" de ", " ");
            s2 = s2.replace(" dat ", " ");
            s2 = s2.replace(" et ", " ");
            s2 = s2.replace(" y ", " ");
            s2 = s2.replace(" & ", " ");
            s2 = s2.replace(" for ", " ");
            
            static RE_SP1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^the ").unwrap());
            static RE_SP2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" co ltd$").unwrap());
            static RE_SP3: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" ltd$").unwrap());
            static RE_SP4: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" llc$").unwrap());
            static RE_SP5: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" inc$").unwrap());
            static RE_SP6: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" spa$").unwrap());
            static RE_SP7: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" ab$").unwrap());
            static RE_SP8: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" ag$").unwrap());
            static RE_SP9: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" pty$").unwrap());
            static RE_SP10: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" gmbh$").unwrap());
            static RE_SP11: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" sa$").unwrap());
            static RE_SP12: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" a/s$").unwrap());
            static RE_SP13: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" corporation$").unwrap());
            static RE_SP14: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" spoo$").unwrap());

            // at beginning...

            s2 = RE_SP1.replace(&s2, "").to_string();
            
            // at end

            s2 = RE_SP2.replace(&s2, "").to_string();
            s2 = RE_SP3.replace(&s2, "").to_string();  
            s2 = RE_SP4.replace(&s2, "").to_string();
            s2 = RE_SP5.replace(&s2, "").to_string();
            s2 = RE_SP6.replace(&s2, "").to_string();
            s2 = RE_SP7.replace(&s2, "").to_string();
            s2 = RE_SP8.replace(&s2, "").to_string();
            s2 = RE_SP9.replace(&s2, "").to_string();
            s2 = RE_SP10.replace(&s2, "").to_string();  
            s2 = RE_SP11.replace(&s2, "").to_string();
            s2 = RE_SP12.replace(&s2, "").to_string();
            s2 = RE_SP13.replace(&s2, "").to_string();
            s2 = RE_SP14.replace(&s2, "").to_string();

            Some(s2)
        },
        None => None,
    }

}


pub fn  split_ids(sd_sid: &String, in_string: &String, source_field: &str) -> Vec<SecondaryId> {
        
    // in_string already known to be non-null, non-empty.
    // most common way of splitting a list of Ids is on a semi-colon.

    let ids: Vec<&str> = in_string.split(";").collect();
    static RE_NTRB: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" \(NTR\d{2}").unwrap());

    let mut revised_ids: Vec<String> = Vec::new();
    for s in ids.clone()  // Usually only 1, 2 or 3 's' entries
    {
        let mut this_s = s.to_string();

        // Where sec_id matches ' \(NTR\d{2}' replace ' (NTR' with ', NTR).

        if s.contains("(NTR") && RE_NTRB.is_match(s) {
            this_s = s.replace(" (NTR", ", NTR").replace(")", "");
        }

        if this_s.contains(", ") {

            // terms may contain multiple ids separated by 1 or more commas, 
            // with the order of id types varying and inconsistent

            if this_s.contains(", NTR") {
                 this_s = this_s.replace(", NTR", "||NTR");
            }
            if this_s.contains(", NL") { 
                 this_s = this_s.replace(", NL", "||NL");
            }
            if this_s.contains(", NIHR") { 
                 this_s = this_s.replace(", NIHR", "||NIHR");
            }
            if this_s.contains(", CPMS") {
                this_s = this_s.replace(", CPMS", "||CPMS");
            }
            if this_s.contains(", IRAS") {
                 this_s = this_s.replace(", IRAS", "||IRAS");
            }

            // Split on the pipe to form separate portions of the string
            let split_ids: Vec<&str> = this_s.split("||").collect();
            let mut new_ids: Vec<String> = split_ids.iter().map(|&s| s.to_string()).collect();

            // add the set (may be the original if no '||') to the revised_ids
            revised_ids.append(&mut new_ids);

        }
        else {
            revised_ids.push(this_s);
        }

    }

    // end up with id list extended if necessary

    let mut id_list = Vec::<SecondaryId>::new();
    for s in revised_ids
    {
        let chars_to_trim: &[char] = &[' ', '\'', ':', '#', ';', '.', '‘', '’'];
        let secid = s.trim_matches(chars_to_trim);

        if secid != sd_sid {

            let secid_low = secid.to_lowercase();

            // Check first it is a possible id

            if !secid_low.starts_with("none") && !secid_low.starts_with("nil")
                && !secid_low.starts_with("not ") && !secid_low.starts_with("date")
                && !secid_low.starts_with("??") 
            {
                // To be worth processing and looking for has to include at least 1 number and be of reasonable length.

                if secid_low.chars().any(|c| c.is_digit(10)) && secid.len() >= 4 {

                    let sec_id_base = get_sec_id_details(secid);
                    
                    // Is the id the same as the sid? (With EUCTR may be, 
                    // because it is simply anoher country code variation)
                    // Also check not 0 (as may be if the is is nonsensical)

                    if sec_id_base.processed_id != sd_sid.to_string() && sec_id_base.sec_id_type_id != 0
                    {
                        id_list.push(SecondaryId::new_from_base(source_field.to_string(), 
                                                    secid.to_string(), sec_id_base));
                    }
                }
                else {    // very small sec_id

                    let sec_id_base = SecIdBase{
                        processed_id: secid.to_string(),
                        sec_id_type_id: 990,
                    };

                    id_list.push(SecondaryId::new_from_base(source_field.to_string(), 
                                                        secid.to_string(), sec_id_base));
                }
            }
        }
    }

    id_list  // may overlap ids with the ids from the other source fields

}


pub fn get_sec_id_details(sec_id: &str) -> SecIdBase {

    if let Some(id) = contains_nct(sec_id) { id } else 
    if let Some(id) = contains_euctr(sec_id) {id } else 
    if let Some(id) = contains_isrctn(sec_id) { id } else 
    if let Some(id) = contains_actrn(sec_id) {id } else 
    if let Some(id) = contains_drks(sec_id) {id } else 
    if let Some(id) = contains_ctri(sec_id) { id } else 
    if let Some(id) = contains_who(sec_id) {id } else 
    if let Some(id) = contains_umin(sec_id) { id } else 
    if let Some(id) = contains_jcrt(sec_id) {id } else 
    if let Some(id) = contains_japic(sec_id) { id } else 
    if let Some(id) = contains_jma(sec_id) {id } else 
    if let Some(id) = contains_jprn(sec_id) {id } else 
    if let Some(id) = contains_nl(sec_id) { id } else 
    if let Some(id) = contains_ntr(sec_id) {id } else 
    if let Some(id) = contains_rpuec(sec_id) {id } 
    else 
    {
        let upid = sec_id.to_uppercase();
        let sec_id_type_id = match upid {
        _ if upid.starts_with("CHICTR") => 118,
        _ if upid.starts_with("IRCT") => 125,
        _ if upid.starts_with("KCT") =>  119,
        _ if upid.starts_with("RBR") => 117, 
        _ if upid.starts_with("RPCEC") => 122,
        _ if upid.starts_with("PACTR") => 128,
        _ if upid.starts_with("SLCTR") => 130,
        _ if upid.starts_with("TCTR") => 131,
        _ if upid.starts_with("LBCTR") => 133,
        _ if upid.starts_with("ITMCTR") => 134,
        _ if upid.starts_with("CHIMCTR") => 134,
        _ => 0
        };

        if sec_id_type_id > 0 {
            SecIdBase{
                processed_id: sec_id.to_string(),
                sec_id_type_id: sec_id_type_id,
            }
        } else {

            // Return the original secondary id without any identified type.

            SecIdBase{
                processed_id: sec_id.to_string(),
                sec_id_type_id: 990,
            }
        }
    }
}


    
// Collection of regex check functions, where the regex is stored
// in a static lazy lock variable - i.e. it needs instantiating only once.

fn contains_nct(hay: &str) -> Option<SecIdBase> {
    if hay.contains("NCT") {
        let hay2 = hay.replace("NCT ", "NCT").replace("NCTNumber", "");
        static RE_NCT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"NCT[0-9]{8}").unwrap());
        match RE_NCT.captures(&hay2) {
            Some(s) => {
            let id = &s[0];
            if id == "NCT11111111" || id == "NCT99999999" 
            || id == "NCT12345678" || id == "NCT87654321" 
            || id == "NCT00000000" {
                Some(SecIdBase{
                    processed_id: "0".to_string(),
                    sec_id_type_id: 0,
                })
            }
            else {
                Some(SecIdBase{
                    processed_id: id.to_string(),
                    sec_id_type_id: 120,
                })
            }
        },
            None => None,
        }  
    }
    else {
        None 
    }
}

fn contains_euctr(hay: &str) -> Option<SecIdBase> {
    static RE_EUCTR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"20[0-9]{2}-[0-9]{6}-[0-9]{2}").unwrap());
    static RE_CTIS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"20[2-9][0-9]-5[0-9]{5}-[0-9]{2}").unwrap());
    match RE_EUCTR.captures(hay) {
        Some(s) => {
            match RE_CTIS.captures(hay) {
                Some (s1) => {
                    let id = &s1[0];
                    let processed_id = format!("CTIS{}", &id[0..14]);
                    Some(SecIdBase{
                        processed_id: processed_id.to_string(),
                        sec_id_type_id: 135,
                    })
                },
                None => {   // assumed EUCTR
                    let id = &s[0];
                    let processed_id = format!("EUCTR{}", &id[0..14]);
                    Some(SecIdBase{
                        processed_id: processed_id.to_string(),
                        sec_id_type_id: 123,
                    })
                },
            }
            },
        None => None,  // no eu match at all
    }
}

fn contains_isrctn(hay: &str) -> Option<SecIdBase> {
    if hay.contains("ISRCTN") {
        let mut hay2 = hay.replace("ISRCTN ", "ISRCTN").replace("(ISRCTN)", "");
        hay2 = hay2.replace("ISRCTN(International", "ISRCTN");
        hay2 = hay2.replace("ISRCTN: ", "ISRCTN");
        hay2 = hay2.replace("ISRCTNISRCTN", "ISRCTN");
        static RE_ISRCTN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"ISRCTN[0-9]{8}").unwrap());
        match RE_ISRCTN.captures(&hay2) {
            Some(s) => {
                let id = &s[0];
                if id == "ISRCTN00000000" {
                    Some(SecIdBase{
                        processed_id: "0".to_string(),
                        sec_id_type_id: 0,
                    })
                }
                else {
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_type_id: 126,
                    })
                }
            },
            None => None,
        }
    }
    else {
        None
    }
}

fn contains_actrn(hay: &str) -> Option<SecIdBase> {
    if hay.contains("ACTRN") {
        static RE_ACTRN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"ACTRN[0-9]{14}").unwrap());
        match RE_ACTRN.captures(hay) {
            Some(s) => {
                let id = &s[0];
                Some(SecIdBase{
                    processed_id: id.to_string(),
                    sec_id_type_id: 116,
                })
            },
            None => None,
        }
    }
    else {
        None
    }
}

fn contains_drks(hay: &str) -> Option<SecIdBase> {
    if hay.contains("DRKS") {
        static RE_DRKS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"DRKS[0-9]{8}").unwrap());
        match RE_DRKS.captures(hay) {
            Some(s) => {
                let id = &s[0];
                Some(SecIdBase{
                    processed_id: id.to_string(),
                    sec_id_type_id: 124,
                })
            },
            None => None,
        }
    }
    else {
        None
    }
}

fn contains_ctri(hay: &str) -> Option<SecIdBase> {
    if hay.contains("CTRI") {
        static RE_CTRI: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"CTRI/[0-9]{4}/[0-9]{2,3}/[0-9]{6}").unwrap());
        match RE_CTRI.captures(hay) {
            Some(s) => {
                let id = &s[0];
                let processed_id = id.replace("/", "-");  // internal representation for CTRI
                Some(SecIdBase{
                    processed_id: processed_id,
                    sec_id_type_id: 121,
                })
            },
            None => None,
        }
    }
    else {
        None
    }
}

fn contains_who(hay: &str) -> Option<SecIdBase> {
    static RE_WHO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"1111-[0-9]{4}-[0-9]{4}").unwrap());
    match RE_WHO.captures(hay) {
        Some(s) => {
            let id = &s[0];
            let processed_id = format!("U{}", id);  // internal representation for CTRI
            Some(SecIdBase{
                processed_id: processed_id,
                sec_id_type_id: 115,
            })
        },
        None => None,
    }
}

fn contains_umin(hay: &str) -> Option<SecIdBase> {
    if hay.contains("UMIN") {
        static RE_UMIN1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"UMIN[0-9]{9}").unwrap());
        static RE_UMIN2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"UMIN-CTR[0-9]{9}").unwrap());
        static RE_UMIN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{9}").unwrap());
        if RE_UMIN1.is_match(hay) || RE_UMIN2.is_match(hay) {
            match RE_UMIN.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    let processed_id = format!("JPRN-UMIN{}", id);  
                    Some(SecIdBase{
                        processed_id: processed_id,
                        sec_id_type_id: 141,
                    })
                },
                None => None,
            }
        }
        else {
            None
        }
    }
    else {
        None
    }
}

fn contains_jcrt(hay: &str) -> Option<SecIdBase> {
    if hay.contains("jRCT") {
        static RE_JRCT1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"jRCTs[0-9]{9}").unwrap());
        static RE_JRCT2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"jRCT[0-9]{10}").unwrap());
        match RE_JRCT1.captures(hay) {
            Some(s1) => {
                let id = &s1[0];
                let processed_id = format!("JPRN-{}", id);  
                Some(SecIdBase{
                    processed_id: processed_id,
                    sec_id_type_id: 140,
                })
            },

            None => {
                match RE_JRCT2.captures(hay) {
                    Some(s2) => {
                        let id = &s2[0];
                        let processed_id = format!("JPRN-{}", id); 
                        Some(SecIdBase{
                            processed_id: processed_id,
                            sec_id_type_id: 140,
                        })
                    },
                    None => None,
                }
            }
        }
    }
    else {
        None
    }
}


fn contains_japic(hay: &str) -> Option<SecIdBase> {
    if hay.starts_with("Japic") {
        static RE_JAPIC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^JapicCTI-\d{6}$").unwrap());
        match RE_JAPIC.captures(hay) {
            Some(s) => {
                let id = &s[0];
                Some(SecIdBase{
                    processed_id: id.to_string(),
                    sec_id_type_id: 139,
                })
            },
            None => None,
        }
    }
    else {
        None
    }
}


fn contains_jma(hay: &str) -> Option<SecIdBase> {
    if hay.starts_with("JMA") {
        static RE_JMA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^JMA-IIA\d{5}$").unwrap());
        match RE_JMA.captures(hay) {
            Some(s) => {
                let id = &s[0];
                Some(SecIdBase{
                    processed_id: id.to_string(),
                    sec_id_type_id: 138,
                })
            },
            None => None,
        }
    }
    else {
        None
    }
}


fn contains_jprn(hay: &str) -> Option<SecIdBase> {
    if hay.starts_with("JPRN") {
        static RE_JPRN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[0-9]{8}").unwrap());
        if RE_JPRN.is_match(hay) {
            match RE_JPRN.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    let processed_id = format!("JPRN-UMIN{}", id);  
                    Some(SecIdBase{
                        processed_id: processed_id,
                        sec_id_type_id: 141,
                    })
                },
                None => None,
            }
        }
        else {
            Some(SecIdBase{
                processed_id: hay.to_string(),
                sec_id_type_id: 127,
            })
        }
    }
    else {
        None
    }
}


fn contains_nl(hay: &str) -> Option<SecIdBase> {
    if hay.starts_with("NL") {
        static RE_OMON: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^NL-OMON\d{5}$").unwrap());
        static RE_NL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^NL\d{1,4}$").unwrap());
        match RE_OMON.captures(hay) {
            Some(s) => {
                let id = &s[0];
                Some(SecIdBase{
                    processed_id: id.to_string(),
                    sec_id_type_id: 132,
                })
            },

            None => {
                match RE_NL.captures(hay) {
                Some(s) => {
                    let id = &s[0];
                    Some(SecIdBase{
                        processed_id: id.to_string(),
                        sec_id_type_id: 182,
                        })
                    },
                    None => None, 
                }
            }
        }
    }
    else {
        None
    }
}

fn contains_ntr(hay: &str) -> Option<SecIdBase> {
    if hay.starts_with("NTR") {
        static RE_NTR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^NTR\d{1,4}$").unwrap());
        match RE_NTR.captures(hay) {
            Some(s) => {
                let id = &s[0];
                Some(SecIdBase{
                    processed_id: id.to_string(),
                    sec_id_type_id: 181,
                })
            },
            None => None,
        }
    }
    else {
        None
    }
}

fn contains_rpuec(hay: &str) -> Option<SecIdBase> {
    if hay.starts_with("PER") {
        static RE_PER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^PER-[0-9]{3}-").unwrap());
        match RE_PER.captures(hay) {
            Some(_s) => { 
                let id = hay;
                Some(SecIdBase{
                    processed_id: id.to_string(),
                    sec_id_type_id: 129,
                })
            },
            None => None,
        }
    }
    else {
        None
    }
}
