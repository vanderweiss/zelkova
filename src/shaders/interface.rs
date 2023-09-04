use std::{
    collections::HashMap,
    fmt::{Formatter, Write},
};

use crate::{
    core::{Bundle, Operation},
    types::{Component, Packet},
};

/*
pub(crate) trait ShaderAbstraction {
    const IMPLS: bool = false;

    fn valid(&self) -> bool {
        Self::IMPLS
    }
}

#[cfg(feature = "wsgl")]
impl ShaderAbstraction {
    const IMPLS: bool = true;
}*/

pub(crate) trait BundleShader: Packet {
    fn alias(&self) -> String;
    fn mode(&self) -> &'static str;
    fn space(&self) -> &'static str;
    fn specifier(&self) -> (&'static str, &'static str);
    fn tag(&self) -> String;
}

#[cfg(feature = "wsgl")]
impl<T> BundleShader for Bundle<T>
where
    T: Component,
    Bundle<T>: Packet,
{
    #[inline]
    fn alias(&self) -> String {
        format!("tsr_{}_{}", self.typename(), self.props.binding)
    }

    #[inline]
    fn mode(&self) -> &'static str {
        self.specifier().1
    }

    #[inline]
    fn space(&self) -> &'static str {
        self.specifier().0
    }

    fn specifier(&self) -> (&'static str, &'static str) {
        if self.buffer.is_uniform() {
            ("uniform", "read")
        } else if self.buffer.is_storage() {
            ("storage", "read_write")
        } else {
            panic!()
        }
    }

    fn tag(&self) -> String {
        let pre = format!(
            "group({}) binding({}) var<{}, {}> {}",
            self.props.group,
            self.props.binding,
            self.space(),
            self.mode(),
            self.alias(),
        );

        let post = {
            if self.ready() {
                format!(
                    "{0}: array<{1}, {2}>",
                    pre,
                    self.typename(),
                    self.props.dims,
                )
            } else {
                format!("{0}: array<{1}>", pre, self.typename())
            }
        };

        post
    }
}

// will be moved to `core` module
trait SupportedComponents {}

pub(crate) trait OperationShader {
    //fn alias(&self) -> String {}
    fn add(&self, lhs: &dyn BundleShader, rhs: &dyn BundleShader);
    fn workgroup(&self) -> String;
}

#[cfg(feature = "wsgl")]
impl<T> OperationShader for Operation<T>
where
    T: Component,
{
    #[inline]
    fn workgroup(&self) -> String {
        format!("@workgroup_size({})", self.workgroup.collapse())
    }
}

#[cfg(feature = "wsgl")]
macro_rules! impl_arithmetic {
    ($($op:ident, $fn:ident, )*) => {$(
        impl OperationShader for Operation {
            fn $fn<L, R>(&self, lhs: &Bundle<L>, rhs: &Bundle<R>)
            {
            }
        }
    )*}
}
