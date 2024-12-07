use crate::parser::{parse, Span};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::HashSet,
};
use nom::Finish;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct AocDay6 {
    #[allow(dead_code)]
    pub guard_start_position: IVec2,
    #[allow(dead_code)]
    pub walls: HashSet<IVec2>,
}

#[derive(Default)]
pub struct AocDay6Loader;

/// Possible errors that can be produced by [`AocDay6Loader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AocDay6LoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    // /// A [RON](ron) Error
    // #[error("Could not parse RON: {0}")]
    // RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for AocDay6Loader {
    type Asset = AocDay6;
    type Settings = ();
    type Error = AocDay6LoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let (input, (player_position, walls)) =
            parse(Span::new(&bytes)).finish().unwrap();
        // dbg!(input);
        // assert!(
        //     input == Span::new(b"\n"),
        //     "input should have been fully consumed"
        // );

        let custom_asset = AocDay6 {
            guard_start_position: player_position,
            walls,
        };
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["day6.txt"]
    }
}
