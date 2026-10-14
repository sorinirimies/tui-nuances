//! Internal `macro_rules!` helpers: builder setters and named-theme constructors.
//!
//! * `color_setters!` – consuming setters `fn field(self, Color) -> Self`.
//! * `named_themes!` – generates `Theme::<name>()` constructors plus the
//!   `NAMED` lookup table from compact color specs.

/// Generate consuming setters for plain `Color` fields.
macro_rules! color_setters {
    ($($field:ident),* $(,)?) => {
        $(
            #[doc = concat!("Set the `", stringify!($field), "` color.")]
            #[must_use]
            pub fn $field(mut self, color: ::ratatui::style::Color) -> Self {
                self.$field = color;
                self
            }
        )*
    };
}

/// Generate named theme constructors from hex specs and a `NAMED` table.
///
/// Each entry: `fn_name, "display-name", "doc" => [bg, fg, muted, surface,
/// accent, secondary, success, warning, error, info], [p0 .. p7]`.
macro_rules! named_themes {
    ($(
        $fn_name:ident, $name:literal, $doc:literal =>
        [$bg:expr, $fg:expr, $muted:expr, $surface:expr, $accent:expr, $secondary:expr, $success:expr, $warning:expr, $error:expr, $info:expr],
        [$($p:expr),+ $(,)?]
    );+ $(;)?) => {
        impl Theme {
            $(
                #[doc = $doc]
                #[must_use]
                pub fn $fn_name() -> Self {
                    Self::from_spec(
                        $name,
                        $crate::theme::Spec {
                            bg: $bg, fg: $fg, muted: $muted, surface: $surface,
                            accent: $accent, secondary: $secondary, success: $success,
                            warning: $warning, error: $error, info: $info,
                            palette: &[$($p),+],
                        },
                    )
                }
            )+
        }

        /// Table of macro-generated themes, in cycling order.
        pub(crate) const NAMED: &[(&str, fn() -> Theme)] = &[
            $( ($name, Theme::$fn_name as fn() -> Theme) ),+
        ];
    };
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    #[derive(Default, Debug, PartialEq)]
    struct Probe {
        a: Color,
        b: Color,
    }

    impl Probe {
        color_setters!(a, b);
    }

    #[test]
    fn color_setter_assigns_field() {
        assert_eq!(Probe::default().a(Color::Red).a, Color::Red);
    }

    #[test]
    fn setters_chain() {
        let p = Probe::default().a(Color::Green).b(Color::Cyan);
        assert_eq!(
            p,
            Probe {
                a: Color::Green,
                b: Color::Cyan
            }
        );
    }
}
