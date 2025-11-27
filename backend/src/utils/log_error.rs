use std::{fmt::{Debug, Display}};

use log::error;

pub trait ResultLogger<T, E> {
    #[allow(unused)]
    fn log_on_error(self) -> Self;
    #[allow(unused)]
    fn log_with_place_on_error(self, place: &'static str) -> Self;
}

impl<T, E> ResultLogger<T, E> for Result<T, E>
where
    E: Display + Debug,
{
    fn log_on_error(self) -> Self {
        if let Err(ref e) = self {
            error!("Error: {:?}", e);
        }

        self
    }

    fn log_with_place_on_error(self, place: &'static str) -> Self {
        if let Err(ref e) = self {
            error!("Place: {}. Error: {:?}", place, e);
        }

        self
    }
}
