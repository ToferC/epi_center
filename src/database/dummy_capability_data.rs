
use uuid::Uuid;
use async_graphql::Error;

use crate::models::{Person, Organization, NewPerson, NewOrganization, 
    Role, NewRole, Team, NewTeam, OrgTier, NewOrgTier, OrgOwnership, NewOrgOwnership,
    TeamOwnership, NewTeamOwnership, NewCapability, Capability, Skill, NewSkill, CapabilityLevel, SkillDomain};

pub fn pre_populate_skills() -> Result<Vec<Skill>, Error> {

    let public_health_skills: Vec<&str> = "
        Epidemiology; One Health; Community Health; Mental Health; Health Inequalities; Multi-sectoral Partnerships; Drug Use; Vaccines; 
        Risk Assessment; Surveillance
    ".split("; ").collect();

    for s in public_health_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::PublicHealth,
        );

        let _res = Skill::create(&ns)?;
    }
    
    let policy_skills: Vec<&str> = "
        Policy Development; Policy Measurement; Policy Implementation; Strategic Policy; Evaluation; MC & TBsub Writing; Governance
    ".split("; ").collect();

    for s in policy_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Policy,
        );

        let _res = Skill::create(&ns)?;
    }

    let data_skills: Vec<&str> = "
        Data Access; Data Collection; Data Analysis; Data Management; Public Health Infomatics; Bioinfomatics; Data Visualization
    ".split("; ").collect();

    for s in data_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Data,
        );

        let _res = Skill::create(&ns)?;
    }

    let it_skills: Vec<&str> = "
        Cloud Administration; Cloud Architecture; Programming - Python; Database Administration; Networking; Back-end Development; Front-end Development
    ".split("; ").collect();

    for s in it_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::InformationTechnology,
        );

        let _res = Skill::create(&ns)?;
    }

    let hr_skills: Vec<&str> = "
        Staffing; Classification; Recruiting; Pay and Compensation
    ".split("; ").collect();

    for s in hr_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::HumanResources,
        );

        let _res = Skill::create(&ns)?;
    }

    let finance_skills: Vec<&str> = "
        Accounting; Forecasting; Audit; Government Budgeting
    ".split("; ").collect();

    for s in finance_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Finance,
        );

        let _res = Skill::create(&ns)?;
    }

    let comms_skills: Vec<&str> = "
        Writing; Public Speaking; Media; Storytelling
    ".split("; ").collect();

    for s in comms_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Communications,
        );

        let _res = Skill::create(&ns)?;
    }

    let administration_skills: Vec<&str> = "
        ATIP; Budgeting; Operations; HR Processing; Travel
    ".split("; ").collect();

    for s in administration_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Administration,
        );

        let _res = Skill::create(&ns)?;
    }

    let scientific_skills: Vec<&str> = "
        Anti-Microbial Resistance; Whole Genome Sequencing; Genomics; Modelling; Climate Change
    ".split("; ").collect();

    for s in scientific_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Scientific,
        );

        let _res = Skill::create(&ns)?;
    }

    let medical_skills: Vec<&str> = "
        Pediatrics; Maternal Health; Respiratory Health; Cardiovascular Health; Dental Health; Nutration; Chronic Disease
    ".split("; ").collect();

    for s in medical_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Medical,
        );

        let _res = Skill::create(&ns)?;
    }

    let management_skills: Vec<&str> = "
        People Management; Action Management; Financial Management; Performance Management
    ".split("; ").collect();

    for s in management_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Management,
        );

        let _res = Skill::create(&ns)?;
    }

    let leadership_skills: Vec<&str> = "
        Vision Setting; Innovation; Fo_resight; Political Influence; Mobilizing People
    ".split("; ").collect();

    for s in leadership_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Leadership,
        );

        let _res = Skill::create(&ns)?;
    }

    let partnership_skills: Vec<&str> = "
        Cross-sectoral Partnerships; International Partnerships; Inter-governmental Partnerships; Community Partnerships
    ".split("; ").collect();

    for s in partnership_skills {
        let ns = NewSkill::new(
            s.to_string(),
            format!("{}_FR", s.to_string()),
            SkillDomain::Partnerships,
        );

        let _res = Skill::create(&ns)?;
    }

    Skill::get_all()

}

pub fn create_fake_capabilities_for_person(id: Uuid) {

}