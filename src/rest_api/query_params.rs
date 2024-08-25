use std::collections::HashMap;

#[derive(Default)]
pub struct QueryParams(HashMap<String, String>);

impl QueryParams {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from_pairs<I, K, V>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: ToQueryParamValue,
    {
        let mut map = HashMap::new();
        pairs.into_iter().for_each(|(key, value)| {
            map.insert(key.into(), value.to_query_param_value());
        });
        Self(map)
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: ToQueryParamValue,
    {
        self.0.insert(key.into(), value.to_query_param_value());
    }
}

impl IntoIterator for QueryParams {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub(crate) trait ToQueryParamValue {
    fn to_query_param_value(self) -> String;
}

impl ToQueryParamValue for String {
    fn to_query_param_value(self) -> String {
        self
    }
}

impl<'a> ToQueryParamValue for &'a str {
    fn to_query_param_value(self) -> String {
        self.to_string()
    }
}

impl ToQueryParamValue for u64 {
	fn to_query_param_value(self) -> String {
		self.to_string()
	}
}

impl ToQueryParamValue for i64 {
	fn to_query_param_value(self) -> String {
		self.to_string()
	}
}

impl ToQueryParamValue for i32 {
	fn to_query_param_value(self) -> String {
		self.to_string()
	}
}

impl ToQueryParamValue for &[&str] {
    fn to_query_param_value(self) -> String {
        self.join(",")
    }
}

impl ToQueryParamValue for bool {
    fn to_query_param_value(self) -> String {
        self.to_string()
    }
}