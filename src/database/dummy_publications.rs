use async_graphql::Error;
use rand::seq::SliceRandom;
use uuid::Uuid;
use chrono::{Duration, Utc};
use rand::Rng;

use crate::models::{Capability, CapabilityLevel, Person, Publication, PublicationContributor, SkillDomain, 
    NewPublication, PublicationStatus, NewPublicationContributor};

pub fn generate_dummy_publications_and_contributors(science_org_ids: &Vec<Uuid>) -> Result<(), Error> {
    // get vec of people with science capabilties

    let mut rng = rand::thread_rng();

    let scientist_capabilities = Capability::get_by_domain_and_level(&SkillDomain::Engineering, CapabilityLevel::Expert)?;

    let mut scientist_ids = Vec::new();
    for capability in &scientist_capabilities {
        let person = Person::get_by_id(&capability.person_id)?;
        scientist_ids.push(person.id);
    };

    // create 20 publications and choose lead_authors and publishing_organizations

    let mut publications = Vec::new();

    
    for i in 0..20 {

        // Choose a scientist
        let scientist_id = scientist_ids.pop().unwrap();

        let new_publication = NewPublication::new(
            *science_org_ids.choose(&mut rng).unwrap(),
            scientist_id,
            format!("Publication on {} - {}", &scientist_capabilities[i].name_en, i),
            format!("On {} - {}", scientist_capabilities[i].name_en, i),
            PublicationStatus::Published,
            Some("https://journalofepidemiology.com".to_string()),
            Some("123456789".to_string()),
            Some(Utc::now().naive_utc() - Duration::days(rng.gen_range(1..100))),
            Some(Utc::now().naive_utc() + Duration::days(rng.gen_range(1..100))),
        );

        let publication = Publication::create(&new_publication)?;

        let new_contributor = NewPublicationContributor::new(
            publication.id,
            scientist_id,
            "Lead Author".to_string(),
        );

        PublicationContributor::create(&new_contributor)?;

        publications.push(publication);
    }

    // Assign the rest of the people to the publications as contributors

    for scientist in scientist_ids {
            
        let publication = publications.choose(&mut rng).unwrap();

        let new_contributor = NewPublicationContributor::new(
            publication.id,
            scientist,
            "Contributor".to_string(),
        );

        PublicationContributor::create(&new_contributor)?;
    }

    Ok(())
}