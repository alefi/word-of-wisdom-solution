use self::error::parse_error::ParseError;
use crate::pow::resource::Resource;
use hashcash::check_with_params;
use std::net::IpAddr;

mod error;
mod hash_storage;
mod resource;

pub struct Pow {
    pub resource: Resource,
    stamp: Option<String>,
}

impl Pow {
    pub fn new(client_ip: &IpAddr, client_id: Option<String>) -> Self {
        let pow_object = Self {
            resource: Resource::new(client_ip, client_id),
            stamp: None,
        };

        hash_storage::set(pow_object.resource.get_hash(), &pow_object.resource);

        pow_object
    }

    pub fn parse(message: String) -> Result<Self, ParseError> {
        if let Some((resource_hash, stamp)) = message.split_once(':') {
            let resource = hash_storage::get_and_remove(resource_hash).unwrap();

            let pow_object = Self {
                resource: Resource::new(
                    resource.get_client_ip(),
                    Some(resource.get_client_id().to_string()),
                ),
                stamp: Some(String::from(stamp)),
            };

            match pow_object.check(resource_hash) {
                Ok(true) => Ok(pow_object),
                _ => Err(ParseError::ResourceInvalid),
            }
        } else {
            Err(ParseError::MessageMalformed)
        }
    }

    fn check(&self, resource_hash: &str) -> Result<bool, ParseError> {
        if resource_hash != self.resource.get_hash() {
            return Err(ParseError::ResourceInvalid);
        }

        let pow_stamp: String = self.stamp.clone().unwrap();

        match check_with_params(&pow_stamp, Some(self.resource.get_hash()), Some(12), None) {
            Ok(res) => Ok(res),
            Err(_) => Err(ParseError::ResourceInvalid),
        }
    }
}
