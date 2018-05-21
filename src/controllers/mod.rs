mod admin;
mod default;

pub struct Controllers {
    pub admin: admin::Index,
    pub default: default::Index,
}

impl Controllers {
    pub fn new() -> Self {
        Controllers {
            admin: admin::Index,
            default: default::Index,
        }
    }
}