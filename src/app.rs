use std::sync::Arc;

use crate::service::{first::FirstService, second::SecondService};

#[derive(Debug, Clone)]
pub struct Params {
    pub a: String,
    pub b: u8,
}

pub struct App {
    first_service: Arc<FirstService>,
    second_service: Arc<SecondService>,
}

impl App {
    pub fn new(params: Params) -> Self {
        let first_service = Arc::new(FirstService::new(params.clone()));
        let second_service = Arc::new(SecondService::new(params));
        Self {
            first_service,
            second_service,
        }
    }

    pub fn run(&self) {
        self.first_service.f1();
        self.second_service.s1();
        self.first_service.f2();
        self.second_service.s2()
    }
}
