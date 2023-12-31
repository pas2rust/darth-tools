use darth_tools::{DarthTools, JsonwebtokenTrait};
use serde_json::json;

#[test]
fn new_jsonwebtoken_hs256_encode_and_decode() {
    let secret = "secret";
    let exp = 1000 * 60;
    let items = json!({"id" : "1"});
    let encode = DarthTools::new_jsonwebtoken_hs256_encode(items.clone(), exp, secret);
    match encode {
        Ok(token) => {
            let decode = DarthTools::new_jsonwebtoken_hs256_decode(token, secret);

            match decode {
                Ok(decoded_token) => {
                    assert_eq!(items, decoded_token.items, "items do not match");
                }
                Err(err) => assert!(false, "{}", err),
            }
        }
        Err(err) => assert!(false, "{}", err),
    }
}

#[test]
fn new_jsonwebtoken_hs384_encode_and_decode() {
    let secret = "secret";
    let exp = 1000 * 60;
    let items = json!({"id" : "1"});
    let encode = DarthTools::new_jsonwebtoken_hs384_encode(items.clone(), exp, secret);
    match encode {
        Ok(token) => {
            let decode = DarthTools::new_jsonwebtoken_hs384_decode(token, secret);

            match decode {
                Ok(decoded_token) => {
                    assert_eq!(items, decoded_token.items, "items do not match");
                }
                Err(err) => assert!(false, "{}", err),
            }
        }
        Err(err) => assert!(false, "{}", err),
    }
}

#[test]
fn new_jsonwebtoken_hs512_encode_and_decode() {
    let secret = "secret";
    let exp = 1000 * 60;
    let items = json!({"id" : "1"});
    let encode = DarthTools::new_jsonwebtoken_hs512_encode(items.clone(), exp, secret);
    match encode {
        Ok(token) => {
            let decode = DarthTools::new_jsonwebtoken_hs512_decode(token, secret);

            match decode {
                Ok(decoded_token) => {
                    assert_eq!(items, decoded_token.items, "items do not match");
                }
                Err(err) => assert!(false, "{}", err),
            }
        }
        Err(err) => assert!(false, "{}", err),
    }
}
