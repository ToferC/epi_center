
use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;
use async_graphql::Error;
use crate::progress::progress::ProgressLogger;

use crate::models::{Affiliation, NewAffiliation, NewCapability, Capability, Skill, NewSkill, CapabilityLevel, SkillDomain, LanguageLevel, LanguageName, NewLanguageData, LanguageData, Person, NewValidation, Validation};

pub fn pre_populate_skills() -> Result<(), Error> {

    let military_combat_skills: Vec<&str> = "
        Infantry Operations; Armor Operations; Artillery Operations; Close Quarters Combat; Marksmanship; Small Unit Tactics; 
        Battlefield Awareness; Weapons Systems; Urban Combat
    ".split("; ").collect();

    for s in military_combat_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Combat,
        );

        let _res = Skill::create(&ns)?;
    }
    
    let military_strategy_skills: Vec<&str> = "
        Strategic Planning; Military Doctrine; Operational Planning; Tactical Analysis; Force Structure Planning; Intelligence Analysis; Mission Planning
    ".split("; ").collect();

    for s in military_strategy_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Strategy,
        );

        let _res = Skill::create(&ns)?;
    }

    let military_intelligence_skills: Vec<&str> = "
        Intelligence Gathering; Reconnaissance; Surveillance; Signal Intelligence; Human Intelligence; Threat Assessment; Data Analysis
    ".split("; ").collect();

    for s in military_intelligence_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Intelligence,
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

    let military_engineering_skills: Vec<&str> = "
        Explosive Ordnance Disposal; Combat Engineering; Fortification Design; Bridge Construction; Demolitions; Mine Warfare; Technical Intelligence
    ".split("; ").collect();

    for s in military_engineering_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::Engineering,
        );

        let _res = Skill::create(&ns)?;
    }

    let combat_medical_skills: Vec<&str> = "
        Combat Medicine; Battlefield Trauma Care; Field Surgery; Emergency Medicine; Triage; Medical Evacuation; Chemical Biological Treatment
    ".split("; ").collect();

    for s in combat_medical_skills {
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

    let joint_operations_skills: Vec<&str> = "
        Joint Operations; Coalition Warfare; Multinational Coordination; Interoperability; Combined Arms; Allied Integration; International Military Cooperation
    ".split("; ").collect();

    for s in joint_operations_skills {
        let ns = NewSkill::new(
            s.trim().to_string(),
            format!("{}_FR", s.trim().to_string()),
            SkillDomain::JointOperations,
        );

        let _res = Skill::create(&ns)?;
    }

    Ok(())

}

pub fn create_fake_capabilities(
    people_ids: &Vec<Uuid>, 
    org_id: Uuid, 
    science_org_ids: &Vec<Uuid>,
) -> Result<(), Error>{

    let mut rng = rand::thread_rng();

    let mut capabilities = Vec::new();
    let mut language_datas = Vec::new();

    for person_id in people_ids {

        let science_org_id = science_org_ids.choose(&mut rng).unwrap();

        // Create LanguageDatas

        let primary_language = vec![LanguageName::English, LanguageName::French]
            .choose(&mut rng).unwrap().clone();

        let secondary_language = match primary_language {
            LanguageName::English => LanguageName::French,
            LanguageName::French => LanguageName::English,
            _ => LanguageName::English,
        };

        let primary = NewLanguageData::new(
            *person_id, 
            primary_language, 
            Some(LanguageLevel::E),
            Some(LanguageLevel::E),
            Some(LanguageLevel::E)
        );

        language_datas.push(primary);

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
                *person_id, 
                secondary_language, 
                Some(chosen[0]),
                Some(chosen[1]),
                Some(chosen[2])
            );

            language_datas.push(secondary);
        }

        // Choose three random domains from SkillDomain

        let mut sds: Vec<SkillDomain> = Vec::new();

        for _ in 0..2 {
            let sd: SkillDomain = rand::random();
            if !sds.contains(&sd) {
                sds.push(sd);
            }
        }

        // If person has Science domain, 20% chance to add an affiliation

        if sds.contains(&SkillDomain::Engineering) && rng.gen_bool(0.2) {
            let na = NewAffiliation::new(
                *person_id,
                *science_org_id,
                org_id,
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

                let nc = NewCapability::new(
                    *person_id, 
                    skill.id, // Error here
                    org_id, 
                    capability_level,
                );

                capabilities.push(nc);

                // respect limits for batch inserts
                if capabilities.len() > 1000 {
                    let _r = Capability::batch_create(&capabilities)?;
                    println!("Inserted {} capabilities", &capabilities.len());
                    capabilities = Vec::new();
                }

                capability_level = capability_level.step_down();
            }

            // create work and tasks
        }
    }

    // save language datas
    let _r = LanguageData::batch_create(language_datas)?;

    // save remaining capabilities
    let _r = Capability::batch_create(&capabilities)?;

    Ok(())
}

pub fn create_validations() -> Result<(), Error> {

    println!("Adding validations to capabilities");

    let mut rng = rand::thread_rng();

    let person_ids = Person::get_all_ids()?;
    
    let capabilities = Capability::get_all()?;
    
    let mut progress = ProgressLogger::new("Adding validations to capabilities".to_owned(),capabilities.len());
    for (i, mut c) in capabilities.into_iter().enumerate() {
        let mut validations = Vec::new();

        let validators: Vec<Uuid> = person_ids.choose_multiple(&mut rng, 4)
            .cloned()
            .collect();

        if i % 100 == 0 {
            print!(".")
        }

        let mut validated_levels: Vec<CapabilityLevel> = Vec::new();

        for validator in validators {

            let assessment = match rng.gen_range(0..10) {
                0..=3 => c.self_identified_level.step_down(),
                4..=6 => c.self_identified_level,
                7..=9 => c.self_identified_level.step_up(),
                _ => c.self_identified_level.step_up(),
            };

            validated_levels.push(assessment);
    
            let v = NewValidation::new(
                validator,
                c.id,
                assessment,
            );

            validations.push(v.clone());
            
        }
        let _r = Validation::batch_create(validations)?;
        c.update_from_batch_validations(&validated_levels)?;
        progress.increment();
    }
    progress.done();

    Ok(())
}