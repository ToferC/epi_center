use async_graphql::Error;
use rand::{seq::SliceRandom, rngs::ThreadRng, Rng};
use uuid::Uuid;

use crate::models::{NewTask, Task, SkillDomain, WorkStatus, NewRequirement, CapabilityLevel, HrGroup};

/// Generate dummy tasks based on some baseline data about the org
pub fn generate_tasks(
    rng: &mut ThreadRng,
    domain: &SkillDomain, 
    subject: &str, 
    creating_role_id: &Uuid,
    tier_level: i32,
) -> Result<Task, Error> {

    let work_nouns: Vec<&str> = "
        briefing note; PowerPoint; Jupyter Notebook; memo; 
        white paper; plan; project documentation; outline; 
        data; research paper; meeting minutes; governance review; 
        genomic data
    ".split("; ").collect();

    let outcome: Vec<&str> = "
        socialize; inform; secure decision; inform action; communicate; 
        create policy; respond to inquiry, respond to audit; manage; 
    ".split("; ").collect();

    let title = format!("{} on {}", 
        work_nouns.choose(rng).unwrap().clone().trim().to_string(),
        subject
    );

    let nt = NewTask::new(
        *creating_role_id,
        title,
        *domain,
        outcome.choose(rng).unwrap().to_string(),
        tier_level,
        "https://www.phac-aspc.ca/some_url".to_string(),
        chrono::Utc::now().naive_utc(),
        chrono::Utc::now().naive_utc(),
        WorkStatus::InProgress,
    );

    let task = Task::create(&nt);

    task
}

/// Generate requirement for a role based on a provided skilldomain
pub fn generate_requirement(role_id: Uuid, skill_id: Uuid, hr_group: HrGroup, hr_level: i32, rng: &mut impl Rng) -> NewRequirement {
    // Add requirements for each role based on the team Primary Domain

    let req_level: CapabilityLevel;

    if  hr_group == HrGroup::EX || hr_group == HrGroup::DM {
        req_level = CapabilityLevel::Expert
    } else {

        // Allow for random changes
        let hr_level = hr_level + rng.gen_range(-2..=2);

        req_level = match hr_level {
            0..=1 => CapabilityLevel::Desired,
            2..=3 => CapabilityLevel::Novice,
            4..=6 => CapabilityLevel::Experienced,
            7..=8 => CapabilityLevel::Expert,
            9..=10 => CapabilityLevel::Specialist,
            _ => CapabilityLevel::Experienced,
        };
    }

    NewRequirement::new(
        role_id,
        skill_id,
        req_level,
    )
}