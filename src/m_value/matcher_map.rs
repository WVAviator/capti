use std::{collections::HashMap, ops::Deref};

use lazy_static::lazy_static;

use super::{match_processor::MatchProcessor, std_matchers::*};

lazy_static! {
    static ref MATCHER_MAP: MatcherMap = MatcherMap::initialize();
}

/// The MatcherMap provides key/value lookups for every MatchProcessor based on the matcher
/// functions defined. Every matcher must be registered here before the map is made available
/// statically.
pub struct MatcherMap(HashMap<String, Box<dyn MatchProcessor>>);

impl MatcherMap {
    pub fn initialize() -> Self {
        let mut map = MatcherMap(HashMap::new());

        map.insert_mp(Exists::new());
        map.insert_mp(Regex::new());
        map.insert_mp(Absent::new());
        map.insert_mp(Empty::new());
        map.insert_mp(Includes::new());
        map.insert_mp(Length::new());
        map.insert_mp(Not::new());

        map
    }

    fn insert_mp(&mut self, processor: Box<dyn MatchProcessor>) {
        self.0.insert(processor.key(), processor);
    }

    pub fn get_matcher(key: &str) -> Option<&Box<dyn MatchProcessor>> {
        MATCHER_MAP.get(key)
    }
}

impl Deref for MatcherMap {
    type Target = HashMap<String, Box<dyn MatchProcessor>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::m_value::m_value::MValue;

    use super::*;

    #[test]
    fn can_get_exists_matcher() {
        let matcher = MatcherMap::get_matcher("$exists").unwrap();
        assert!(matcher
            .is_match(&MValue::Null, &MValue::Bool(false))
            .unwrap());
    }
}
