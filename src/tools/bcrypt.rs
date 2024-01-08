use super::darth_tools::DarthTools;
type BcryptHashResult = Result<String, bcrypt::BcryptError>;
type BcryptVerifyResult = Result<bool, bcrypt::BcryptError>;

pub trait BcryptTrait {
    fn new_bcrypt_hash(value: String, cost: u32) -> BcryptHashResult;
    fn new_bcrypt_verify(value: String, hash: &str) -> BcryptVerifyResult;
}

impl BcryptTrait for DarthTools {
    fn new_bcrypt_hash(value: String, cost: u32) -> BcryptHashResult {
        bcrypt::hash(value, cost)
    }
    fn new_bcrypt_verify(value: String, hash: &str) -> BcryptVerifyResult {
        bcrypt::verify(value, hash)
    }
}
