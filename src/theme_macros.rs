
#[macro_export]
macro_rules! make_color {
	($r:expr, $g:expr, $b:expr) => ( Color::Rgba($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0));
	($r:expr, $g:expr, $b:expr, $a:expr) => ( Color::Rgba($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, $a as f32 / 255.0));
}

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