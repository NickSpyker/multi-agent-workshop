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
                    continue; // Skip non-UTF8 files
                };
                // Skip patterns that fail to parse (e.g., multi-state patterns)
                if let Ok(pattern) = Pattern::parse_rle(content) {
                    patterns.push(pattern);
                }
            }
        }

        // Sort patterns by name for consistent ordering
        patterns.sort_by(|a, b| {
            a.display_name()
                .to_lowercase()
                .cmp(&b.display_name().to_lowercase())
        });

        Ok(Self { patterns })
    }

    /// Get all patterns
    pub fn patterns(&self) -> &[Pattern] {
        &self.patterns
    }

    /// Search patterns by name (case-insensitive)
    pub fn search(&self, query: &str) -> Vec<&Pattern> {
        let query_lower = query.to_lowercase();
        self.patterns
            .iter()
            .filter(|p| p.display_name().to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Get the number of patterns
    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    /// Check if collection is empty
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }
}
