use uuid::Uuid;

pub type Id = Uuid;

pub fn generate_id() -> Id {
    Uuid::new_v4()
}

pub fn parse_id_raw(s: &str) -> Result<Id, uuid::Error> {
    Uuid::parse_str(s)
}