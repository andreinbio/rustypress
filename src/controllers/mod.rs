mod storefront;
mod admin;
mod default;
use utils::Utils;
use rustyview::View;

pub struct Controllers {
    pub storefront: storefront::Index,
    pub admin: admin::Index,
    pub default: default::Index,
}

impl Controllers {
    pub fn new() -> Self {
        let util_helper = Utils::new();
        let admin_template = View::new(util_helper.get_admin_path());
        Controllers {
            storefront: storefront::Index::new(util_helper.clone(), admin_template.clone()),
            admin: admin::Index,
            default: default::Index,
        }
    }
}