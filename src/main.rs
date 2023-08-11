use crate::parsio::ParsioClient;
use crate::reservation::Reservation;
use crate::task::{RelativeDate, Task};
use crate::todoist::TodoistClient;
use anyhow::{bail, Result};
use chrono::Days;
use std::env;

mod parsio;
mod reservation;
mod task;
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

        let description = res.get_description()?;
        let parent_task = Task::new(
            &format!("Booking fra {}", res.check_in),
            &description,
            RelativeDate::AfterCheckout(Days::new(3)),
        );

        let todoist_parent_task = parent_task.to_todoist(&res, None, &todoist_project_id);

        let response = todoist.post_task(todoist_parent_task)?;
        let todoist_parent_task_id = match response.id {
            None => bail!("Missing Todoist parent ID"),
            Some(id) => id,
        };

        let sub_tasks = vec![
            Task::new(
                "Bestill vaskehjelp",
                &description,
                RelativeDate::Immediately,
            ),
            Task::new(
                "Fiks egen overnatting Even",
                &description,
                RelativeDate::Immediately,
            ),
            Task::new(
                "Fiks egen overnatting Kristin",
                &description,
                RelativeDate::Immediately,
            ),
            Task::new(
                "Opprett dørkode",
                &description,
                RelativeDate::BeforeCheckIn(Days::new(3)),
            ),
            Task::new(
                "Klargjør leiligheten",
                &description,
                RelativeDate::BeforeCheckIn(Days::new(1)),
            ),
            Task::new(
                "Send velkomstmelding",
                &description,
                RelativeDate::BeforeCheckIn(Days::new(1)),
            ),
            Task::new(
                "Slett dørkode",
                &description,
                RelativeDate::AfterCheckout(Days::new(2)),
            ),
            Task::new(
                "Følg opp anmeldelse",
                &description,
                RelativeDate::AfterCheckout(Days::new(3)),
            ),
        ];

        for sub_task in sub_tasks {
            let todoist_sub_task =
                sub_task.to_todoist(&res, Some(&todoist_parent_task_id), &todoist_project_id);
            todoist.post_task(todoist_sub_task)?;
        }
    }

    Ok(())
}
