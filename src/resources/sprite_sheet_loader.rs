use bevy::prelude::*;

// This event gets sent once all sheets are loaded
pub struct SpriteSheetsLoaded;

// A tracker for all sprite sheets that we want to load in
#[derive(Default)]
pub struct SpriteSheetLoader(pub Vec<Handle<Texture>>);