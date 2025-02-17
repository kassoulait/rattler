use fxhash::FxBuildHasher;
use indexmap::IndexMap;
use pep508_rs::{Requirement, VersionOrUrl};
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DeserializeAs, DisplayFromStr};

pub(crate) struct Pep440MapOrVec;

impl<'de> DeserializeAs<'de, Vec<Requirement>> for Pep440MapOrVec {
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<Requirement>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[serde_as]
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MapOrVec {
            Vec(Vec<Requirement>),
            Map(
                #[serde_as(as = "IndexMap<_, DisplayFromStr, FxBuildHasher>")]
                IndexMap<String, pep440_rs::VersionSpecifiers, FxBuildHasher>,
            ),
        }

        Ok(match MapOrVec::deserialize(deserializer)? {
            MapOrVec::Vec(v) => v,
            MapOrVec::Map(m) => m
                .into_iter()
                .map(|(name, spec)| pep508_rs::Requirement {
                    name,
                    extras: None,
                    version_or_url: if spec.is_empty() {
                        None
                    } else {
                        Some(VersionOrUrl::VersionSpecifier(spec))
                    },
                    marker: None,
                })
                .collect(),
        })
    }
}
