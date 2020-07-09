pub enum ClientMsg {
    Open { my_name: String },
    Move { point: String },
}

pub fn build_message_open(name: String) -> String {
    vec!["OPEN", name].join(" ")
}

pub fn build_message_move(point: String) -> String {
    vec!["MOVE", point].join(" ")
}
