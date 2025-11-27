use uuid::Uuid;
use short_uuid::ShortUuid;

pub fn get_uuid(id: &str) -> Option<Uuid> {
    let trimmed = id.trim();

    if let Ok(short_uuid) = ShortUuid::parse_str(&trimmed) {
        return Some(short_uuid.to_uuid());
    }

    let lowercased = trimmed.to_lowercase();
    if let Ok(uuid) = Uuid::parse_str(&lowercased) {
        return Some(uuid);
    }

    None
}
