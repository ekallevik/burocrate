use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use std::fmt::{Display, Formatter};
use string_builder::Builder;

pub struct Reservation {
    pub guest: String,
    pub origin: String,
    pub number_of_guests: String,
    pub check_in: NaiveDate,
    pub checkout: NaiveDate,
    pub comment: String,
    pub guest_paid: String,
    pub host_payout: String,
    pub confirmation_code: String,
}

impl Reservation {
    pub fn get_description(&self) -> Result<String> {
        let mut builder = Builder::default();
        builder.append(format!("- **Gjester**: {}\n", &self.number_of_guests));
        builder.append(format!("- **Innsjekk**: {}\n", &self.check_in));
        builder.append(format!("- **Utsjekk**: {}\n", &self.checkout));
        builder.append(format!("- **Bosted**: {}\n", &self.origin));
        builder.append(format!("- **Payout**: {}\n", &self.host_payout));
        builder.append(format!("- **Confirmation**: {}\n", &self.confirmation_code));
        Ok(builder.string()?)
    }

    pub fn get_duration(&self) -> String {
        let check_in = match self.check_in.month() == self.checkout.month() {
            true => self.check_in.format("%-d."),
            false => self.check_in.format("%-d. %b"),
        };

        format!("{} til {}", check_in, self.checkout.format("%-d. %b"))
    }
}

impl Display for Reservation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} has booked {} through {} for {}. Payout {}",
            self.guest, self.check_in, self.checkout, self.number_of_guests, self.host_payout
        )
    }
}
