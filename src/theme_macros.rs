//! Provides common macros for creating themes

#[macro_export]
macro_rules! map_styles {
    { $($type:ty => $fun:expr),*, } => {
        {
            let mut map = StyleMap::default();

            $(map.insert(TypeId::of::<$type>(), $fun);)*

            map
        }
    };
}

#[macro_export]
macro_rules! theme {
    ($Style:ty,
        default {
            $($dprop:ident = $dval:expr);*;
        }
        $(
            $($requirenment:ident),* {
                $($prop:ident = $val:expr);*;
            }
        )*
    ) => {
        {
            let mut default_style = <$Style>::default();
            { $( default_style.$dprop = Some($dval); )* }
            WidgetStyle::new(default_style)
            $(
                .when(InteractionState::default()$(.$requirenment())*, {
                    let mut style = <$Style>::default();
                    { $( style.$prop = Some($val); )* }
                    style
                })
            )*
            .dynamic()
        }
    };
    ($Style:ty,
        $(
            $($requirenment:ident),* {
                $($prop:ident = $val:expr);*;
            }
        )*
    ) => {
        {
            WidgetStyle::new(<$Style>::default())
            $(
                .when(InteractionState::default()$(.$requirenment())*, {
                    let mut style = <$Style>::default();
                    { $( style.$prop = Some($val); )* }
                    style
                })
            )*
            .dynamic()
        }
    };
}
