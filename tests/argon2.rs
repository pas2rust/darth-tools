use darth_tools::{Argon2Trait, DarthTools};

#[test]
fn argon2_default_gen_hash_and_verify_password() {
    let password = "12345";
    let hash = DarthTools::argon2_default_encode(password).expect("gen hash failed");
    let verify = DarthTools::argon2_default_verify_password(password, &hash);
    match verify {
        Ok(ok) => assert!(ok),
        Err(err) => assert!(false, "{}", err),
    };
}
