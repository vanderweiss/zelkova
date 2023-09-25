use std::any::type_name;

use crate::{
    core::{Bundle, Operation},
    types::{Packet, SupportedPacket},
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

pub(crate) trait BundleShader {
    fn alias(&self) -> String;
    fn binding(&self) -> String;
    fn group(&self) -> String;
    fn storage(&self) -> String;
    fn var(&self) -> String;
}

#[cfg(feature = "wsgl")]
impl<T> BundleShader for Bundle<T>
where
    Packet<T>: SupportedPacket,
{
    #[inline]
    fn alias(&self) -> String {
        format!("tsr_{}_{}", self.typename(), self.props.binding)
    }

    #[inline]
    fn binding(&self) -> String {
        format!("binding({})", self.props.binding)
    }

    #[inline]
    fn group(&self) -> String {
        format!("group({})", self.props.group)
    }

    fn storage(&self) -> String {
        if self.ready() {
            format!("array<{}, {}>", self.typename(), self.props.dims)
        } else {
            format!("array<{}>", self.typename())
        }
    }

    fn var(&self) -> String {
        let (space, mode) = {
            if self.buffer.is_uniform() {
                ("uniform", "read")
            } else {
                ("storage", "read_write")
            }
        };
        format!("var<{}, {}>", space, mode)
    }
}

// will be moved to `core` module
trait SupportedComponents {}

pub(crate) trait OperationShader {
    //fn alias(&self) -> String {}
    fn workgroup(&self) -> String;
}

#[cfg(feature = "wsgl")]
impl<T> OperationShader for Operation<T>
where
    Packet<T>: SupportedPacket,
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
