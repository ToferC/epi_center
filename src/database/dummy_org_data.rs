
use rand::Rng;
use rand::{seq::SliceRandom};
use async_graphql::Error;
use uuid::Uuid;

use crate::progress::progress::ProgressLogger;
use crate::database::{create_validations, generate_requirement};
use crate::models::{Person, Organization, NewPerson, NewOrganization, 
    Role, NewRole, Team, NewTeam, OrgTier, NewOrgTier, OrgOwnership, NewOrgOwnership,
    TeamOwnership, NewTeamOwnership, HrGroup, SkillDomain, Skill, NewWork, CapabilityLevel, WorkStatus, Work,
    NewRequirement, Requirement,
};

use super::{create_fake_capabilities, generate_dummy_publications_and_contributors, generate_tasks};

/// Creates basic Org, People, Teams, Roles, Work, etc in the database
pub fn pre_populate_db_schema() -> Result<(), Error> {

    //let mut conn = connection()?;

    // Set up Organization
    println!("Creating Organization");

    let mut science_org_ids: Vec<uuid::Uuid> = Vec::new();

    let o = NewOrganization::new(
        "Canadian Armed Forces".to_string(),
        "Forces armées canadiennes".to_string(),
        "CAF".to_string(),
        "FAC".to_string(),
        "Military".to_string(),
        "somehttps://www.canada.ca/en/department-national-defence.html_url".to_string(),
    );

    let org = Organization::create(&o).expect("Unable to create new organization");

    science_org_ids.push(org.id);

    // Set up Science Orgs for Affiliations

    let places = vec![("British Columbia", "UBC"), ("Manitoba", "UM"), ("Toronto", "UofT"), ("Quebec", "UQAM"),
        ("Alberta", "UofA"), ("Saskatchewan", "USask"), ("New Brunswick", "UNB"), ("Nova Scotia", "NSCAD")];


    for place in places {
        
        let new_science_org = NewOrganization::new(
            format!("University of {}", place.0),
            format!("Universitaire de {}", place.0),
            place.1.to_owned(),
            place.1.to_owned(),
            "Academic".to_string(),
            "some_url".to_string(),
        );

        let science_org = Organization::create(&new_science_org)
            .expect("Unable to create new organization");

        science_org_ids.push(science_org.id);
    }

    // Set up Org Tiers

    let mut org_tiers: Vec<OrgTier> = Vec::new();

    let mut requirements_vec: Vec<NewRequirement> = Vec::new();

    let tt = NewOrgTier::new(
            org.id, 
            1, 
            "Office of the Chief of Defence Staff".to_string(), 
            "Bureau de chef d’état-major de la Défense".to_string(), 
            SkillDomain::Leadership,
            None);

    let top_tier = OrgTier::create(&tt).unwrap();

    org_tiers.push(top_tier.clone());

    let org_path = "seeds/org_structure.csv";

    let mut reader = csv::Reader::from_path(org_path)
        .expect("Unable to load csv");

    for r in reader.records() {

        let record = r.unwrap();

        let division: String = String::from(&record[0]);
        let centre: String = String::from(&record[1]);
        let branch: String = String::from(&record[2]);
        let domain: String = String::from(&record[3]);

        let domain: SkillDomain = match domain.as_str() {
            "FIN" => SkillDomain::Finance,
            "HR" => SkillDomain::HumanResources,
            "COM" => SkillDomain::Communications,
            "STR" => SkillDomain::Strategy,
            "CMB" => SkillDomain::Combat,
            "JOP" => SkillDomain::JointOperations,
            "MAN" => SkillDomain::Management,
            "LEAD" => SkillDomain::Leadership,
            "ADM" => SkillDomain::Administration,
            "INT" => SkillDomain::Intelligence,
            "IT" => SkillDomain::InformationTechnology,
            "ENG" => SkillDomain::Engineering,
            "MED" => SkillDomain::Medical,
            _ => SkillDomain::Combat,
        };

        println!("Creating Org Tiers for: {}", branch);

        // create org tiers if not already existing
        // Create branch and get id
        let adm = NewOrgTier::new(
            org.id, 
            2, 
            branch.to_owned(), 
            branch.to_owned(),
            SkillDomain::Leadership,
            Some(top_tier.id),
        );
    
        let adm_tier = OrgTier::get_or_create(&adm)
            .expect("Unable to get or create org_tier");


        org_tiers.push(adm_tier.clone());

        // Create centre and get id
        let ctr = NewOrgTier::new(
            org.id, 
            3, 
            centre.to_owned(), 
            centre.to_owned(),
            SkillDomain::Leadership,
            Some(adm_tier.id),
        );
    
        let ctr_tier = OrgTier::get_or_create(&ctr)
            .expect("Unable to get or create org_tier");

        org_tiers.push(ctr_tier.clone());

        // Create division and get id
        let div = NewOrgTier::new(
            org.id, 
            4, 
            division.to_owned(), 
            division.to_owned(),
            domain,
            Some(ctr_tier.id),
        );
    
        let div_tier = OrgTier::get_or_create(&div)
            .expect("Unable to get or create org_tier");

        org_tiers.push(div_tier.clone());


        // Create 3 teams per division
        for i in 1..=3 {
            let tm = NewOrgTier::new(
                org.id, 
                5, 
                format!("{} Team {}", division.to_owned(), i), 
                format!("{} Team {}", division.to_owned(), i),
                domain,
                Some(div_tier.id),
            );
        
            let tm_tier = OrgTier::get_or_create(&tm)
                .expect("Unable to get or create org_tier");


            org_tiers.push(tm_tier);
        }
    }

    // Create Org Addresses

    let mut addresses = Vec::new();

    addresses.push(vec![
        "200 René Lévesque Blvd. West".to_string(),
        "Montreal".to_string(),
        "Quebec".to_string(),
        "H2Z 1X4".to_string(),
    ]);

    addresses.push(vec![
        "100 Colonnade Rd".to_string(),
        "Ottawa".to_string(),
        "Ontario".to_string(),
        "K2E 7J5".to_string(),
    ]);

    addresses.push(vec![
        "391 York Avenue".to_string(),
        "Winnipeg".to_string(),
        "Manitoba".to_string(),
        "R3C 4W1".to_string(),
    ]);

    addresses.push(vec![
        "180 Queen Street West".to_string(),
        "Toronto".to_string(),
        "Ontario".to_string(),
        "M5V 3L7".to_string(),
    ]);

    // Set up Persons
    println!("Set up people and capabilities");

    let mut rng = rand::thread_rng();

    let mut new_people: Vec<NewPerson> = Vec::new();

    let path = "seeds/names.csv";

    let mut reader = csv::Reader::from_path(path).unwrap();

    for r in reader.records() {

        let record = r.unwrap();

        let gn: String = record[0].trim().to_owned();
        let famn: String = record[1].trim().to_owned();

        let addr = addresses.choose(&mut rng).unwrap();
        
        let p = NewPerson::new(
            uuid::Uuid::new_v4(),
            famn.to_owned(),
            gn.to_owned(),
            format!("{}.{}_{}@phac-aspc.gc.ca", &gn, &famn, rng.gen_range(0..9999)).to_lowercase(),
            gen_rand_number(),
            addr[0].to_owned(),
            addr[1].to_owned(),
            addr[2].to_owned(),
            addr[3].to_owned(),
            "Canada".to_string(),
            org.id,
            gen_rand_number(),
            gen_rand_number(),
        );
        new_people.push(p);
    }

    // Insert people
    let mut progress_people = ProgressLogger::new("Inserting People".to_owned(),new_people.len());

    let r = Person::batch_create(new_people)?;
    println!("Inserted {} people.", r);
    progress_people.done();

    let mut people_ids = Person::get_all_ids()?;

    let mut progress_cap = ProgressLogger::new("Inserting Capabilities".to_owned(),people_ids.len());

    let _r = create_fake_capabilities(&people_ids, org.id, &science_org_ids)?;

    progress_cap.increment(); 
    progress_cap.done();

    // Set up Teams and roles data
    println!("Set up teams and roles");
    let mut progress_tier = ProgressLogger::new("Inserting teams and roles".to_owned(),org_tiers.len());


    let roles: Vec<&str> = "
        Sr. Policy Analyst; Policy Analyst; Jr. Policy Analyst; Epidemiologist; Administrative Officer; Designer; 
        Sr. Data Analyst; Data Analyst; Jr. Data Analyst; Project Officer; Scientist; Researcher".split("; ").collect();

    let work_verbs: Vec<&str> = "
        design; write; revise; audit; draft; review; approve; present; 
        research; analyze data on; visualize data on; develop; plan; 
        create mvp on; test; prototype; peer review on".split("; ").collect();

    // Set up OrgTierOwnership
    for ot in org_tiers.clone() {
        // allocate people to org tiers - starting at the top

        // set org_tier_owner
        let owner_id = people_ids.pop().unwrap();

        // get domain skills
        let domain_skills = Skill::get_by_domain(ot.primary_domain)
                .expect("Unable to get skills");
        
        // set exec grp and level

        let (grp, level, num_members, title_str) = match ot.tier_level {
            1 => (HrGroup::DM, 1, 2, "President"),
            2 => (HrGroup::EX, 3, 2, "Vice President"),
            3 => (HrGroup::EX, 2, 2, "Director General"),
            4 => (HrGroup::EX, 1, 1, "Director"),
            5 => (HrGroup::EC, 4, 4, "Manager"),
            _ => (HrGroup::EC, 2, 3, "Special Advisor"),
        };

        let ownership = NewOrgOwnership::new(
            owner_id,
            ot.id,
        );

        let _res = OrgOwnership::create(&ownership).unwrap();

        // create team at this level

        let team_name = ot.name_en.clone().trim().to_string();

        let new_team = NewTeam::new(
            team_name.trim().to_string(), 
            format!("{}_FR", team_name.trim()),
            org.id, 
            ot.id,
            ot.primary_domain,
            "Description_EN".to_string(), 
            "Description_FR".to_string()
        );

        let team = Team::get_or_create(&new_team).expect("Unable to create team");
        
        // if owner, also set management role for team at that tier

        let nr = NewRole::new(
            Some(owner_id), 
            team.id, 
            format!("{} - {}", title_str, ot.name_en.clone()), 
            format!("{} - {}", title_str, ot.name_fr.clone()), 
            0.80, 
            true,
            grp,
            level,
            chrono::Utc::now().naive_utc(), 
            None
        );

        let role_res = Role::create(&nr).unwrap();

        // Create requirements for role
        let skill_ids: Vec<Uuid> = domain_skills
                    .choose_multiple(&mut rng, 3)
                    .map(|x| x.id)
                    .collect();

        for skill_id in skill_ids {
            let role_requirement = generate_requirement(role_res.id, skill_id, grp, level, &mut rng);
            requirements_vec.push(role_requirement.clone());
        }

        // Set up tasks from this manager
        // Could base this on the managers skills, but too much detail for now

        let mut tasks = Vec::new();

        // Generate tasks for team based on skills under chosen domain
        for _ in 0..=8 {

            let subject = domain_skills
                .choose(&mut rng)
                .unwrap()
                .clone();

            let task = generate_tasks(
                &mut rng,
                &ot.primary_domain,
                &subject.name_en, 
                &role_res.id,
                ot.tier_level
            ).unwrap();

            tasks.push(task);
        }

        // Set team ownership

        let new_team_ownership = NewTeamOwnership::new(
            owner_id,
            team.id,
            chrono::Utc::now().naive_utc(),
            None,
        );

        let _to = TeamOwnership::create(&new_team_ownership).expect("Unable to create ownership");

        // Populate the rest of the team, assigning roles at random
        // println!("Populate the rest of the team, assigning roles at random for {} people", num_members.min(people.len()));

        for _i in 0..num_members.min(people_ids.len()) {

            let mut role_vec = Vec::new();
            
            let person_id = people_ids.pop().unwrap();

            let role = *roles.choose(&mut rng).unwrap();

            let grp: HrGroup = rand::random();

            let level = rng.gen_range(2..6);

            
            let start_date = chrono::Utc::now().naive_utc();
            let modifier = chrono::Duration::days(rng.gen_range(-300..300));

            // Cover 3 years, 50% chance to move each year

            // set 15% chance for role to be vacant

            let p_id = if rng.gen_bool(0.85) {
                Some(person_id)
            } else {
                None
            };

            let mut nr = NewRole::new(
                p_id, 
                team.id, 
                role.trim().to_string(), 
                format!("{}_FR", role.trim()), 
                0.80, 
                true,
                grp,
                level,
                start_date + modifier, 
                None
            );

            let role_res = Role::create(&nr).unwrap();

            // Create requirements for role
            let skill_ids: Vec<Uuid> = domain_skills
                    .choose_multiple(&mut rng, 3)
                    .map(|x| x.id)
                    .collect();

            for skill_id in skill_ids {
                let role_requirement = generate_requirement(role_res.id, skill_id, grp, level, &mut rng);
                requirements_vec.push(role_requirement.clone());
            }

            match rng.gen_range(0..10) {
                0..=5 => continue,
                6..=8 => {
                    nr.active = false;
                    nr.hr_level -= 1;
                    nr.start_datestamp -= chrono::Duration::days(rng.gen_range(-300..-100));
                    role_vec.push(nr.clone())
                },
                9..=10 => {
                    for _i in 1..=3 {
                        nr.active = false;
                        nr.hr_level -= 1;
                        nr.start_datestamp -= chrono::Duration::days(rng.gen_range(-600..-150));
                        role_vec.push(nr.clone())
                    }
                },
                _ => continue,
            };

            let _r = Role::batch_create(role_vec)?;

            // Assign work to the roles based on the team's tasks
            let mut work = Vec::new();

            for _ in 0..rng.gen_range(2..=4) {

                let task = tasks.choose(&mut rng).unwrap().clone();

                let capability_level: CapabilityLevel = rand::random();
                
                let effort = rng.gen_range(1..=3);

                let task_status: WorkStatus = rand::random();

                let nw = NewWork::new(
                    task.id,
                    role_res.id,
                    format!("{} {}",
                        work_verbs.choose(&mut rng).unwrap().trim(),
                        task.title.trim()),
                    Some("https://www.phac-aspc.ca/some_url".to_string()),
                    task.domain,
                    capability_level,
                    effort,
                    task_status,
                );

                work.push(nw);
            }
            let _r = Work::batch_create(&work)?;
        }

        progress_tier.increment();

        if requirements_vec.len() > 500 {
            let _res = Requirement::batch_create(&requirements_vec)?;
            requirements_vec = Vec::new();
        }

    }
    progress_tier.done();

    // Create Publications and Assign Contributors
    println!("Pre-populating Publications and contributors");

    let _res = generate_dummy_publications_and_contributors(&science_org_ids)
        .expect("Unable to create publications and contributors");

    // Create dummy validatoins for capabilities
    let _res = create_validations()
        .expect("Unable to create validations");

    let _res = Requirement::batch_create(&requirements_vec)?;

    Ok(())

}


pub fn gen_rand_number() -> String {
    let mut rng = rand::thread_rng();

    let rand_num: String = (0..11)
        .map(|_| {
            let i = rng.gen_range(0..10);
            i.to_string()
        }).collect();

    rand_num
}