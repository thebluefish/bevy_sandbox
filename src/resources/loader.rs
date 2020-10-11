use bevy::{prelude::*, asset::{AssetServerError, HandleId}};
use std::path::Path;
use bevy::asset::LoadState;

#[derive(thiserror::Error, Debug)]
pub enum LoaderError {
    #[error("Failed to load asset.")]
    AssetServerError(#[from] AssetServerError),
}

#[derive(Default)]
pub struct Loader(Vec<HandleId>);

impl Loader {
    #[inline]
    pub fn track(&mut self, id: HandleId) {
        self.0.push(id);
    }

    pub fn load<T: Resource, P: AsRef<Path>>(&mut self, asset_server: &AssetServer, paths: &[P]) -> Result<(), LoaderError> {
        for path in paths {
            self.track(asset_server.load::<T, _>(path)?.id);
        }

        Ok(())
    }

    // Returns true if we are not waiting for anything to load
    pub fn check(&mut self, asset_server: &AssetServer) -> bool {
        let mut all_loaded = true;

        self.0.retain(|id| {
            match asset_server.get_load_state_untyped(*id) {
                Some(LoadState::Loaded(_)) => {
                    return false; // removes element from local cache
                },
                Some(LoadState::Failed(_)) => panic!("failed to load asset!"), // TODO: Do we handle this more succinctly?
                _ => { all_loaded = false },
            }

            true
        });

        all_loaded
    }
}