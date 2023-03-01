use rand::Rng;
use rand::{thread_rng, seq::SliceRandom};

use crate::models::{Person, Organization, NewPerson, NewOrganization, 
    Role, NewRole, Team, NewTeam, OrgTier, NewOrgTier, OrgOwnership, NewOrgOwnership,
    TeamOwnership, NewTeamOwnership, NewCapability, Capability, Skill, NewSkill, CapabilityLevel};

pub fn pre_populate_skills() -> Result<Vec<Skill>> {

    let public_health_skills: Vec<String> = "
        Epidemiology; One Health; Community Health; Mental Health; Health Inequalities; Multi-sectoral Partnerships; Drug Use; Vaccines
    ".split("; ").iter().collect();
    
    let policy_skills: Vec<String> = "
        Policy Development; Policy Measurement; Policy Implementation; Strategic Policy; Evaluation; MC & TBsub Writing; Governance
    ".split("; ").iter().collect();

    let data_skills: Vec<String> = "
        Data Access; Data Collection; Data Analysis; Data Management; Public Health Infomatics; Bioinfomatics; Data Visualization
    ".split("; ").iter().collect();

    let it_skills: Vec<String> = "
        Cloud Administration; Cloud Architecture; Programming - Python; Database Administration; Networking; Back-end Development; Front-end Development
    ".split("; ").iter().collect();

    let hr_skills: Vec<String> = "
        Staffing; Classification; Recruiting; Pay and Compensation
    ".split("; ").iter().collect();

    let finance_skills: Vec<String> = "
        Accounting; Forecasting; Audit; Government Budgeting
    ".split("; ").iter().collect();

    let comms_skills: Vec<String> = "
        Writing; Public Speaking; Media; Storytelling
    ".split("; ").iter().collect();

    let administration_skills: Vec<String> = "
        ATIP; Budgeting; Operations; HR Processing; Travel
    ".split("; ").iter().collect();

    let scientific_skills: Vec<String> = "
        Anti-Microbial Resistance; Whole Genome Sequencing; Genomics; Modelling; Climate Change
    ".split("; ").iter().collect();

    let medical_skills: Vec<String> = "
        Pediatrics; Maternal Health; Respiratory Health; Cardiovascular Health; Dental Health; Nutration; Chronic Disease
    ".split("; ").iter().collect();

    let management_skills: Vec<String> = "
        People Management; Action Management; Financial Management; Performance Management
    ".split("; ").iter().collect();

    let leadership_skills: Vec<String> = "
        Vision Setting; Innovation; Foresight; Political Influence; Mobilizing People
    ".split("; ").iter().collect();

    let partnership_skills: Vec<String> = "
        Cross-sectoral Partnerships; International Partnerships; Inter-governmental Partnerships; Community Partnerships
    ".split("; ").iter().collect();


}

pub fn create_fake_capabilities_for_person(id: Uuid) -> Result<Capability> {

}