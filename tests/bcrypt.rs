use darth_tools::{BcryptTrait, DarthTools};

#[test]
fn bcrypt_gen_hash_and_verify_password() {
    let password = "12345";
    let hash = DarthTools::new_bcrypt_hash(password.to_string(), 12).expect("gen hash failed");
    let verify = DarthTools::new_bcrypt_verify(password.to_string(), &hash);
    match verify {
        Ok(ok) => assert!(ok),
        Err(err) => assert!(false, "{}", err),
    };
}
