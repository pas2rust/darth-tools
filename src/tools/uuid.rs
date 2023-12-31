use uuid::Uuid;


use super::darth_tools::DarthTools;

pub trait UuidTrait {
    fn new_uuid() -> String;
    fn verify(uuid: String) -> Result<bool, String>;
}

impl UuidTrait for DarthTools {
    fn new_uuid() -> String {
        Uuid::new_v4().to_string()
    }
    fn verify(uuid: String) -> Result<bool, String> {
        match Uuid::parse_str(uuid.as_str()) {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    }
}
