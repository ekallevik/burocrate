use std::env;
use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::Read;
use string_builder::Builder;

pub struct ParsioClient {
    api_key: String,
    mailbox_id: String,
}

impl ParsioClient {

    pub fn new() -> Self {
        ParsioClient {
            api_key: env::var("PARSIO_API_KEY").expect("PARSIO_API_KEY is not set"),
            mailbox_id: env::var("PARSIO_MAILBOX_ID").expect("PARSIO_MAILBOX_ID is not set")
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
    pub docs: Vec<ParsioDoc>
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

impl ParsioDoc {

    pub fn get_title(&self) -> String {
        format!("{} har booket fra {}", self.guest_name, self.check_in)
    }

    pub fn get_description(&self) -> Result<String> {
        let mut builder = Builder::default();
        builder.append(format!("- **Bosted**: {}\n", &self.guest_location));
        builder.append(format!("- **Innsjekk**: {}\n", &self.check_in));
        builder.append(format!("- **Utsjekk**: {}\n", &self.checkout));
        builder.append(format!("- **Gjester**: {}\n", &self.number_of_guests));
        builder.append(format!("- **Payout**: {}\n", &self.host_payout));
        Ok(builder.string()?)
    }

}

impl Display for ParsioDoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has booked {} to {} for {}. Payout {}", self.guest_name, self.check_in, self.checkout, self.number_of_guests, self.host_payout)
    }
}
