pub fn open_message(name: &str) -> String {
    vec!["OPEN", name].join(" ")
}

pub fn move_message(point: &str) -> String {
    vec!["MOVE", point].join(" ")
}
