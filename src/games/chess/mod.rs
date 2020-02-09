mod ai;
mod r#impl;

use crate::base::{Namespace};

pub fn make_namespace<'a>() -> impl Namespace {
    return r#impl::NamespaceImpl::new();
}
