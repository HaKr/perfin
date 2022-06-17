use uuid::Uuid; // 0.8.1

fn show_uuid(uuid: &Uuid) {
    println!("bytes: {:?}", uuid.as_bytes());
    println!("simple: {}", uuid.as_simple());
}

fn main() {
    // Generate a new UUID
    let uuid = Uuid::new_v4();
    show_uuid(&uuid);

    // Parse an existing UUID
    let uuid = Uuid::parse_str("95022733-f013-301a-0ada-abc18f151006").unwrap();
    show_uuid(&uuid);
}
