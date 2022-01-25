use crate::namespace::Namespace;

pub trait Adapter {
    fn namespace(&self, app_id: &String) -> Namespace;
}
