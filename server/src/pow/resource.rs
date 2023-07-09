use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Resource {
    client_id: String,
    client_ip: IpAddr,
    hash: String,
}

impl Resource {
    /// Creates a new `resource` that will be used to send to a client to ask it for challenge.
    pub fn new(client_ip: &IpAddr, client_id: Option<String>) -> Self {
        let client_ip = client_ip.to_owned();
        let client_id = client_id.unwrap_or(Uuid::new_v4().to_string());
        let hash = Self::encode(&client_ip, &client_id);

        Self {
            client_ip,
            client_id,
            hash,
        }
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn get_client_ip(&self) -> &IpAddr {
        &self.client_ip
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    fn encode(client_ip: &IpAddr, client_id: &str) -> String {
        let mut hasher: DefaultHasher = DefaultHasher::new(); // TODO Try different algorithm
        let hash: String = format!("{}:{}", client_id, client_ip);

        hasher.write(hash.as_bytes());
        hasher.finish().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn get_client_ip() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    }

    #[test]
    fn test_on_instantiate_with_client_id() {
        let client_id = String::from("dummy-id");
        let client_ip = get_client_ip();

        assert_eq!(
            Resource::new(&client_ip, Some(client_id.to_owned())).get_client_id(),
            client_id
        );
    }

    #[test]
    fn test_on_instantiate_with_no_client_id() {
        let client_ip = get_client_ip();

        assert!(!Resource::new(&client_ip, None).get_client_id().is_empty());
    }

    #[test]
    fn test_on_get_client_ip() {
        let client_ip = get_client_ip();

        assert_eq!(
            Resource::new(&client_ip, None).get_client_ip().to_string(),
            get_client_ip().to_string()
        );
    }

    #[test]
    fn test_on_encode_resource() {
        let client_ip = get_client_ip();
        let client_id = String::from("dummy-id");

        assert_eq!(
            Resource::new(&client_ip, Some(client_id)).get_hash(),
            String::from("1992790771812023011")
        );
    }
}
