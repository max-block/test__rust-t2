use crate::app::Params;

pub struct FirstService {
    params: Params,
}

impl FirstService {
    pub fn new(params: Params) -> Self {
        Self { params }
    }

    pub fn f1(&self) {
        println!("f1: {:?}", self.params);
    }

    pub fn f2(&self) {
        println!("f2");
    }
}
