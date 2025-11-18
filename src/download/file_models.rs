use chrono::NaiveDate;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct WHOLine
{
    pub trial_id: String,                           // 0
    pub last_updated: String,                       // 1
    pub sec_ids: String,                            // 2
    pub pub_title: String,                          // 3
    pub scientific_title: String,                   // 4
    pub url: String,                                // 5

    pub pub_contact_first_name: String,             // 6
    pub pub_contact_last_name: String,              // 7
    pub pub_contact_address: String,                // 8
    pub pub_contact_email: String,                  // 9
    pub pub_contact_tel: String,                    // 10
    pub pub_contact_affiliation: String,            // 11

    pub sci_contact_first_name: String,             // 12
    pub sci_contact_last_name: String,              // 13
    pub sci_contact_address: String,                // 14
    pub sci_contact_email: String,                  // 15
    pub sci_contact_tel: String,                    // 16
    pub sci_contact_affiliation: String,            // 17

    pub study_type: String,                         // 18
    pub study_design: String,                       // 19
    pub phase: String,                              // 20
    pub date_registration: String,                  // 21
    pub date_enrollement: String,                   // 22
    pub target_size: String,                        // 23
    pub recruitment_status:String,                  // 24

    pub primary_sponsor: String,                    // 25
    pub secondary_sponsors: String,                 // 26
    pub source_support: String,                     // 27
    pub countries: String,                          // 28
    pub conditions: String,                         // 29
    pub interventions: String,                      // 30

    pub age_min: String,                            // 31
    pub age_max: String,                            // 32
    pub gender: String,                             // 33
    pub inclusion_criteria: String,                 // 34
    pub exclusion_criteria: String,                 // 35

    pub primary_outcome: String,                    // 36
    pub secondary_outcomes: String,                 // 37

    pub bridging_flag: String,                      // 38
    pub bridged_type: String,                       // 39
    pub childs: String,                             // 40
    pub type_enrolment: String,                     // 41
    pub retrospective_flag: String,                 // 42

    pub results_actual_enrollment: String,          // 43
    pub results_url_link: String,                   // 44
    pub results_summary: String,                    // 45
    pub results_date_posted: String,                // 46
    pub results_date_first_pub: String,             // 47
    pub results_baseline_char: String,              // 48
    pub results_participant_flow: String,           // 49
    pub results_adverse_events: String,             // 50
    pub results_outcome_measures: String,           // 51
    pub results_url_protocol: String,               // 52

    pub results_ipd_plan: String,                   // 53
    pub results_ipd_description: String,            // 54
    pub results_date_completed: String,             // 55
    pub results_yes_no: String,                     // 56

    pub ethics_status: String,                      // 57
    pub ethics_approval_date: String,               // 58
    pub ethics_contact_name: String,                // 59
    pub ethics_contact_address:String,              // 60
    pub ethics_contact_phone: String,               // 61
    pub ethics_contact_email: String,               // 62

}


#[derive(Debug, serde::Serialize)]
pub struct WHORecord
{
    pub source_id: i32, 
    pub record_date: Option<String>,
    pub sd_sid: String, 
    pub pub_title: Option<String>,
    pub scientific_title: Option<String>,
    pub remote_url: Option<String>,
    pub pub_contact_givenname: Option<String>,
    pub pub_contact_familyname: Option<String>,
    pub pub_contact_email: Option<String>,
    pub pub_contact_affiliation: Option<String>,
    pub scientific_contact_givenname: Option<String>,
    pub scientific_contact_familyname: Option<String>,
    pub scientific_contact_email: Option<String>,
    pub scientific_contact_affiliation: Option<String>,
    pub study_type_orig: Option<String>,
    pub study_type: i32,
    pub study_status_orig: Option<String>,
    pub study_status: i32,
    pub date_registration: Option<String>,
    pub date_enrolment: Option<String>,
    pub target_size: Option<String>,
    pub primary_sponsor: Option<String>,
    pub secondary_sponsors: Option<String>,
    pub source_support: Option<String>,
    pub interventions: Option<String>,
    pub agemin: Option<String>,
    pub agemin_units: Option<String>,
    pub agemax: Option<String>,
    pub agemax_units: Option<String>,
    pub gender: Option<String>,
    pub inclusion_criteria: Option<String>,
    pub exclusion_criteria: Option<String>,
    pub primary_outcome: Option<String>,
    pub secondary_outcomes: Option<String>,
    pub bridging_flag: Option<String>,
    pub bridged_type: Option<String>,
    pub childs: Option<String>,
    pub type_enrolment: Option<String>,
    pub retrospective_flag: Option<String>,
    pub results_actual_enrollment: Option<String>,
    pub results_url_link: Option<String>,
    pub results_summary: Option<String>,
    pub results_date_posted: Option<String>,
    pub results_date_first_pub: Option<String>,
    pub results_url_protocol: Option<String>,
    pub ipd_plan: Option<String>,
    pub ipd_description: Option<String>,
    pub results_date_completed: Option<String>,
    pub results_yes_no: Option<String>,

