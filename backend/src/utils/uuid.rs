use uuid::Uuid;
use short_uuid::ShortUuid;

/// Converts a string containing a short UUID or full UUID into a UUID object
/// Returns None if the UUID (or short UUID) is invalid
/// Returns Some with the parsed UUID if the UUID (or short UUID) is correct
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
