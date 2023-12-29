use darth_tools::{DarthTools, UuidTrait};

#[test]
fn gen_and_verify_uuid() {
    let uuid = DarthTools::new_uuid();
    let verify = DarthTools::verify(uuid);
    match verify {
        Ok(ok) => assert!(ok),
        Err(err) => assert!(false, "{}", err),
    };
}
