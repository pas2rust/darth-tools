use darth_tools::{DarthTools, LettreTrait};

#[test]
fn test_new_lettre_send_email() {
    dotenv::dotenv().ok();
    let secret = std::env::var("SMTP_GMAIL_PASSWORD").expect("env SMTP_GMAIL_PASSWORD error");
    let result: Result<(), lettre::error::Error> = DarthTools::new_lettre_send_email(
        "Test Sender",
        "officialdarthstore@gmail.com",
        "Test Receiver",
        "pas2000@proton.me",
        "officialdarthstore",
        secret.as_str(),
        "smtp.gmail.com",
        "Test Subject",
        "Test Body",
        587,
    );

    assert!(result.is_ok(), "Email sending failed: {:?}", result.err());
}
