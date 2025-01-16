use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Client {
    id: Uuid,
}

impl Client {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn dispose(&self) {
        unimplemented!()
    }
}
