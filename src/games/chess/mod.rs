mod ai;
mod r#impl;

use crate::base::{Namespace};

pub fn make_namespace() -> impl Namespace {
    return r#impl::NamespaceImpl::new();
}
