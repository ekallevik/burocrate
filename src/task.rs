use crate::reservation::Reservation;
use crate::todoist::TodoistTask;
use anyhow::Result;
use chrono::{Days, NaiveDate, Utc};
use std::fmt::{Display, Formatter};

pub struct Task {
    title: String,
    description: String,
    due_date: RelativeDate,
    assigned_to: Option<String>,
    labels: Vec<String>,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.assigned_to {
            None => write!(f, "Task: '{}' due on {}", self.title, self.due_date),
            Some(assignee) => write!(
                f,
                "Task: '{}' due on {} and assigned to {}",
                self.title, self.due_date, assignee
            ),
        }
    }
}

impl Task {
    pub fn new(
        title: &str,
        reservation: &Reservation,
        due_date: RelativeDate,
        assigned_to: Option<String>,
        label: Option<&str>,
        appendix: bool,
    ) -> Result<Self> {
        let processed_titled = match appendix {
            false => title.to_string(),
            true => format!("{} - {}", title, reservation.guest),
        };

        let labels = vec![Some("airbnb"), label]
            .into_iter()
            .flatten()
            .map(|a| a.to_string())
            .collect::<Vec<_>>();

        Ok(Task {
            title: processed_titled,
            description: reservation.get_description()?,
            due_date,
            assigned_to,
            labels,
        })
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
            &(self.title.clone() + " for " + &reservation.guest),
            self.due_date.resolve(reservation),
            Some(self.description.clone()),
            self.assigned_to.clone(),
            self.labels.clone(),
        )
    }
}

#[allow(dead_code)]
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

impl Display for RelativeDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeDate::Immediately => write!(f, "immediately"),
            RelativeDate::BeforeCheckIn(d) => write!(f, "{:?} before check-in", d),
            RelativeDate::AfterCheckIn(d) => write!(f, "{:?} after check-in", d),
            RelativeDate::BeforeCheckout(d) => write!(f, "{:?} before checkout", d),
            RelativeDate::AfterCheckout(d) => write!(f, "{:?} after checkout", d),
        }
    }
}
