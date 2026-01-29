use super::{ParseError, Pattern};
use include_dir::{include_dir, Dir};

static PATTERNS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/patterns");

pub struct PatternCollection {
    patterns: Vec<Pattern>,
}

impl PatternCollection {
    pub fn load() -> Result<Self, ParseError> {
        let mut patterns: Vec<Pattern> = Vec::new();

        for file in PATTERNS_DIR.files() {
            if file.path().extension().map_or(false, |ext| ext == "rle") {
                let Some(content) = file.contents_utf8() else {
                    continue;
                };

                if let Ok(pattern) = Pattern::parse_rle(content) {
                    patterns.push(pattern);
                }
            }
        }

        patterns.sort_by(|a, b| {
            a.display_name()
                .to_lowercase()
                .cmp(&b.display_name().to_lowercase())
        });

        Ok(Self { patterns })
    }

    pub fn patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    pub fn search(&self, query: &str) -> Vec<&Pattern> {
        let query_lower = query.to_lowercase();
        self.patterns
            .iter()
            .filter(|p| p.display_name().to_lowercase().contains(&query_lower))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.patterns.len()
    }
}
