use async_graphql::*;

use crate::models::{Task};
use uuid::Uuid;

/*
use crate::common_utils::{RoleGuard, is_admin, UserRole};
*/

#[derive(Default)]
pub struct TaskQuery;

#[Object]
impl TaskQuery {

    // Task 
    #[graphql(name = "allTasks")]
    /// Accepts argument of "count" and returns a vector of {count} tasks ordered by
    /// family name.D
    pub async fn all_tasks(
        &self, 
        _context: &Context<'_>,
    ) -> Result<Vec<Task>> {

        Task::get_all()
    }

    #[graphql(name = "Tasks")]
    /// Accepts argument of "count" and returns a vector of {count} tasks ordered by
    /// family name
    pub async fn get_tasks(
        &self, 
        _context: &Context<'_>,
        count: i64,
    ) -> Result<Vec<Task>> {

        Task::get_count(count)
    }

    #[graphql(name = "taskById")]
    pub async fn task_by_id(
        &self, 
        _context: &Context<'_>,
        id: Uuid
    ) -> Result<Task> {

        Task::get_by_id(&id)
    }

    #[graphql(name = "taskByName")]
    pub async fn task_by_name(
        &self, 
        _context: &Context<'_>,
        name: String,
    ) -> Result<Vec<Task>> {

        Task::get_by_title(&name)
    }
}