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
impl<C> BundleShader for Bundle<C>
where
    C: Component,
{
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
                    self.props.length,
                )
            } else {
                format!("{0}: array<{1}>", pre, self.props.typename())
            }
        };

        post
    }
}

struct OperationComponents<L, R, T>
where
    L: Component,
    R: Component,
    T: Component;

trait SupportedComponents {}

pub(crate) trait OperationShader {
    type Target;

    fn add<L, R>(lhs: &dyn BundleShader, rhs: &dyn BundleShader) -> Self;
}

#[cfg(feature = "wsgl")]
impl OperationShader for Operation {
    fn add<L, R>(lhs: &dyn BundleShader, rhs: &dyn BundleShader) -> Self
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

        Bundle::bind_future(0, op)
    }
}
