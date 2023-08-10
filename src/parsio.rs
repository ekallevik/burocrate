use crate::reservation::Reservation;
use anyhow::{Context, Result};
use date_time_parser::DateParser;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
use std::io::Read;

pub struct ParsioClient {
    api_key: String,
    mailbox_id: String,
}

impl ParsioClient {
    pub fn new() -> Self {
        ParsioClient {
            api_key: env::var("PARSIO_API_KEY").expect("PARSIO_API_KEY is not set"),
            mailbox_id: env::var("PARSIO_MAILBOX_ID").expect("PARSIO_MAILBOX_ID is not set"),
        }
    }

    pub fn get_mailbox(&self) -> Result<Vec<ParsioDoc>> {
        let url = format!("https://api.parsio.io/mailboxes/{}/parsed", self.mailbox_id);

        let mut res = Client::new()
            .get(url)
            .header("Accept", "application/json")
            .header("User-Agent", "burocrate")
            .header("X-API-Key", &self.api_key)
            .send()?;

        let mut body = String::new();
        res.read_to_string(&mut body)?;

        let response: ParsioResponse = serde_json::from_str(&body)?;

        Ok(response.docs)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParsioResponse {
    limit: u64,
    page: u64,
    pub docs: Vec<ParsioDoc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParsioDoc {
    #[serde(rename = "Guest Full Name")]
    guest_name: String,
    #[serde(rename = "Guest Location")]
    guest_location: String,
    #[serde(rename = "Booking Check In Date")]
    check_in: String,
    #[serde(rename = "Booking Checkout Date")]
    checkout: String,
    #[serde(rename = "Booking Comment")]
    booking_comment: String,
    #[serde(rename = "Booking Guests Nb")]
    number_of_guests: String,
    #[serde(rename = "Booking Confirmation Code")]
    confirmation_code: String,
    #[serde(rename = "Guest Paid Total")]
    guest_paid_total: String,
    #[serde(rename = "Host Payout Total")]
    host_payout: String,
    received_at_datetime: String,
}

impl Display for ParsioDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} has booked {} to {} for {}. Payout {}",
            self.guest_name, self.check_in, self.checkout, self.number_of_guests, self.host_payout
        )
    }
}

impl TryInto<Reservation> for ParsioDoc {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<Reservation, Self::Error> {
        let check_in =
            DateParser::parse(self.check_in.as_str()).context("Could not parse a date")?;
        let checkout =
            DateParser::parse(self.checkout.as_str()).context("Could not parse a date")?;

        Ok(Reservation {
            guest: self.guest_name,
            origin: self.guest_location,
            number_of_guests: self.number_of_guests,
            check_in,
            checkout,
            comment: self.booking_comment,
            guest_paid: self.guest_paid_total,
            host_payout: self.host_payout,
            confirmation_code: self.confirmation_code,
        })
    }
}
