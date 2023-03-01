
use rand::Rng;
use rand::distributions::Standard;
use rand::{thread_rng, seq::SliceRandom};

use crate::models::{Person, Organization, NewPerson, NewOrganization, 
    Role, NewRole, Team, NewTeam, OrgTier, NewOrgTier, OrgOwnership, NewOrgOwnership,
    TeamOwnership, NewTeamOwnership, HrGroup};

use super::create_fake_capabilities_for_person;

/// Creates basic Org, People, Teams, Roles, Work, etc in the database
pub fn pre_populate_db_schema() {

    // Set up Organization

    let o = NewOrganization::new(
        "Public Health Agency of Canada".to_string(),
        "Agence de Sante Public de Canada".to_string(),
        "PHAC".to_string(),
        "ASPC".to_string(),
        "Government".to_string(),
    );

    let org = Organization::create(&o).expect("Unable to create new organization");

    // Set up Org Tiers

    let mut org_tiers: Vec<OrgTier> = Vec::new();

    let tt = NewOrgTier::new(
            org.id, 
            1, 
            "President".to_string(), 
            "President".to_string(), 
            None);

    let top_tier = OrgTier::create(&tt).unwrap();

    // Second Tier

    let adm = NewOrgTier::new(
        org.id, 
        2, 
        "VP CDSB".to_string(), 
        "VP CDSB".to_string(), 
        Some(top_tier.id),
    );

    let adm_tier = OrgTier::create(&adm).unwrap();

    // Third Tier
    
    let dg = NewOrgTier::new(
        org.id, 
        3, 
        "DG DMIA".to_string(), 
        "DG DMIA".to_string(), 
        Some(adm_tier.id),
    );

    let dg_tier = OrgTier::create(&dg).unwrap();

    let dg2 = NewOrgTier::new(
        org.id, 
        3, 
        "DG SDPI".to_string(), 
        "DG SDPI".to_string(), 
        Some(adm_tier.id),
    );

    let dg_tier2 = OrgTier::create(&dg2).unwrap();

    // Fourth Tier

    let d1 = NewOrgTier::new(
        org.id, 
        4, 
        "DIR Data Science".to_string(), 
        "DIR Science de Donnees".to_string(), 
        Some(dg_tier.id),
    );

    let dir1 = OrgTier::create(&d1).unwrap();

    let d2 = NewOrgTier::new(
        org.id, 
        4, 
        "DIR Data Policy".to_string(), 
        "DIR Politique de Donnees".to_string(), 
        Some(dg_tier.id),
    );

    let dir2 = OrgTier::create(&d2).unwrap();


    let d3 = NewOrgTier::new(
        org.id, 
        4, 
        "DIR Data Partnerships".to_string(), 
        "DIR Partenariat de Donnees".to_string(), 
        Some(dg_tier2.id),
    );

    let dir3 = OrgTier::create(&d3).unwrap();

    let d4 = NewOrgTier::new(
        org.id, 
        4, 
        "DIR Strategic Policy".to_string(), 
        "DIR Politique Strategique".to_string(), 
        Some(dg_tier2.id),
    );

    let dir4 = OrgTier::create(&d4).unwrap();

    // Fifth Tier

    let m1 = NewOrgTier::new(
        org.id, 
        5, 
        "MGR Data Ingestion".to_string(), 
        "MGR Data Ingestion".to_string(), 
        Some(dir1.id),
    );

    let man1 = OrgTier::create(&m1).unwrap();

    let m2 = NewOrgTier::new(
        org.id, 
        5, 
        "MGR Data Mgt".to_string(), 
        "MGR Data Mgt".to_string(), 
        Some(dir1.id),
    );

    let man2 = OrgTier::create(&m2).unwrap();

    let m3 = NewOrgTier::new(
        org.id, 
        5, 
        "MGR Data Ethics".to_string(), 
        "MGR Data Ethics".to_string(), 
        Some(dir2.id),
    );

    let man3 = OrgTier::create(&m3).unwrap();

    let m4 = NewOrgTier::new(
        org.id, 
        5, 
        "MGR Data Governance".to_string(), 
        "MGR Data Governance".to_string(), 
        Some(dir2.id),
    );

    let man4 = OrgTier::create(&m4).unwrap();


    let m5 = NewOrgTier::new(
        org.id, 
        5, 
        "MGR Internal Partnerships".to_string(), 
        "MGR Internal Partnerships".to_string(), 
        Some(dir3.id),
    );

    let man5 = OrgTier::create(&m5).unwrap();

    let m6 = NewOrgTier::new(
        org.id, 
        5, 
        "MGR International Partnerships".to_string(), 
        "MGR International Partnerships".to_string(), 
        Some(dir3.id),
    );

    let man6 = OrgTier::create(&m6).unwrap();

    let m7 = NewOrgTier::new(
        org.id, 
        5, 
        "MGR MCs and TBsubs".to_string(), 
        "MGR MCs and TBsubs".to_string(), 
        Some(dir4.id),
    );

    let man7 = OrgTier::create(&m7).unwrap();

    let m8 = NewOrgTier::new(
        org.id, 
        5, 
        "MGR New Public Health".to_string(), 
        "MGR New Public Health".to_string(), 
        Some(dir4.id),
    );

    let man8 = OrgTier::create(&m8).unwrap();

    org_tiers.push(top_tier);
    org_tiers.push(adm_tier);
    org_tiers.push(dg_tier);
    org_tiers.push(dg_tier2);
    org_tiers.push(dir1);
    org_tiers.push(dir2);
    org_tiers.push(dir3);
    org_tiers.push(dir4);
    org_tiers.push(man1);
    org_tiers.push(man2);
    org_tiers.push(man3);
    org_tiers.push(man4);
    org_tiers.push(man5);
    org_tiers.push(man6);
    org_tiers.push(man7);
    org_tiers.push(man8);

    // Set up Persons
    let mut rng = rand::thread_rng();

    let mut people: Vec<Person> = Vec::new();

    let path = "names.csv";

    let mut reader = csv::Reader::from_path(path).unwrap();

    for r in reader.records() {

        let record = r.unwrap();

        let gn: String = String::from(&record[0]);
        let famn: String = String::from(&record[1]);

        let grp: HrGroup = rand::random();
        
        let p = NewPerson::new(
            uuid::Uuid::new_v4(),
            famn.to_owned(),
            gn.to_owned(),
            format!("{}.{}@phac-aspc.gc.ca", &gn, &famn).to_lowercase(),
            gen_rand_number(),
            "130 Colonnade Rd".to_string(),
            "Ottawa".to_string(),
            "Ontario".to_string(),
            "K2E 7K3".to_string(),
            org.id,
            gen_rand_number(),
            gen_rand_number(),
        );

        let person = Person::create(&p).expect("Unable to create person");

        let _capabilities_for_person = create_fake_capabilities_for_person(person.id);

        people.push(person);
    } 

    // Set up Teams and roles data

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

        let _res = Role::create(&nr).unwrap();

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

            let _res = Role::create(&nr).unwrap();

        }
    }
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