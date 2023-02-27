use crate::errors::CustomError;

use sendgrid::SGClient;
use sendgrid::{Destination, Mail};


#[derive(Debug, Clone)]
pub struct Email {
    target_address: String,
    email_html: String,
    subject: String,
    sg: SGClient,
}

impl Email {
    pub fn new(target_address: String, email_html: String, subject: String, sg: SGClient) -> Self {
        Email {
            target_address,
            subject,
            email_html,
            sg,
        }
    }

    pub async fn send(email: &Email) -> Result<(), CustomError> {
        
        let mail_info = Mail::new()
            .add_to(Destination {
                address: email.target_address.as_str(),
                name: "Participant",
            })
            .add_from("usersupport@intersectional-data.ca")
            .add_subject(&email.subject)
            .add_html(email.email_html.as_str())
            .add_from_name("User Support at Intersectional-Data.ca")
            .add_header("x-system-generated".to_string(), "confirmed");

        match email.sg.send(mail_info) {
            Ok(body) => {
                println!("Response: {:?}", &body);
                return Ok(())
            },
            Err(err) => {
                println!("Error: {}", err);
                return Err(CustomError::new(101, format!("message not sent: {}", err)))
            },
        };
    }
}