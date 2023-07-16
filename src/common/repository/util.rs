use std::any::Any;
use tokio_postgres::Error;

pub fn handle_already_exist_state<T>(e: Error, default: T) -> T
where
    T: Any,
{
    match e.code() {
        Some(code) => match code.code() {
            "23505" => {
                eprintln!("already exists");
            }
            _ => {
                eprintln!("error: {}", e);
            }
        },
        None => {
            eprintln!("error: {}", e);
        }
    }

    default
}
