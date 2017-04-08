
#[macro_export]
macro_rules! bkpt {
    () => {
        unsafe { asm!("bkpt" :::: "volatile") }
    };
    ($imm:expr) => {
        unsafe { asm!(concat!("bkpt #", stringify!($imm)) :::: "volatile") }
    };
}

#[macro_export]
macro_rules! implement_enum {
    ($name:ident: $value_type:ty {
        $($label:ident = $value:expr, )*
    }) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $($label = $value, )*
        }

        impl Default for $name {
            fn default() -> $name {
                0.into()
            }
        }

        impl From<$value_type> for $name {
            fn from(val: $value_type) -> $name {
                match val {
                    $($value => $name::$label, )*
                    _ => panic!("Invalid value {} for {}", val, stringify!(name))
                }
            }
        }
    }
}