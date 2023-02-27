use std::{io::stdin};
use chrono::prelude::*;
use chrono::Duration;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2::{self};
use rand::Rng;
use std::env;
use uuid::Uuid;
use rand::{thread_rng, seq::SliceRandom};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::errors::error_handler::CustomError;

pub type PostgresPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

use crate::models::{Person, Organization, NewPerson, NewOrganization, 
    Role, NewRole, Team, NewTeam, OrgTier, NewOrgTier, OrgOwnership, NewOrgOwnership,
    TeamOwnership, NewTeamOwnership};
use crate::models::{User, UserData, InsertableUser};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

lazy_static! {
    pub static ref POOL: PostgresPool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        PostgresPool::new(manager).expect("Failed to create DB Pool")
    };
}

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn init() {

    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get DB connection");
    run_migration(&mut conn);

    // Auto-add admin if does not exist
    let admin_name = env::var("ADMIN_NAME").expect("Unable to load admin name");
    let admin_email = env::var("ADMIN_EMAIL").expect("Unable to load admin email");
    let admin_pwd = env::var("ADMIN_PASSWORD").expect("Unable to load admin password");
    
    let admin = User::get_by_email(&admin_email);

    match admin {
        // Checking admin and if not, add default data structures
        Ok(u) => println!("Admin exists {:?} - bypass setup", &u),
        Err(_e) => {

            let admin_data = UserData {
                name: admin_name.trim().to_owned(),
                email: admin_email.trim().to_owned(),
                password: admin_pwd.trim().to_owned(),
                role: "ADMIN".to_owned(),
            };
        
            let test_admin = InsertableUser::from(admin_data);
        
            let admin = User::create(test_admin)
                .expect("Unable to create admin");
        
            println!("Admin created: {:?}", &admin);

            pre_populate_db_schema();

            populate_db_with_demo_data(&conn);
        }
    }
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}

