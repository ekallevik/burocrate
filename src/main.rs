use crate::parsio::ParsioClient;
use crate::todoist::{TodoistClient, TodoistTask};
use anyhow::Result;
use chrono::Days;
use std::env;

mod parsio;
mod todoist;

fn main() -> Result<()> {
    let todoist = TodoistClient::new();
    let todoist_project_id = env::var("TODOIST_PROJECT_ID").expect("TODOIST_PROJECT_ID is not set");
    let parsio = ParsioClient::new();

    let docs = parsio.get_mailbox()?;

    println!("Got {} reservations", docs.len());
    for doc in docs {
        println!("{}", doc);
        let task = TodoistTask::new(
            &None,
            &todoist_project_id,
            doc.get_title()?.as_str(),
            doc.get_checkout()?.checked_add_days(Days::new(3)),
            doc.get_description().ok(),
            None,
        );

        let response = todoist.post_task(task)?;
        if let Some(task_id) = response.id {
            println!(" - taskId={task_id}")
        }
    }

    Ok(())
}