    pub design_string: Option<String>,
    pub phase_string: Option<String>,

    pub country_list: Option<Vec<String>>,
    pub secondary_ids: Option<Vec<SecondaryId>>,
    pub study_features: Option<Vec<WhoStudyFeature>>,
    pub condition_list: Option<Vec<String>>,
    pub meddra_condition_list: Option<Vec<MeddraCondition>>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[allow(dead_code)]
pub struct SecondaryId
{
    pub source_field: String,
    pub sec_id: String,
    pub processed_id: String,
    pub sec_id_source: usize,
    pub sec_id_type_id: usize,
    pub sec_id_type: String,
}

#[allow(dead_code)]
impl SecondaryId {

    pub fn new(source_field: String, sec_id: String, 
        processed_id: String, sec_id_source: usize,
        sec_id_type_id: usize, sec_id_type: String)
         -> Self {SecondaryId {  
                source_field,
                sec_id,
                processed_id,
                sec_id_source,
                sec_id_type_id,
                sec_id_type,
            }
    }
  
    pub fn new_from_base(source_field: String, sec_id: String,
                sid: SecIdBase)
         -> Self {SecondaryId {  
            source_field,
            sec_id,
            processed_id: sid.processed_id,
            sec_id_source: sid.sec_id_source,
            sec_id_type_id: sid.sec_id_type_id,
            sec_id_type: sid.sec_id_type,
         }
    }

    pub fn clone(&self) -> SecondaryId {
        SecondaryId {
            source_field: self.source_field.clone(),
            sec_id: self.sec_id.clone(),
            processed_id: self.processed_id.clone(),
            sec_id_source: self.sec_id_source,
            sec_id_type_id: self.sec_id_type_id,
            sec_id_type: self.sec_id_type.clone(),
        }
    }
    
}


#[allow(dead_code)]
pub struct SecIdBase
{
    pub processed_id: String,
    pub sec_id_source: usize,
    pub sec_id_type_id: usize,
    pub sec_id_type: String,
} 

#[allow(dead_code)]
impl SecIdBase {

    pub fn new(processed_id: String, sec_id_source: usize,
        sec_id_type_id: usize , sec_id_type: String) -> Self {
        SecIdBase {  
            processed_id,
            sec_id_source,
            sec_id_type_id,
            sec_id_type,
         }
    }
} 


#[derive(Debug, serde::Serialize)]
#[allow(dead_code)]
pub struct WhoStudyFeature
{
    pub ftype_id: usize,
    pub ftype: String,
    pub fvalue_id: usize,
    pub fvalue: String,
}

#[allow(dead_code)]
impl WhoStudyFeature {

    pub fn new(ftype_id: usize, ftype: &str,
        fvalue_id: usize, fvalue: &str) -> Self {
        WhoStudyFeature {
            ftype_id,
            ftype: ftype.to_string(),
            fvalue_id,
            fvalue: fvalue.to_string(),
        }
    }
}


#[derive(Debug, serde::Serialize)]
#[allow(dead_code)]
pub struct MeddraCondition
{
    pub version: String,
    pub level: String,
    pub code: String,
    pub term: String,
    pub soc_code: String,
    pub soc_term: String,
}

#[allow(dead_code)]
impl MeddraCondition {

    pub fn new(version: String, level: String,
        code: String, term: String, soc_code: String, 
        soc_term: String) -> Self {
            MeddraCondition {
            version,
            level,
            code,
            term,
            soc_code,
            soc_term,
        }
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct WHOSummary
{
    pub source_id: i32, 
    pub sd_sid: String, 
    pub title: Option<String>,
    pub remote_url: Option<String>,
    pub study_type: i32,
    pub study_status: i32,
    pub secondary_ids: Option<Vec<SecondaryId>>,
    pub date_registration: Option<String>,
    pub date_enrolment: Option<String>,
    pub results_yes_no: Option<String>,
    pub results_url_link: Option<String>,
    pub results_url_protocol: Option<String>,
    pub results_date_posted: Option<NaiveDate>,
    pub results_date_first_pub: Option<NaiveDate>,
    pub results_date_completed: Option<NaiveDate>,
    pub table_name: String,
    pub country_list: Option<Vec<String>>,
    pub date_last_rev: Option<NaiveDate>,
    pub dl_id: i32,
}
