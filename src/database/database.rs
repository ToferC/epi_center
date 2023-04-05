use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2::{self};
use std::env;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::database::pre_populate_db_schema;
use crate::database::pre_populate_skills;
use crate::errors::error_handler::CustomError;

pub type PostgresPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

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

            let _res = pre_populate_skills().expect("error in populating skills");

            println!("Pre-populating database");
            let _res = pre_populate_db_schema()
                .expect("Unable to pre-populate database");

            //populate_db_with_demo_data();
        }
    }
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}

/// Testing function to generate dummy data when resetting the database
/// Started adding unique names to countries, so only works once when DB is reset.
pub fn populate_db_with_demo_data() {

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