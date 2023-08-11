use crate::reservation::Reservation;
use crate::todoist::TodoistTask;
use chrono::{Days, NaiveDate, Utc};

pub struct Task {
    name: String,
    description: String,
    due_date: RelativeDate,
}

impl Task {
    pub fn new(name: &str, description: &str, due_date: RelativeDate) -> Self {
        Task {
            name: name.to_string(),
            description: description.to_string(),
            due_date,
        }
    }

    pub fn to_todoist(
        &self,
        reservation: &Reservation,
        parent_task_id: Option<&String>,
        project_id: &str,
    ) -> TodoistTask {
        TodoistTask::new(
            parent_task_id,
            project_id,
            &(self.name.clone() + " for " + &reservation.guest),
            self.due_date.resolve(reservation),
            Some(self.description.clone()),
            None,
        )
    }
}

pub enum RelativeDate {
    Immediately,
    BeforeCheckIn(Days),
    AfterCheckIn(Days),
    BeforeCheckout(Days),
    AfterCheckout(Days),
}

impl RelativeDate {
    fn resolve(&self, reservation: &Reservation) -> Option<NaiveDate> {
        match self {
            RelativeDate::Immediately => Some(Utc::now().date_naive()),
            RelativeDate::BeforeCheckIn(days) => reservation.check_in.checked_sub_days(*days),
            RelativeDate::AfterCheckIn(days) => reservation.check_in.checked_add_days(*days),
            RelativeDate::BeforeCheckout(days) => reservation.checkout.checked_sub_days(*days),
            RelativeDate::AfterCheckout(days) => reservation.checkout.checked_add_days(*days),
        }
    }
}
