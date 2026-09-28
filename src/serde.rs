pub fn save_to_json<T: serde::Serialize>(value: T) -> Result<(), serde_json::Error> {
    std::fs::write(
        "exchange.json", 
        serde_json::to_string(&value)?)
        .expect("Write");
    Ok(())
}

pub fn load_from_json<T: serde::de::DeserializeOwned>() -> Result<T, serde_json::Error> {
    let bytes = std::fs::read("exchange.json").expect("Read");
    serde_json::from_slice(bytes.as_slice())
}