/// Creates basic Org, People, Teams, Roles, Work, etc in the database
pub fn pre_populate_db_schema() {

    // Set up Organization
    let mut conn = connection().unwrap();

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

    org_tiers.push(top_tier);
    org_tiers.push(adm_tier);
    org_tiers.push(dg_tier);
    org_tiers.push(dg_tier2);
    org_tiers.push(dir1);
    org_tiers.push(dir2);
    org_tiers.push(man1);
    org_tiers.push(man2);
    org_tiers.push(man3);
    org_tiers.push(man4);

    // Set up Persons
    let mut rng = rand::thread_rng();

    let mut people: Vec<Person> = Vec::new();

    let path = "names.csv";

    let mut reader = csv::Reader::from_path(path).unwrap();

    for r in reader.records() {

        let record = r.unwrap();

        let gn: String = String::from(&record[0]);
        let famn: String = String::from(&record[1]);
        
        let p = NewPerson::new(
            uuid::Uuid::new_v4(),
            famn,
            gn,
            org.id,
            rng.gen_range(100000000..999999999).to_string(),
        );

        let person = Person::create(&p).expect("Unable to create person");

        people.push(person);
    } 

    // Set up Teams

    let teams = vec!["PMO", "VPO", "DGO DMIA", "DGO SPDI", "DO-Data Science",
        "DO-Data Policy", "Data Ingestion", "Data Management", "Data Ethics", "Data Governance"];

    
    // Set up OrgTierOwnership

    for ot in org_tiers {
        // allocate people to org tiers

        // if owner, also set management role for team at that tier

        // remove from vec using pop
    }


    // Set up Teams


    // Set up Roles


    // Set up Team Ownership


    // Set
    /*
    // Set up countries
    
    let mut new_countries: Vec<NewCountry> = Vec::new();

    new_countries.push(NewCountry::new("United Kingdom".to_string(), 0.05));
    new_countries.push(NewCountry::new("Canada".to_string(), 0.03));
    new_countries.push(NewCountry::new("Singapore".to_string(), 0.02));
    new_countries.push(NewCountry::new("USA".to_string(), 0.04));
    new_countries.push(NewCountry::new("France".to_string(), 0.03));
    new_countries.push(NewCountry::new("Brazil".to_string(), 0.06));
   
    let mut countries: Vec<Country> = Vec::new();

    for np in new_countries {
        let c = Country::create(conn, &np).unwrap();
        countries.push(c);
    };

    // Set up places
    let mut new_places:Vec<NewPlace> = Vec::new();
    new_places.push(NewPlace::new("London".to_string(), countries[0].id));
    new_places.push(NewPlace::new("Singapore".to_string(), countries[2].id));
    new_places.push(NewPlace::new("Florida".to_string(), countries[3].id));
    new_places.push(NewPlace::new("Paris".to_string(), countries[4].id));
    new_places.push(NewPlace::new("Chicago".to_string(), countries[3].id));
    new_places.push(NewPlace::new("Rio".to_string(), countries[5].id));
    new_places.push(NewPlace::new("New York".to_string(), countries[3].id));
    new_places.push(NewPlace::new("Ottawa".to_string(), countries[1].id));
    new_places.push(NewPlace::new("Montreal".to_string(), countries[1].id));
    new_places.push(NewPlace::new("Vancouver".to_string(), countries[1].id));
    new_places.push(NewPlace::new("Calgary".to_string(), countries[1].id));
    new_places.push(NewPlace::new("Toronto".to_string(), countries[1].id));

    for np in new_places {
        let _p = Place::create(conn, &np).unwrap();
    };

    // Add Vaccines
    let mut new_vaccines = Vec::new();

    let approved_on: NaiveDate = Utc.ymd(2021, 09, 21).naive_utc();

    new_vaccines.push(
        NewVaccine::new(
            "Comirnaty".to_string(),
            "Phizer".to_string(),
            "mRNA".to_string(),
            2,
            true,
            approved_on,
            "XXX YYY".to_string()
    ));

    new_vaccines.push(
        NewVaccine::new(
            "SpikeVax".to_string(),
            "Moderna".to_string(),
            "mRNA".to_string(),
            2,
            true,
            approved_on,
            "XXX YYY".to_string()
    ));

    new_vaccines.push(
        NewVaccine::new(
            "Vaxzeria".to_string(),
            "AstraZeneca".to_string(),
            "Viral Vector-based".to_string(),
            2,
            true,
            approved_on,
            "XXX YYY".to_string()
    ));

    new_vaccines.push(
        NewVaccine::new(
            "Jannsen".to_string(),
            "Johnson & Johnson".to_string(),
            "mRNA".to_string(),
            1,
            true,
            approved_on,
            "XXX YYY".to_string()
    ));

    for v in new_vaccines {
        let _res = Vaccine::create(conn, &v).unwrap();
    }
     */
}

/// Create an administrative user. An admin account is needed to create additional users and access
/// some guarded mutations.
pub fn create_admin_user(conn: &PgConnection) {

        println!("What is the administrator's name?");

        let mut name_input = String::new();
        stdin().read_line(&mut name_input).expect("Unable to read name");

        println!("What is the administrator's email address?");

        let mut email_input = String::new();
        stdin().read_line(&mut email_input).expect("Unable to read email");

        println!("Enter the administrator password?");

        let mut password_input = String::new();
        stdin().read_line(&mut password_input).expect("Unable to read password");
        
        let admin_data = UserData {
            name: name_input.trim().to_owned(),
            email: email_input.trim().to_owned(),
            password: password_input.trim().to_owned(),
            role: "ADMIN".to_owned(),
        };
    
        let mut test_admin = InsertableUser::from(admin_data);
    
        test_admin.role = "ADMIN".to_owned();
    
        let admin = User::create(test_admin)
            .expect("Unable to create admin");
    
        println!("Admin created: {:?}", &admin);
}

