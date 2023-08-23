use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Write},
    marker::PhantomData,
};

use crate::{
    core::{Bundle, Operation},
    internals::Component,
};

pub(crate) trait BundleShader {
    fn alias(&self) -> String;
    fn mode(&self) -> &'static str;
    fn space(&self) -> &'static str;
    fn specifier(&self) -> (&'static str, &'static str);
    fn tag(&self) -> String;
}

#[cfg(feature = "wsgl")]
impl BundleShader for Bundle {
    #[inline]
    fn alias(&self) -> String {
        format!("tsr{}", self.props.typename())
    }

    #[inline]
    fn mode(&self) -> &'static str {
        self.specifier().1
    }

    #[inline]
    fn space(&self) -> &'static str {
        self.specifier().0
    }

    #[inline]
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
            "group({0}) binding({1}) var<{2}, {3}> tsr{2}>",
            self.props.group,
            self.props.binding,
            self.space(),
            self.mode(),
        );

        let post = {
            if self.props.ready() {
                format!(
                    "{0}: array<{1}, {2}>",
                    pre,
                    self.props.typename(),
                    self.props.dims,
                )
            } else {
                format!("{0}: array<{1}>", pre, self.props.typename())
            }
        };

        post
    }
}

trait SupportedComponents {}

pub(crate) trait OperationShader {
    fn add<L, R>(&self, lhs: &impl BundleShader, rhs: &impl BundleShader);
}

#[cfg(feature = "wsgl")]
impl OperationShader for Operation {
    fn add<L, R>(&self, lhs: &impl BundleShader, rhs: &impl BundleShader)
    where
        L: Component,
        R: Component,
    {
        let post = {
            if lhs.props.length == rhs.props.length {
                String::new()
            } else {
                panic!()
            }
        };

        let op = Operation::feed(post);

        Bundle::bind_future(0, op);
    }
}

#[cfg(feature = "wsgl")]
macro_rules! impl_arithmetic {
    ($($op:ident, $fn:ident, )*) => {$(
        impl OperationShader for Operation {
            fn $fn<L, R>(&self, lhs: &impl BundleShader, rhs: &impl BundleShader)
            where
                L: Component,
                R: Component,
            {
            }
        }
    )*}
}
