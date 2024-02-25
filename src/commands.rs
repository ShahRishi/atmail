// commands.rs
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::Mailbox;
use lettre::address::Address;
use dotenv::dotenv;
use std::fs;
use lettre::message::{Attachment, header::ContentType, };
use lettre::message::{MultiPart, SinglePart};
use mime_guess::from_path;
use std::path::Path;


pub fn send(to: String, path: String) {
    dotenv().ok();

    let smtp_username = std::env::var("EMAIL").expect("EMAIL must be set, please set this using atm config").to_string();
    let smtp_password = std::env::var("TOKEN").expect("TOKEN must be set, please set this using atm config").to_string();

    let from_address: Vec<&str> = smtp_username.split('@').collect();
    let to_address: Vec<&str> = to.split('@').collect();

    let subject = "Here is your file!".to_string();
    let body = "This is a message from Atmail!".to_string();

    

    let file_path = Path::new(&path);
    let filename = String::from("file");

    let filebody = fs::read(file_path).unwrap();
    let mime_guess = from_path(file_path).first_or_octet_stream();
    let content_type = ContentType::parse(&mime_guess.to_string()).unwrap();
    let attachment = Attachment::new(filename).body(filebody, content_type);
        
    let from_address = match Address::new("shahrishi108", "gmail.com") {
        Ok(addr) => addr,
        Err(e) => {
            panic!("Failed to create address: {}", e);
        }
    };

    let to_address = match Address::new(to_address[0], to_address[1]) {
        Ok(addr) => addr,
        Err(e) => {
            panic!("Failed to create address: {}", e);
        }
    };

    let from_user = Mailbox::new(None, from_address.clone());
    let to_user = Mailbox::new(None, to_address.clone());

    let email = Message::builder()
        .from(from_user.clone())
        .to(to_user.clone())
        .subject(subject.clone())
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::html(body.clone()))
                .singlepart(attachment.clone())
        ).unwrap();


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
