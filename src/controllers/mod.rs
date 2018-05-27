mod storefront;
mod admin;
mod default;

pub struct Controllers {
    pub storefront: storefront::Index,
    pub admin: admin::Index,
    pub default: default::Index,
}

impl Controllers {
    pub fn new() -> Self {
        Controllers {
            storefront: storefront::Index,
            admin: admin::Index,
            default: default::Index,
        }
    }
}