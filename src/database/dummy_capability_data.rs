
use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;
use async_graphql::Error;

use crate::models::{Affiliation, NewAffiliation, Organization, NewPerson, NewOrganization, 
    Role, NewRole, Team, NewTeam, OrgTier, NewOrgTier, OrgOwnership, NewOrgOwnership,
    TeamOwnership, NewTeamOwnership, NewCapability, Capability, Skill, NewSkill, CapabilityLevel, SkillDomain, LanguageLevel, LanguageName, NewLanguageData, LanguageData};

pub fn pre_populate_skills() -> Result<(), Error> {

    let public_health_skills: Vec<&str> = "
        Epidemiology; One Health; Community Health; Mental Health; Health Inequalities; Multi-sectoral Partnerships; Drug Use; Vaccines; 
        Risk Assessment; Surveillance
    ".split("; ").collect();

    for s in public_health_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::PublicHealth,
        );

        let _res = Skill::create(&ns)?;
    }
    
    let policy_skills: Vec<&str> = "
        Policy Development; Policy Measurement; Policy Implementation; Strategic Policy; Evaluation; MC & TBsub Writing; Governance
    ".split("; ").collect();

    for s in policy_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Policy,
        );

        let _res = Skill::create(&ns)?;
    }

    let data_skills: Vec<&str> = "
        Data Access; Data Collection; Data Analysis; Data Management; Public Health Infomatics; Bioinfomatics; Data Visualization
    ".split("; ").collect();

    for s in data_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Data,
        );

        let _res = Skill::create(&ns)?;
    }

    let it_skills: Vec<&str> = "
        Cloud Administration; Cloud Architecture; Programming - Python; Database Administration; Networking; Back-end Development; Front-end Development
    ".split("; ").collect();

    for s in it_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::InformationTechnology,
        );

        let _res = Skill::create(&ns)?;
    }

    let hr_skills: Vec<&str> = "
        Staffing; Classification; Recruiting; Pay and Compensation
    ".split("; ").collect();

    for s in hr_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::HumanResources,
        );

        let _res = Skill::create(&ns)?;
    }

    let finance_skills: Vec<&str> = "
        Accounting; Forecasting; Audit; Government Budgeting
    ".split("; ").collect();

    for s in finance_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Finance,
        );

        let _res = Skill::create(&ns)?;
    }

    let comms_skills: Vec<&str> = "
        Writing; Public Speaking; Media; Storytelling
    ".split("; ").collect();

    for s in comms_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Communications,
        );

        let _res = Skill::create(&ns)?;
    }

    let administration_skills: Vec<&str> = "
        ATIP; Budgeting; Operations; HR Processing; Travel
    ".split("; ").collect();

    for s in administration_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Administration,
        );

        let _res = Skill::create(&ns)?;
    }

    let scientific_skills: Vec<&str> = "
        Anti-Microbial Resistance; Whole Genome Sequencing; Genomics; Modelling; Climate Change
    ".split("; ").collect();

    for s in scientific_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Scientific,
        );

        let _res = Skill::create(&ns)?;
    }

    let medical_skills: Vec<&str> = "
        Pediatrics; Maternal Health; Respiratory Health; Cardiovascular Health; Dental Health; Nutration; Chronic Disease
    ".split("; ").collect();

    for s in medical_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Medical,
        );

        let _res = Skill::create(&ns)?;
    }

    let management_skills: Vec<&str> = "
        People Management; Action Management; Financial Management; Performance Management
    ".split("; ").collect();

    for s in management_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Management,
        );

        let _res = Skill::create(&ns)?;
    }

    let leadership_skills: Vec<&str> = "
        Vision Setting; Innovation; Foresight; Political Influence; Mobilizing People
    ".split("; ").collect();

    for s in leadership_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Leadership,
        );

        let _res = Skill::create(&ns)?;
    }

    let partnership_skills: Vec<&str> = "
        Cross-sectoral Partnerships; International Partnerships; Inter-governmental Partnerships; Community Partnerships
    ".split("; ").collect();

    for s in partnership_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Partnerships,
        );

        let _res = Skill::create(&ns)?;
    }

    // create tasks for each skill

    let skills = Skill::get_all();

    Ok(())

}

pub fn create_fake_capabilities_for_person(person_id: Uuid, org_id: Uuid, science_org_id: Uuid) -> Result<(), Error>{

    let mut rng = rand::thread_rng();

    // Create LanguageDatas

    let primary_language = vec![LanguageName::English, LanguageName::French]
        .choose(&mut rng).unwrap().clone();

    let secondary_language = match primary_language {
        LanguageName::English => LanguageName::French,
        LanguageName::French => LanguageName::English,
        _ => LanguageName::English,
    };

    let primary = NewLanguageData::new(
        person_id, 
        primary_language, 
        Some(LanguageLevel::E),
        Some(LanguageLevel::E),
        Some(LanguageLevel::E)
    );

    let _res = LanguageData::create(&primary)?;

    if rng.gen_bool(0.5) {

        let beginner = vec![LanguageLevel::B, LanguageLevel::A, LanguageLevel::A];
        let intermediate = vec![LanguageLevel::C, LanguageLevel::B, LanguageLevel::B];
        let professional = vec![LanguageLevel::C, LanguageLevel::B, LanguageLevel::C];
        let fluent = vec![LanguageLevel::E, LanguageLevel::E, LanguageLevel::E];

        let chosen = match rng.gen_range(0..=10) {
            0..=3 => beginner,
            4..=6 => intermediate,
            7..=9 => professional,
            10 => fluent,
            _ => beginner,
        };

        let secondary = NewLanguageData::new(
            person_id, 
            secondary_language, 
            Some(chosen[0]),
            Some(chosen[1]),
            Some(chosen[2])
        );

        let _res = LanguageData::create(&secondary)?;
    }

    // Choose three random domains from SkillDomain

    let mut sds: Vec<SkillDomain> = Vec::new();

    for _ in 0..3 {
        let sd: SkillDomain = rand::random();
        if !sds.contains(&sd) {
            sds.push(sd);
        }
    }

    // If person has Science domain, 20% chance to add an affiliation

    if sds.contains(&SkillDomain::Scientific) && rng.gen_bool(0.2) {
        let na = NewAffiliation::new(
            person_id,
            science_org_id,
            "Research Affiliate".to_string(),
            None,
        );

        let _res = Affiliation::create(&na)?;
    }

    // Identify highest CapabilityLevel

    for sd in sds {
        let domain = sd;

        // Choose 3-5 random skills from each domain

        let skills_in_domain = Skill::get_by_domain(domain)?;

        // Choose 3-5 random skills from domain

        let mut selected_skills: Vec<Skill> = Vec::new();

        for _ in 0..3 {
            let skill = skills_in_domain.choose(&mut rng).unwrap();
            if !selected_skills.contains(&skill) {
                selected_skills.push(skill.clone());
            }
        }

        let mut capability_level: CapabilityLevel = rand::random();

        for skill in selected_skills {
            let nc = NewCapability::new(person_id, skill.id, org_id, capability_level);
            let res = Capability::create(&nc)?;
            capability_level = capability_level.step_down();
        }

        // create work and tasks
    }

    Ok(())
}