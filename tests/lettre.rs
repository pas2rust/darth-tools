use darth_tools::{DarthTools, LettreTrait};

#[test]
fn test_new_lettre_send_email() {
    let result = DarthTools::new_lettre_send_email(
        "Test Sender",
        "officialdarthstore@gmail.com",
        "Test Receiver",
        "pas2000@proton.me",
        "officialdarthstore",
        "tefr qwjy ppno dwqa",
        "smtp.gmail.com",
        "Test Subject",
        "Test Body",
        587,
    );

    assert!(
        result.is_ok(),
        "Email sending failed: {:?}",
        result.err()
    );
}
