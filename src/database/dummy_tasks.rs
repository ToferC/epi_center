use async_graphql::Error;
use rand::{seq::SliceRandom, rngs::ThreadRng};
use uuid::Uuid;

use crate::models::{NewTask, Task, SkillDomain, WorkStatus};

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