use std::collections::BTreeMap;

/// Query parameter builder for API requests.
///
/// # Examples
/// ```
/// use serwis_planner::QueryParams;
///
/// let params = QueryParams::new()
///     .filter("name__contains", "STB")
///     .filter("status", "active")
///     .order("name", "asc")
///     .limit(50)
///     .page(1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct QueryParams {
    filters: Vec<(String, String)>,
    filter_or: Vec<Vec<(String, String)>>,
    filter_and: Vec<Vec<(String, String)>>,
    order: Vec<(String, String)>,
    fields: Vec<String>,
    extra_fields: Vec<String>,
    limit: Option<u64>,
    page: Option<u64>,
    offset: Option<u64>,
    with_relations: Option<bool>,
    with_cache: Option<bool>,
    limit_to_my_settings: Option<bool>,
    lang: Option<String>,
    for_metadata: Vec<(String, String)>,
}

impl QueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filter(mut self, key: &str, value: impl Into<String>) -> Self {
        self.filters.push((key.to_string(), value.into()));
        self
    }

    pub fn filter_or(mut self, filters: Vec<Vec<(&str, &str)>>) -> Self {
        self.filter_or = filters
            .into_iter()
            .map(|group| {
                group
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            })
            .collect();
        self
    }

    pub fn filter_and(mut self, filters: Vec<Vec<(&str, &str)>>) -> Self {
        self.filter_and = filters
            .into_iter()
            .map(|group| {
                group
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            })
            .collect();
        self
    }

    pub fn order(mut self, field: &str, direction: &str) -> Self {
        self.order.push((field.to_string(), direction.to_string()));
        self
    }

    pub fn fields(mut self, fields: Vec<&str>) -> Self {
        self.fields = fields.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn extra_fields(mut self, fields: Vec<&str>) -> Self {
        self.extra_fields = fields.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_relations(mut self, v: bool) -> Self {
        self.with_relations = Some(v);
        self
    }

    pub fn with_cache(mut self, v: bool) -> Self {
        self.with_cache = Some(v);
        self
    }

    pub fn limit_to_my_settings(mut self, v: bool) -> Self {
        self.limit_to_my_settings = Some(v);
        self
    }

    pub fn lang(mut self, lang: &str) -> Self {
        self.lang = Some(lang.to_string());
        self
    }

    pub fn for_metadata(mut self, key: &str, value: impl Into<String>) -> Self {
        self.for_metadata
            .push((key.to_string(), value.into()));
        self
    }

    pub(crate) fn get_limit(&self) -> Option<u64> {
        self.limit
    }

    /// Build into a flat map of URL query parameters.
    pub fn build(&self) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();

        // Filters
        for (key, value) in &self.filters {
            let (field_parts, op) = parse_filter_key(key);
            let param_key = build_filter_param("filter", &field_parts, &op);
            params.insert(param_key, value.clone());
        }

        // filterOr
        for (idx, group) in self.filter_or.iter().enumerate() {
            for (key, value) in group {
                let (field_parts, op) = parse_filter_key(key);
                let param_key =
                    format!("filterOr[{}]{}", idx, build_filter_suffix(&field_parts, &op));
                params.insert(param_key, value.clone());
            }
        }

        // filterAnd
        for (idx, group) in self.filter_and.iter().enumerate() {
            for (key, value) in group {
                let (field_parts, op) = parse_filter_key(key);
                let param_key =
                    format!("filterAnd[{}]{}", idx, build_filter_suffix(&field_parts, &op));
                params.insert(param_key, value.clone());
            }
        }

        // Order
        for (field, direction) in &self.order {
            params.insert(format!("order[{}]", field), direction.clone());
        }

        // Fields
        if !self.fields.is_empty() {
            params.insert("fields".to_string(), self.fields.join(","));
        }

        // Extra fields
        if !self.extra_fields.is_empty() {
            params.insert("extra_fields".to_string(), self.extra_fields.join(","));
        }

        // Pagination
        if let Some(limit) = self.limit {
            params.insert("page[limit]".to_string(), limit.to_string());
        }
        if let Some(page) = self.page {
            params.insert("page[number]".to_string(), page.to_string());
        }
        if let Some(offset) = self.offset {
            params.insert("page[offset]".to_string(), offset.to_string());
        }

        // Settings
        if let Some(v) = self.with_relations {
            params.insert(
                "setting[with_relations]".to_string(),
                v.to_string(),
            );
        }
        if let Some(v) = self.with_cache {
            params.insert("setting[with_cache]".to_string(), v.to_string());
        }
        if let Some(v) = self.limit_to_my_settings {
            params.insert(
                "setting[limit_to_my_settings]".to_string(),
                v.to_string(),
            );
        }
        if let Some(ref lang) = self.lang {
            params.insert("setting[lang]".to_string(), lang.clone());
        }

        // For metadata
        for (key, value) in &self.for_metadata {
            params.insert(format!("for[{}]", key), value.clone());
        }

        params
    }
}

