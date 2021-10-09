#[macro_export]
macro_rules! grant_options {
    ($service:expr) => {
        routes::request::grant_options()
            .and(with_service($service))
            .and_then(handlers::request::grant_options)
    };
}

#[macro_export]
macro_rules! grant_post {
    ($service:expr) => {
        routes::request::grant_post()
            .and(with_service($service))
            .and_then(handlers::request::grant_post)
    };
}
