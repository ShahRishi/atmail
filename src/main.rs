use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::Mailbox;
use lettre::address::Address;
use dotenv::dotenv;
use std::fs;
use lettre::message::{Attachment, header::ContentType};
use lettre::message::MultiPart;
use lettre::message::SinglePart;
use mime_guess::from_path;


// use mime::APPLICATION_OCTET_STREAM;
use std::path::Path;


fn main() {
    dotenv().ok();
    let smtp_username = "shahrishi108@gmail.com";
    let smtp_password = std::env::var("SMTP_TOKEN").expect("SMTP_TOKEN must be set.").to_string();
    // let filename = String::from("test.txt");
    // let smtp_port = 587;

    let filename = String::from("image.png");
    let file_path = Path::new("image.png");
    let filebody = fs::read("image.png").unwrap();
    let mime_guess = from_path(file_path).first_or_octet_stream();
    let content_type = ContentType::parse(&mime_guess.to_string()).unwrap();
    let attachment = Attachment::new(filename).body(filebody, content_type);

    
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
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::html(String::from("Hey, here's your file from Atmail!")))
                .singlepart(attachment)
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
