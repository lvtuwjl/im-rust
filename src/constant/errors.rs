use std::num::NonZeroU16;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU16);

impl StatusCode {
    #[inline]
    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    pub fn canonical_reason(&self) -> Option<&'static str> {
        canonical_reason(self.0.get())
    }
}

impl From<StatusCode> for u16 {
    #[inline]
    fn from(status: StatusCode) -> u16 {
        status.0.get()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_main() {
        use super::StatusCode;
        let sc = StatusCode::DATAERROR;
        println!("{:?}", sc.canonical_reason());
        println!("{}", sc.as_u16());
    }

    #[test]
    fn test_axum() {
        use axum::http::StatusCode;
        let sc = StatusCode::OK;
        println!("{:?}", sc.canonical_reason());
        println!("{}", sc.as_u16());
    }
}

#[macro_export]
macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl StatusCode {
        $(
            $(#[$docs])*
            pub const $konst: StatusCode = StatusCode(unsafe { NonZeroU16::new_unchecked($num) });
        )+

        }

        fn canonical_reason(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

status_codes! {
/// 100 Continue
/// [[RFC7231, Section 6.2.1](https://tools.ietf.org/html/rfc7231#section-6.2.1)]
(1000, NOPERMISSION, "No Permission");
/// 101 Switching Protocols
/// [[RFC7231, Section 6.2.2](https://tools.ietf.org/html/rfc7231#section-6.2.2)]
(1001, DATAERROR, "Data Error");
/// 102 Processing
/// [[RFC2518](https://tools.ietf.org/html/rfc2518)]
(1002, TOOLENGTH, "Too Length");
}
