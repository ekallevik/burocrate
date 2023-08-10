use std::env;
use anyhow::Result;
use crate::parsio::ParsioClient;
use crate::todoist::{TodoistTask, TodoistClient};

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
            doc.get_title().as_str(),
            None,
            doc.get_description().ok(),
            None,
        );
        let response = todoist.post_task(task)?;
        if let Some(taskId) = response.id {
            println!(" - taskId={taskId}")
        }
    }

    Ok(())
}
