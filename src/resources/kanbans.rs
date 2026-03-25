use std::sync::Arc;

use crate::client::ClientInner;
use crate::resources::base::Resource;

/// Kanbans resource (`/api/kanbans`).
pub struct KanbansResource {
    pub resource: Resource,
}

impl KanbansResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self {
            resource: Resource::new(inner, "/api/kanbans"),
        }
    }
}

impl std::ops::Deref for KanbansResource {
    type Target = Resource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}
