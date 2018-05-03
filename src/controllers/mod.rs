mod admin;

pub struct Controllers {
    pub admin: admin::Index,
}

impl Controllers {
    pub fn new() -> Self {
        Contollers {
            admin: admin::Index::new()
        }
    }
}