/// Parse a filter key like "name__contains" or "attributes.476__hasText".
///
/// Returns (field_parts, operator).
/// - "name__contains" → (["name"], "contains")
/// - "status" → (["status"], "eq")
/// - "attributes.476__hasText" → (["attributes", "476"], "hasText")
/// - "commissionPhase.commissionPhaseId" → (["commissionPhase.commissionPhaseId"], "eq")
fn parse_filter_key(key: &str) -> (Vec<String>, String) {
    let (field_str, op) = if let Some(pos) = key.find("__") {
        (&key[..pos], key[pos + 2..].to_string())
    } else {
        (key, "eq".to_string())
    };

    // Split on dots, but only if any segment is purely numeric
    let segments: Vec<&str> = field_str.split('.').collect();
    let has_numeric = segments.iter().any(|s| s.chars().all(|c| c.is_ascii_digit()));

    let field_parts = if has_numeric && segments.len() > 1 {
        segments.iter().map(|s| s.to_string()).collect()
    } else {
        vec![field_str.to_string()]
    };

    (field_parts, op)
}

fn build_filter_param(prefix: &str, field_parts: &[String], op: &str) -> String {
    format!("{}{}", prefix, build_filter_suffix(field_parts, op))
}

fn build_filter_suffix(field_parts: &[String], op: &str) -> String {
    let mut s = String::new();
    for part in field_parts {
        s.push_str(&format!("[{}]", part));
    }
    s.push_str(&format!("[{}]", op));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_filter() {
        let params = QueryParams::new().filter("status", "active").build();
        assert_eq!(params.get("filter[status][eq]").unwrap(), "active");
    }

    #[test]
    fn test_filter_with_operator() {
        let params = QueryParams::new()
            .filter("name__contains", "STB")
            .build();
        assert_eq!(params.get("filter[name][contains]").unwrap(), "STB");
    }

    #[test]
    fn test_filter_with_numeric_segment() {
        let params = QueryParams::new()
            .filter("attributes.476__hasText", "keyword")
            .build();
        assert_eq!(
            params.get("filter[attributes][476][hasText]").unwrap(),
            "keyword"
        );
    }

    #[test]
    fn test_filter_dot_without_numeric() {
        let params = QueryParams::new()
            .filter("commissionPhase.commissionPhaseId", "1")
            .build();
        assert_eq!(
            params
                .get("filter[commissionPhase.commissionPhaseId][eq]")
                .unwrap(),
            "1"
        );
    }

    #[test]
    fn test_pagination() {
        let params = QueryParams::new().limit(50).page(2).build();
        assert_eq!(params.get("page[limit]").unwrap(), "50");
        assert_eq!(params.get("page[number]").unwrap(), "2");
    }

    #[test]
    fn test_order() {
        let params = QueryParams::new().order("name", "asc").build();
        assert_eq!(params.get("order[name]").unwrap(), "asc");
    }

    #[test]
    fn test_fields() {
        let params = QueryParams::new().fields(vec!["id", "name"]).build();
        assert_eq!(params.get("fields").unwrap(), "id,name");
    }

    #[test]
    fn test_settings() {
        let params = QueryParams::new()
            .with_relations(true)
            .lang("pl")
            .build();
        assert_eq!(params.get("setting[with_relations]").unwrap(), "true");
        assert_eq!(params.get("setting[lang]").unwrap(), "pl");
    }
}
