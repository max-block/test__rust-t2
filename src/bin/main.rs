use rust1::app::{Params, App};

fn main() {
    let params = Params{a:"zzz".into(), b:3};
    let app = App::new(params);
    app.run();
}
