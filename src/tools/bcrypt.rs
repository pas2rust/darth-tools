use super::darth_tools::DarthTools;
type BcryptHashResult = Result<String, bcrypt::BcryptError>;
type BcryptVerifyResult = Result<bool, bcrypt::BcryptError>;

pub trait BcryptTrait {
    /// Creates a new bcrypt hash from a value and a cost.
    ///
    /// # Arguments
    ///
    /// * `value` - The string for which the bcrypt hash will be
    ///   generated.
    /// * `cost` - The cost of the bcrypt hash, which is a factor that
    ///   determines the time required to compute the hash.
    ///
    /// # Returns
    ///
    /// A `BcryptHashResult` which is a `Result` with a `String` if
    /// the operation is successful, or a `BcryptError` if the
    /// operation fails.
    fn new_bcrypt_hash(value: String, cost: u32) -> BcryptHashResult;

    /// Verifies a value against a bcrypt hash.
    ///
    /// # Arguments
    ///
    /// * `value` - The string that will be verified against the hash.
    /// * `hash` - A reference to a string that contains the bcrypt
    ///   hash.
    ///
    /// # Returns
    ///
    /// A `BcryptVerifyResult` which is a `Result` with a `bool` if
    /// the operation is successful, or a `BcryptError` if the
    /// operation fails.
    fn new_bcrypt_verify(
        value: String,
        hash: &str,
    ) -> BcryptVerifyResult;
}

// Implementing the `BcryptTrait` trait for the `DarthTools` struct.
impl BcryptTrait for DarthTools {
    fn new_bcrypt_hash(value: String, cost: u32) -> BcryptHashResult {
        bcrypt::hash(value, cost)
    }
    fn new_bcrypt_verify(
        value: String,
        hash: &str,
    ) -> BcryptVerifyResult {
        bcrypt::verify(value, hash)
    }
}
