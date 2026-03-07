use uuid::Uuid;

#[derive(Clone)]
pub struct GroupId(uuid::Uuid);

impl GroupId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for GroupId {
    fn default() -> Self {
        Self::new()
    }
}
