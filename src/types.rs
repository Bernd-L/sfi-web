use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct User {
    pub uuid: Uuid,
}
