use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;

use ndarray::prelude::*;
use snips_nlu_ontology::{BuiltinEntity, BuiltinEntityKind};

use entity_parser::{BuiltinEntityParser, CustomEntity, CustomEntityParser};
use errors::*;
use resources::SharedResources;
use resources::gazetteer::Gazetteer;
use resources::stemmer::Stemmer;
use resources::word_clusterer::WordClusterer;

pub fn file_path(filename: &str) -> ::std::path::PathBuf {
    ::dinghy_test::try_test_file_path("data")
        .unwrap_or_else(|| "../data".into())
        .join(filename)
}

pub fn assert_epsilon_eq_array1(a: &Array1<f32>, b: &Array1<f32>, epsilon: f32) {
    assert_eq!(a.dim(), b.dim());
    for (index, elem_a) in a.indexed_iter() {
        assert!(epsilon_eq(*elem_a, b[index], epsilon))
    }
}

pub fn epsilon_eq(a: f32, b: f32, epsilon: f32) -> bool {
    let diff = a - b;
    diff < epsilon && diff > -epsilon
}


pub struct SharedResourcesBuilder {
    builtin_entity_parser: Arc<BuiltinEntityParser>,
    custom_entity_parser: Arc<CustomEntityParser>,
    gazetteers: HashMap<String, Arc<Gazetteer>>,
    stemmer: Option<Arc<Stemmer>>,
    word_clusterers: HashMap<String, Arc<WordClusterer>>,
}

impl Default for SharedResourcesBuilder {
    fn default() -> Self {
        Self {
            builtin_entity_parser: Arc::<MockedBuiltinEntityParser>::default(),
            custom_entity_parser: Arc::<MockedCustomEntityParser>::default(),
            gazetteers: HashMap::default(),
            stemmer: None,
            word_clusterers: HashMap::default()
        }
    }
}

impl SharedResourcesBuilder {
    pub fn builtin_entity_parser<P: BuiltinEntityParser + 'static>(mut self, parser: P) -> Self {
        self.builtin_entity_parser = Arc::new(parser) as _;
        self
    }

    pub fn custom_entity_parser<P: CustomEntityParser + 'static>(mut self, parser: P) -> Self {
        self.custom_entity_parser = Arc::new(parser) as _;
        self
    }

    pub fn build(self) -> SharedResources {
        SharedResources {
            builtin_entity_parser: self.builtin_entity_parser,
            custom_entity_parser: self.custom_entity_parser,
            gazetteers: self.gazetteers,
            stemmer: self.stemmer,
            word_clusterers: self.word_clusterers
        }
    }
}

#[derive(Default)]
pub struct MockedBuiltinEntityParser {
    pub mocked_outputs: HashMap<String, Vec<BuiltinEntity>>
}

impl BuiltinEntityParser for MockedBuiltinEntityParser {
    fn extract_entities(
        &self,
        sentence: &str,
        _filter_entity_kinds: Option<&[BuiltinEntityKind]>,
        _use_cache: bool,
    ) -> Result<Vec<BuiltinEntity>> {
        Ok(self.mocked_outputs.get(sentence)
            .cloned()
            .unwrap_or(vec![]))
    }
}

impl FromIterator<(String, Vec<BuiltinEntity>)> for MockedBuiltinEntityParser {
    fn from_iter<T: IntoIterator<Item=(String, Vec<BuiltinEntity>)>>(iter: T) -> Self {
        Self { mocked_outputs: HashMap::from_iter(iter) }
    }
}

#[derive(Default)]
pub struct MockedCustomEntityParser {
    pub mocked_outputs: HashMap<String, Vec<CustomEntity>>
}

impl CustomEntityParser for MockedCustomEntityParser {
    fn extract_entities(
        &self,
        sentence: &str,
        _filter_entity_kinds: Option<&[String]>,
    ) -> Result<Vec<CustomEntity>> {
        Ok(self.mocked_outputs.get(sentence)
            .cloned()
            .unwrap_or(vec![]))
    }
}

impl FromIterator<(String, Vec<CustomEntity>)> for MockedCustomEntityParser {
    fn from_iter<T: IntoIterator<Item=(String, Vec<CustomEntity>)>>(iter: T) -> Self {
        Self { mocked_outputs: HashMap::from_iter(iter) }
    }
}
