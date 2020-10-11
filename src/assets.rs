// A simple macro to help me organize/reference assets by hand
macro_rules! make_assets {
    ($name:ident {
        {$($root_asset:ident: $root_ty: ty, $root_path:expr),* $(,)?}
        $($cat:ident {
            $($asset:ident: $ty: ty, $path:expr),* $(,)?
        }),* $(,)?
    }) => {
        paste::paste! {
            $(
                pub const [<$root_asset:upper>]: &str = concat!("assets/", $root_path);
            )*
            $(
                $(
                    pub const [<$cat:upper _ $asset:upper>]: &str = concat!("assets/", stringify!($cat), "/", $path);
                )*
            )*
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