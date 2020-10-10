#[macro_export]
macro_rules! match_variants {
    ($input:expr, $enum:ty { $($variant:ident),* },
     $varname:ident => $matchexpr:expr) => {
        {
            type Enum = $enum;
            match $input {
                $(
                    Enum::$variant($varname) => $matchexpr
                ),*
            }
        }
    };
}
