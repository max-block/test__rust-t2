use std::sync::Arc;

use crate::app::Params;

use super::first::FirstService;

pub struct SecondService {
    params: Params,
    first_service: Arc<FirstService>
}

impl SecondService {
    pub fn new(params: Params, first_service: Arc<FirstService>) -> Self {
        Self { params, first_service }
    }

    pub fn s1(&self) {
        println!("s1: {:?}", self.params);
        self.first_service.f1();
    }

    pub fn s2(&self) {
        println!("s2: {:?}", self.params);
        self.first_service.f2();
    }
}