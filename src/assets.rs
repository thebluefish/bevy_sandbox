use bevy::prelude::*;

macro_rules! make_assets {
    ($name:ident {
        {$($root_asset:ident: $root_ty: ty, $root_path:expr),* $(,)?}
        $($cat:ident {
            $($asset:ident: $ty: ty, $path:expr),* $(,)?
        }),* $(,)?
    }) => {
        paste::paste! {
            pub mod [<$name:lower _paths>] {
                $(
                    pub const [<$root_asset:upper>]: &str = concat!("assets/", $root_path);
                )*
                $(
                    $(
                        pub const [<$cat:upper _ $asset:upper>]: &str = concat!("assets/", stringify!($cat), "/", $path);
                    )*
                )*
            }

            $(
                pub struct [<$name:camel $cat:camel>];

                impl [<$name:camel $cat:camel>] {
                    pub fn check_all(resources: &Resources) -> bool {
                        let asset_server = resources.get::<AssetServer>().unwrap();
                        let mut all_good = true;

                        $(
                            if !Self::[<check_ $cat:lower _ $asset:lower>](&asset_server, &resources.get::<Assets<$ty>>().unwrap()) { all_good = false; }
                        )*

                        all_good
                    }

                    $(
                        #[inline]
                        pub fn [<check_ $cat:lower _ $asset:lower>](asset_server: &AssetServer, assets: &Assets<$ty>) -> bool {
                            let handle = match asset_server.get_handle([<$name:lower _paths>]::[<$cat:upper _ $asset:upper>]) {
                                Some(handle) => handle,
                                None => asset_server.load([<$name:lower _paths>]::[<$cat:upper _ $asset:upper>]).unwrap(),
                            };

                            assets.get(&handle).map_or(false, |_| true)
                        }

                        #[inline]
                        pub fn [<$cat:lower _ $asset:lower>](asset_server: &AssetServer) -> Option<Handle<$ty>> {
                            match asset_server.get_handle([<$name:lower _paths>]::[<$cat:upper _ $asset:upper>]) {
                                Some(handle) => Some(handle),
                                None => asset_server.load([<$name:lower _paths>]::[<$cat:upper _ $asset:upper>]).ok(),
                            }
                        }
                    )*
                }
            )*

            pub struct [<$name:camel Assets>];

            impl [<$name:camel Assets>] {
                pub fn check_all(resources: &Resources) -> bool {
                    let asset_server = resources.get::<AssetServer>().unwrap();

                    let mut all_good = true;

                    $(
                        if !Self::[<check_ $root_asset:lower>](&asset_server, &resources.get::<Assets<$root_ty>>().unwrap()) { all_good = false; }
                    )*

                    all_good
                }

                $(
                    #[inline]
                    pub fn [<check_ $root_asset:lower>](asset_server: &AssetServer, assets: &Assets<$root_ty>) -> bool {
                        let handle = match asset_server.get_handle([<$name:lower _paths>]::[<$root_asset:upper>]) {
                            Some(handle) => handle,
                            None => asset_server.load([<$name:lower _paths>]::[<$root_asset:upper>]).unwrap(),
                        };

                        assets.get(&handle).map_or(false, |_| true)
                    }

                    #[inline]
                    pub fn [<$root_asset:lower>](asset_server: &AssetServer) -> Option<Handle<$root_ty>> {
                        match asset_server.get_handle([<$name:lower _paths>]::[<$root_asset:upper>]) {
                            Some(handle) => Some(handle),
                            None => asset_server.load([<$name:lower _paths>]::[<$root_asset:upper>]).ok(),
                        }
                    }
                )*
            }
        }
    }
}

make_assets!(Game {
    {
        loading: Texture, "loading.png",
    }
    fonts {
        mini_square: Font, "Kenney Mini Square.ttf",
        pixel_square: Font, "Kenney Pixel Square.ttf",
    },
    textures {
        roguelike_char: Texture, "roguelikeChar_transparent.png",
        roguelike_dungeon: Texture, "roguelikeDungeon_transparent.png",
        roguelike_sheet: Texture, "roguelikeSheet_transparent.png",
        uipack_sheet: Texture, "UIpackSheet_transparent.png",
    },
});