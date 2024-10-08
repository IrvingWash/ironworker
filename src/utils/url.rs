use std::collections::HashMap;

pub struct Url {
    origin: String,
    path: String,
    query_params: HashMap<String, String>,
}

impl Url {
    pub fn new(url: &str) -> Result<Self, String> {
        let parts: Vec<&str> = url.split('.').collect();

        if parts.len() < 2 {
            return Err(format!("Wrong URL: {}", url));
        };

        let top_level_and_path: Vec<&str> = parts[1].split('/').collect();

        if top_level_and_path.len() == 1 {
            return Ok(Self {
                origin: url.to_owned(),
                path: String::from(""),
                query_params: Url::extract_query_params(url)?,
            });
        };

        Ok(Self {
            origin: format!("{}.{}", parts[0], top_level_and_path[0]),
            path: top_level_and_path[1].to_owned(),
            query_params: Url::extract_query_params(url)?,
        })
    }

    #[allow(dead_code)]
    pub fn from_url(path: &str, url: &Url) -> Result<Self, String> {
        let path = if path.starts_with('/') {
            &path[1..path.len()]
        } else {
            path
        };

        let path = if path.ends_with('/') {
            &path[0..path.len() - 1]
        } else {
            path
        };

        Ok(Self {
            origin: url.origin().to_owned(),
            path: path.to_owned(),
            query_params: Url::extract_query_params(path)?,
        })
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    #[allow(dead_code)]
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn href(&self) -> String {
        let mut query_params: Vec<String> = Vec::new();

        for (key, value) in &self.query_params {
            query_params.push(format!("{}={}", key, value));
        }

        let mut path = format!("{}/{}", self.origin, self.path);

        if !query_params.is_empty() {
            path = format!("{}?{}", path, query_params.join("&"));
        };

        path
    }

    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    pub fn add_query_param(&mut self, key: &str, value: &str) {
        self.query_params.insert(key.to_owned(), value.to_owned());
    }

    fn extract_query_params(url: &str) -> Result<HashMap<String, String>, String> {
        let parts: Vec<&str> = url.split('?').collect();

        let mut map: HashMap<String, String> = HashMap::new();

        if parts.len() == 1 {
            return Ok(map);
        };

        if parts.len() != 2 {
            return Err(format!("Wrong URL: {}", url));
        };

        let params: Vec<&str> = parts[1].split('&').collect();

        for param in params {
            let key_value: Vec<&str> = param.split('=').collect();

            if key_value.len() != 2 {
                return Err(format!("Wrong key-value pair: {}", key_value.join("=")));
            };

            map.insert(key_value[0].to_owned(), key_value[1].to_owned());
        }

        Ok(map)
    }
}

#[cfg(test)]
mod url_tests {
    use super::Url;

    #[test]
    fn test_new() -> Result<(), String> {
        let url = Url::new("https://bandcampcom/orgoneus/");

        assert_eq!(
            url.err(),
            Some("Wrong URL: https://bandcampcom/orgoneus/".to_owned())
        );

        let url = Url::new("https://bandcamp.com/orgoneus/")?;

        assert_eq!(url.origin(), "https://bandcamp.com");
        assert_eq!(url.path(), "orgoneus");

        Ok(())
    }

    #[test]
    fn test_from_url() -> Result<(), String> {
        let url = Url::new("https://bandcamp.com/orgoneus")?;

        let url = Url::from_url("/miracleworkerus/", &url)?;

        assert_eq!(url.origin(), "https://bandcamp.com");
        assert_eq!(url.path(), "miracleworkerus");

        Ok(())
    }

    #[test]
    fn test_href() -> Result<(), String> {
        let url = Url::new("https://bandcamp.com/krallice/")?;

        assert_eq!(url.href(), "https://bandcamp.com/krallice");

        Ok(())
    }

    #[test]
    fn test_query_params() -> Result<(), String> {
        let url = Url::new("https://bandcamp.com/gorguts?download");

        assert!(url.is_err());

        let url = Url::new("https://bandcamp.com/gorguts?download=true")?;

        assert_eq!(url.query_params().len(), 1);

        let url = Url::from_url("miracleworkerus?albums=5&from=2012", &url)?;

        assert_eq!(url.query_params().len(), 2);

        Ok(())
    }
}