/// Testing function to generate dummy data when resetting the database
/// Started adding unique names to countries, so only works once when DB is reset.
pub fn populate_db_with_demo_data(conn: &PgConnection) {

    /*s
    // Set up RNG
    let mut rng = thread_rng();

    // Load country, place and vaccine data
    
    let country_hash = Country::load_into_hash(&conn);

    let countries = country_hash.values().cloned().collect::<Vec<Country>>();

    let canada_id = *&country_hash.iter().find(|h| h.1.country_name == "Canada".to_string()).unwrap().0;

    let places_hash = Place::load_into_hash(&conn);

    let mut origins: Vec<Place> = Vec::new();
    let mut destinations: Vec<Place> = Vec::new();

    for (_, p) in places_hash {
        if p.country_id == *canada_id {
            destinations.push(p);
        } else {
            origins.push(p);
        }
    };

    let vaccine_hash = Vaccine::load_into_hash(&conn);
    let vaccines = vaccine_hash.values().cloned().collect::<Vec<Vaccine>>();

    // Populate with fake population data

    for _i in 0..100 {

        let tg = crate::models::NewTravelGroup::new();

        let res = TravelGroup::create_travel_group(conn, &tg);

        let travel_group = res.unwrap();

        for _i in 0..4 {

            let country = countries.choose(&mut rng).unwrap();

            // Create person
            let person = NewPerson::fake(
                country.id,
                travel_group.id,
            );

            let created_p = Person::create(conn, &person).expect("Unable to create person");
                
            // Create trip
            let origin  = origins.choose(&mut rng).unwrap();
            let destination = destinations.choose(&mut rng).unwrap();
            
            let nt = NewTrip::fake(
                &travel_group.id, 
                &created_p.id, 
                &origin.id, 
                &destination.id
            );
            
            let _t = Trip::create(conn, &nt).expect("Unable to create trip");

            // Create public health profile
            let profile = NewPublicHealthProfile::new(
                created_p.id.to_owned(), 
                Some(Uuid::new_v4().to_string()),
            );

            let created_ph_profile = PublicHealthProfile::create(conn, &profile).unwrap();

            // Create vaccinations
            for _i in 0..2 {
                let new_vaccination = NewVaccination::fake(
                    vaccines.choose(&mut rng).unwrap().id, 
                    "local pharmacy".to_string(), 
                    origin.id, 
                    Utc::now().naive_utc() - Duration::days(rng.gen_range(1..90)), 
                    created_ph_profile.id,
                );

                Vaccination::create(conn, &new_vaccination).unwrap();
            }

            // Create COVID Test
            let test_result = rng.gen_bool(country.risk_rate);

            let new_test = NewCovidTest::new(
                created_ph_profile.id, 
                "Test-X01".to_string(), 
                "molecular".to_string(), 
                Utc::now().naive_utc() - Duration::days(rng.gen_range(1..14)), 
                test_result);

            let _c = CovidTest::create(conn, &new_test).expect("Unable to create CovidTest");

            // Create Postal Address
            let quarantine_address = SlimAddress::new(
                "1011 testing street".to_owned(),
                *&destination.id,
                "Default".to_owned(),
                *&destination.country_id,
                "K2L 3F1".to_owned(),
                None,
            );

            let insertable_address = NewPostalAddress::from(quarantine_address);

            let qa = PostalAddress::create(conn, &insertable_address).expect("Unable to insert PostalAddress");

            // Create quarantine plan
            let date_created: NaiveDate = Utc::today().naive_utc() - Duration::days(rng.gen_range(1..14));

            let new_qp = NewQuarantinePlan::new(
                created_ph_profile.id,
                date_created,
                false,
                false,
                *&qa.id,
                false,
            );

            let _r = QuarantinePlan::create(conn, &new_qp).unwrap();

            println!("Demo data insert complete: {}", &travel_group.id);

        }
    }
     */

}