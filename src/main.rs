use crate::parsio::ParsioClient;
use crate::reservation::Reservation;
use crate::task::{RelativeDate, Task};
use crate::todoist::TodoistClient;
use anyhow::{bail, Result};
use chrono::{Days, Utc};
use clokwerk::{Scheduler, TimeUnits};
use std::time::Duration;
use std::{env, thread};

mod parsio;
mod reservation;
mod task;
mod todoist;

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.every(15.minutes()).run(|| {
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

    let reservations = parsio
        .get_mailbox()?
        .into_iter()
        .map(|doc| doc.try_into().unwrap())
        .filter(|res: &Reservation| res.check_in > Utc::now().date_naive())
        .collect::<Vec<_>>();

    println!("Got {} reservations", reservations.len());
    for res in reservations {
        let parent_task = Task::new(
            &format!("{} - {}", res.guest, res.get_duration()),
            &res,
            RelativeDate::AfterCheckout(Days::new(3)),
            None,
            None,
            false,
        )?;
        let sub_tasks = get_sub_tasks(&res)?;

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

fn get_sub_tasks(reservation: &Reservation) -> Result<Vec<Task>> {
    let alice_id = env::var("TODOIST_ID_ALICE").expect("TODOIST_ID_ALICE is not set");
    let bob_id = env::var("TODOIST_ID_BOB").expect("TODOIST_ID_BOB is not set");

    let subtasks = vec![
        Task::new(
            "Bestill vaskehjelp",
            reservation,
            RelativeDate::Immediately,
            None,
            Some("vaskehjelp"),
            true,
        ),
        Task::new(
            "Fiks egen overnatting",
            reservation,
            RelativeDate::Immediately,
            Some(alice_id),
            Some("overnatting"),
            true,
        ),
        Task::new(
            "Fiks egen overnatting",
            reservation,
            RelativeDate::Immediately,
            Some(bob_id),
            Some("overnatting"),
            true,
        ),
        Task::new(
            "Opprett dørkode",
            reservation,
            RelativeDate::BeforeCheckIn(Days::new(3)),
            None,
            Some("yale"),
            true,
        ),
        Task::new(
            "Klargjør leiligheten",
            reservation,
            RelativeDate::BeforeCheckIn(Days::new(1)),
            None,
            None,
            true,
        ),
        Task::new(
            "Send velkomstmelding",
            reservation,
            RelativeDate::BeforeCheckIn(Days::new(1)),
            None,
            None,
            true,
        ),
        Task::new(
            "Slett dørkode",
            reservation,
            RelativeDate::AfterCheckout(Days::new(2)),
            None,
            Some("yale"),
            true,
        ),
        Task::new(
            "Følg opp anmeldelse",
            reservation,
            RelativeDate::AfterCheckout(Days::new(3)),
            None,
            None,
            true,
        ),
    ];

    subtasks.into_iter().collect()
}
