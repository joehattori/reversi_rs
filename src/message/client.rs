pub fn open_message(name: &str) -> String {
    vec!["OPEN", name].join(" ")
}

pub fn move_message(pos: &str) -> String {
    vec!["MOVE", pos].join(" ")
}
