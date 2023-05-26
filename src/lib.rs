pub mod common {
    pub mod errors;
    pub mod pagination;
    pub mod password_encoder;
}

pub mod infrastructure {
    pub mod database;
}

pub mod todo {
    pub mod handlers;
    pub mod models;
    pub mod repository;
    pub mod views;
}

pub mod user {
    pub mod handlers;
    pub mod models;
    pub mod repository;
    pub mod service;
    pub mod views;
}
