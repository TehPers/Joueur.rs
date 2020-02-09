mod ai;
mod r#impl;

use crate::base::{Namespace};

pub fn make_namespace() -> impl Namespace<'a> {
    return r#impl::NamespaceImpl<'a>::new();
}
