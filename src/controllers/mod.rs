mod storefront;
mod admin;
mod default;
mod mount;
use utils::Utils;
use rustyview::View;

pub struct Controllers {
    pub storefront: storefront::Index,
    pub admin: admin::Index,
    pub default: default::Index,
    pub mount: mount::Index,
}

impl Controllers {
    pub fn new() -> Self {
        let util_helper = Utils::new();
        let admin_template = View::new(util_helper.get_admin_path());
        Controllers {
            storefront: storefront::Index::new(util_helper.clone(), admin_template.clone()),
            admin: admin::Index::new(util_helper.clone(), admin_template.clone()),
            default: default::Index::new(util_helper.clone(), admin_template.clone()),
            mount: mount::Index::new(util_helper.clone(), admin_template.clone()),
        }
    }
}