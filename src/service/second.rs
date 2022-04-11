use crate::app::Params;

pub struct SecondService {
    params: Params
}

impl SecondService {
    pub fn new(params: Params) -> Self {
        Self { params }
    }

    pub fn s1(&self) {
        println!("s1: {:?}", self.params);
    }

    pub fn s2(&self) {
        println!("s2: {:?}", self.params);
    }
}