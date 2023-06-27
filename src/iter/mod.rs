//! Provide the implement of LINQ to Objects, based on `Iterator`.

mod average;
mod m_builtin;
mod m_distinct;
mod m_enumerable;
mod m_method;
mod m_order_by;
mod m_select;
mod m_union;

pub use m_enumerable::*;
