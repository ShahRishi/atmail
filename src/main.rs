use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::Mailbox;
use lettre::address::Address;
use dotenv::dotenv;

// use mime::APPLICATION_OCTET_STREAM;
// use std::path::Path;


fn main() {
    dotenv().ok();
    // let smtp_server = "smtp.gmail.com";
    let smtp_username = "shahrishi108@gmail.com";
    let smtp_password = std::env::var("SMTP_TOKEN").expect("SMTP_TOKEN must be set.").to_string();
    // let filename = String::from("test.txt");
    // let smtp_port = 587;

    let from_address = match Address::new("shahrishi108", "gmail.com") {
        Ok(addr) => addr,
        Err(e) => {
            panic!("Failed to create address: {}", e);
        }
    };

    let to_address = match Address::new("shahrishi108", "gmail.com") {
        Ok(addr) => addr,
        Err(e) => {
            panic!("Failed to create address: {}", e);
        }
    };

    let from_user = Mailbox::new(None, from_address.clone());
    let to_user = Mailbox::new(None, to_address.clone());


    let email = Message::builder()
        .from(from_user)
        .to(to_user)
        .subject("hello world!")
        .body(String::from("Be happy!")).unwrap();


    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }

}
