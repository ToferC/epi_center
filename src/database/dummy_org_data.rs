
use rand::Rng;
use rand::{seq::SliceRandom};

use crate::models::{Person, Organization, NewPerson, NewOrganization, 
    Role, NewRole, Team, NewTeam, OrgTier, NewOrgTier, OrgOwnership, NewOrgOwnership,
    TeamOwnership, NewTeamOwnership, HrGroup, SkillDomain, Skill, NewWork, CapabilityLevel, WorkStatus, Work};

use super::{create_fake_capabilities_for_person, generate_dummy_publications_and_contributors, generate_tasks};

/// Creates basic Org, People, Teams, Roles, Work, etc in the database
pub fn pre_populate_db_schema() {

    // Set up Organization
    println!("Creating Organization");

    let mut science_org_ids: Vec<uuid::Uuid> = Vec::new();

    let o = NewOrganization::new(
        "Public Health Agency of Canada".to_string(),
        "Agence de Sante Public de Canada".to_string(),
        "PHAC".to_string(),
        "ASPC".to_string(),
        "Government".to_string(),
        "some_url".to_string(),
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

    let tt = NewOrgTier::new(
            org.id, 
            1, 
            "President".to_string(), 
            "President".to_string(), 
            None);

    let top_tier = OrgTier::create(&tt).unwrap();

    org_tiers.push(top_tier.clone());

    let org_path = "org_structure.csv";

    let mut reader = csv::Reader::from_path(org_path)
        .expect("Unable to load csv");

    for r in reader.records() {

        let record = r.unwrap();

        let division: String = String::from(&record[0]);
        let centre: String = String::from(&record[1]);
        let branch: String = String::from(&record[2]);

        // create org tiers if not already existing
        // Create branch and get id
        let adm = NewOrgTier::new(
            org.id, 
            2, 
            branch.to_owned(), 
            branch.to_owned(), 
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

    let mut people: Vec<Person> = Vec::new();

    let path = "names.csv";

    let mut reader = csv::Reader::from_path(path).unwrap();

    for r in reader.records() {

        let record = r.unwrap();

        let gn: String = String::from(&record[0]);
        let famn: String = String::from(&record[1]);

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

        let person = Person::create(&p).expect("Unable to create person");

        let science_org_id = &science_org_ids.choose(&mut rng).unwrap();

        let _capabilities_for_person = create_fake_capabilities_for_person(
            person.id, 
            org.id,
            **science_org_id,
        )
            .expect("Unable to create capabilities for person");

        people.push(person);
    } 

    // Set up Teams and roles data
    println!("Set up teams and roles");


    let teams = vec![("PMO", 5), ("VPO", 5), ("DGO DMIA", 5), ("DGO SPDI", 5), 
            ("Director Data Science", 2), ("Director Data Policy", 2), ("Director Data Partnerships", 2), ("Director Strategic Policy", 2),
            ("Data Ingestion", 8), ("Data Management", 8), ("Data Ethics", 8), ("Data Governance", 8),
            ("Internal Partnerships", 8), ("International Partnerships", 8), ("MCs and TBsubs", 8),
            ("New Public Health", 8),
        ];

    let roles: Vec<&str> = "
        Sr. Policy Analyst; Policy Analyst; Jr. Policy Analyst; Epidemiologist; Administrative Officer; Designer; 
        Sr. Data Analyst; Data Analyst; Jr. Data Analyst; Project Officer; Scientist; Researcher; 
        ".split("; ").collect();

    let work_verbs: Vec<&str> = "
        design; write; revise; audit; draft; review; approve; present; 
        research; analyze data on; visualize data on; develop; plan; 
        create mvp on; test; prototype; peer review on; 
        ".split("; ").collect();

    // Set up OrgTierOwnership

    for (ind, ot) in org_tiers.clone().iter().enumerate() {
        // allocate people to org tiers - starting at the top

        // set org_tier_owner
        let mut owner = people.pop().unwrap();
        
        // set exec grp and level

        let (grp, lvl) = match ot.tier_level {
            1 => (HrGroup::DM, 1),
            2 => (HrGroup::EX, 4),
            3 => (HrGroup::EX, 3),
            4 => (HrGroup::EX, 1),
            5 => (HrGroup::EC, 7),
            _ => (HrGroup::EC, 4),
        };

        let owner = owner.update().expect("Unable to update person");

        let ownership = NewOrgOwnership::new(
            owner.id,
            ot.id,
        );

        let _res = OrgOwnership::create(&ownership).unwrap();

        // create team at this level

        let (team_name, num_members) = teams[ind];

        let new_team = NewTeam::new(
            team_name.trim().to_string(), 
            format!("{}_FR", team_name.trim()),
            org.id, 
            ot.id, 
            "Description_EN".to_string(), 
            "Description_FR".to_string()
        );

        let team = Team::create(&new_team).expect("Unable to create team");
        
        // if owner, also set management role for team at that tier

        let nr = NewRole::new(
            owner.id, 
            team.id, 
            ot.name_en.clone(), 
            ot.name_fr.clone(), 
            0.80, 
            true,
            grp,
            lvl,
            chrono::Utc::now().naive_utc(), 
            None
        );

        let role_res = Role::create(&nr).unwrap();

        // Set up tasks from this manager
        // Could base this on the managers skills, but too much detail for now
        let sd: SkillDomain = rand::random();

        let subjects = Skill::get_by_domain(sd)
                .expect("Unable to get skills");

        let mut tasks = Vec::new();

        // Generate tasks for team based on skills under chosen domain
        for _ in 0..=8 {

            let subject = subjects
                .choose(&mut rng)
                .unwrap()
                .clone();

            let task = generate_tasks(
                &mut rng,
                &sd,
                &subject.name_en, 
                &role_res.id,
                ot.tier_level
            ).unwrap();

            tasks.push(task);
            
        }

        // Set team ownership

        let new_team_ownership = NewTeamOwnership::new(
            owner.id,
            team.id,
            chrono::Utc::now().naive_utc(),
            None,
        );

        let _to = TeamOwnership::create(&new_team_ownership).expect("Unable to create ownership");

        // Populate the rest of the team, assigning roles at random

        for _i in 0..num_members.min(people.len()) {
            let person = people.pop().unwrap();

            let role = *roles.choose(&mut rng).unwrap();

            let grp: HrGroup = rand::random();

            let nr = NewRole::new(
                person.id, 
                team.id, 
                role.trim().to_string(), 
                format!("{}_FR", role.trim()), 
                0.80, 
                true,
                grp,
                rng.gen_range(2..8),
                chrono::Utc::now().naive_utc(), 
                None
            );

            let role_res = Role::create(&nr).unwrap();

            // Assign work to the roles based on the team's tasks

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

                let _work = Work::create(&nw)
                    .expect("Unable to create work");
            }


        }    
    }

    // Create Publications and Assign Contributors
    println!("Pre-populating Publications and contributors");

    let _res = generate_dummy_publications_and_contributors(&science_org_ids)
        .expect("Unable to create publications and contributors");

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