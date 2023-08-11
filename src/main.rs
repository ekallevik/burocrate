use crate::parsio::ParsioClient;
use crate::reservation::Reservation;
use crate::task::{RelativeDate, Task};
use crate::todoist::TodoistClient;
use anyhow::{bail, Result};
use chrono::{Days, Utc};
use std::{env, thread};
use std::time::Duration;
use clokwerk::{Job, Scheduler, TimeUnits};

mod parsio;
mod reservation;
mod task;
mod todoist;

fn main() {

    let mut scheduler = Scheduler::new();
    scheduler
        .every(15.minutes())
        .run(|| {
            println!("\n\n --- Time: {} ---", Utc::now());
            run_process().unwrap();
        });

    // run the process forever
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
}

fn run_process() -> Result<()> {

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
            None,
        );
        let sub_tasks = get_sub_tasks(&description);

        let todoist_parent_task = parent_task.to_todoist(&res, None, &todoist_project_id);

        let response = todoist.post_task(todoist_parent_task)?;
        let todoist_parent_task_id = match response.id {
            None => bail!("Missing Todoist parent ID"),
            Some(id) => id,
        };

        for sub_task in sub_tasks {
            let todoist_sub_task =
                sub_task.to_todoist(&res, Some(&todoist_parent_task_id), &todoist_project_id);
            todoist.post_task(todoist_sub_task)?;
        }
    }
    Ok(())
}

fn get_sub_tasks(description: &str) -> Vec<Task> {
    let alice_id = env::var("TODOIST_ID_ALICE").expect("TODOIST_ID_ALICE is not set");
    let bob_id = env::var("TODOIST_ID_BOB").expect("TODOIST_ID_BOB is not set");

    vec![
        Task::new(
            "Bestill vaskehjelp",
            &description,
            RelativeDate::Immediately,
            None,
        ),
        Task::new(
            "Fiks egen overnatting",
            &description,
            RelativeDate::Immediately,
            Some(alice_id),
        ),
        Task::new(
            "Fiks egen overnatting",
            &description,
            RelativeDate::Immediately,
            Some(bob_id),
        ),
        Task::new(
            "Opprett dørkode",
            &description,
            RelativeDate::BeforeCheckIn(Days::new(3)),
            None,
        ),
        Task::new(
            "Klargjør leiligheten",
            &description,
            RelativeDate::BeforeCheckIn(Days::new(1)),
            None,
        ),
        Task::new(
            "Send velkomstmelding",
            &description,
            RelativeDate::BeforeCheckIn(Days::new(1)),
            None,
        ),
        Task::new(
            "Slett dørkode",
            &description,
            RelativeDate::AfterCheckout(Days::new(2)),
            None,
        ),
        Task::new(
            "Følg opp anmeldelse",
            &description,
            RelativeDate::AfterCheckout(Days::new(3)),
            None,
        ),
    ]
}
