use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

// pub async fn send_a_verify_mail(id: &str) -> String {
//     let otp_payload = otp::gen_otp(id);
//     let email = Message::builder()
//         .from("NoBody <nobody@domain.tld>".parse().unwrap())
//         // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
//         .to(format!("u{}@tcivs.tc.edu.tw", id).parse().unwrap())
//         .subject("驗證碼")
//         .header(ContentType::TEXT_PLAIN)
//         .body(otp_payload.clone())
//         .unwrap();
//
//     let creds = Credentials::new(
//         "u210718@tcivs.tc.edu.tw".to_owned(),
//         "mlaz ijyv xble rhtc ".to_owned(),
//     );
//
//     // Open a remote connection to gmail
//     let mailer = SmtpTransport::relay("smtp.gmail.com")
//         .unwrap()
//         .credentials(creds)
//         .build();
//
//     // Send the email
//     match mailer.send(&email) {
//         Ok(_) => println!("Email sent successfully!"),
//         Err(e) => panic!("Could not send email: {e:?}"),
//     }
//     otp_payload
// }
