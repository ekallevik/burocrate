use crate::parsio::ParsioClient;
use crate::reservation::Reservation;
use crate::todoist::{TodoistClient, TodoistTask};
use anyhow::Result;
use chrono::Days;
use std::env;

mod parsio;
mod reservation;
mod todoist;

fn main() -> Result<()> {
    let todoist = TodoistClient::new();
    let todoist_project_id = env::var("TODOIST_PROJECT_ID").expect("TODOIST_PROJECT_ID is not set");
    let parsio = ParsioClient::new();

    let docs = parsio.get_mailbox()?;

    println!("Got {} reservations", docs.len());
    for doc in docs {

        let res: Reservation = doc.try_into()?;
        println!("{res}");

        let task = TodoistTask::new(
            &None,
            &todoist_project_id,
            &res.get_title(),
            res.checkout.checked_add_days(Days::new(3)),
            res.get_description().ok(),
            None,
        );

        let response = todoist.post_task(task)?;
        if let Some(task_id) = response.id {
            println!(" - taskId={task_id}")
        }
    }

    Ok(())
}
