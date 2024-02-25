// commands.rs
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::Mailbox;
use lettre::address::Address;
use dotenv::from_path as dotenv_from_path;
use std::fs;
use lettre::message::{Attachment, header::ContentType, };
use lettre::message::{MultiPart, SinglePart};
use mime_guess::from_path as mime_from_path;
use std::path::{Path, PathBuf};


pub fn config(email: String, token: String, config_dir: &PathBuf) {
    // write email and token to config file in configDir
    let config_path = config_dir.join("config.env");
    let config_contents = format!("EMAIL=\"{}\"\nTOKEN=\"{}\"", email, token);

    match fs::write(config_path.clone(), config_contents) {
        Ok(_) => println!("Successfully updated credentials at: {}", config_path.into_os_string().into_string().unwrap()),
        Err(e) => eprintln!("Failed to save configuration: {}", e),
    }
}

pub fn send(to: String, path: String, config_dir: &PathBuf) {
    let config_path = config_dir.join("config.env");

    match dotenv_from_path(config_path) {
        Ok(_) => {},
        Err(_e) => eprintln!("No config file found, please set using atmail config --help")
    }

    let smtp_username = std::env::var("EMAIL").expect("email must be set, see atmail config --help").to_string();
    let smtp_password = std::env::var("TOKEN").expect("token must be set, see tmail config --help").to_string();

    let from: Vec<&str> = smtp_username.split('@').collect();
    assert!(from.len() == 2, "Issues with config email, which must be in the format [user]@[domain], see atmail config --help");
    let to: Vec<&str> = to.split('@').collect();
    assert!(to.len() == 2, "The 'to' email address must be in the format [user]@[domain]");

    let subject = "Here is your file!".to_string();
    let body = "This is a message from Atmail!".to_string();

    let file_path = Path::new(&path);
    let filename = String::from("file");

    let filebody = fs::read(file_path).unwrap();
    let mime_guess = mime_from_path(file_path).first_or_octet_stream();
    let content_type = ContentType::parse(&mime_guess.to_string()).unwrap();
    let attachment = Attachment::new(filename).body(filebody, content_type);
        
    let from_address = Address::new(from[0], from[1]).expect("Error with email configuration, see atm config --help");
    let to_address = Address::new(to[0], to[1]).expect("'to' address must be [user]@[domain]");

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
