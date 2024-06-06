use uuid::Uuid;

pub fn v4() -> String {
    Uuid::new_v4().to_string()
}

pub fn v7() -> String {
    Uuid::now_v7().to_string()
}