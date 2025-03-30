// Implements different URL output formatters
use super::UrlData;
use std::fmt;

pub trait Formatter: fmt::Debug + Send + Sync {
    fn format(&self, url_data: &UrlData, is_last: bool) -> String;
    fn clone_box(&self) -> Box<dyn Formatter>;
}

impl Clone for Box<dyn Formatter> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct PlainFormatter;

impl PlainFormatter {
    pub fn new() -> Self {
        PlainFormatter
    }
}

impl Formatter for PlainFormatter {
    fn format(&self, url_data: &UrlData, _is_last: bool) -> String {
        match &url_data.status {
            Some(status) => format!("{} [{}]\n", url_data.url, status),
            None => format!("{}\n", url_data.url),
        }
    }

    fn clone_box(&self) -> Box<dyn Formatter> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct JsonFormatter;

impl JsonFormatter {
    pub fn new() -> Self {
        JsonFormatter
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, url_data: &UrlData, is_last: bool) -> String {
        let json = match &url_data.status {
            Some(status) => format!("{{\"url\":\"{}\",\"status\":\"{}\"}}", url_data.url, status),
            None => format!("{{\"url\":\"{}\"}}", url_data.url),
        };

        if is_last {
            format!("{}\n", json)
        } else {
            format!("{},", json)
        }
    }

    fn clone_box(&self) -> Box<dyn Formatter> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct CsvFormatter;

impl CsvFormatter {
    pub fn new() -> Self {
        CsvFormatter
    }
}

impl Formatter for CsvFormatter {
    fn format(&self, url_data: &UrlData, _is_last: bool) -> String {
        match &url_data.status {
            Some(status) => format!("{},{}\n", url_data.url, status),
            None => format!("{},\n", url_data.url),
        }
    }

    fn clone_box(&self) -> Box<dyn Formatter> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_formatter() {
        let formatter = PlainFormatter::new();

        // Test URL without status
        let url_data = UrlData::new("https://example.com".to_string());
        assert_eq!(formatter.format(&url_data, false), "https://example.com\n");

        // Test URL with status
        let url_data_status =
            UrlData::with_status("https://example.com".to_string(), "200 OK".to_string());
        assert_eq!(
            formatter.format(&url_data_status, true),
            "https://example.com [200 OK]\n"
        );
    }

    #[test]
    fn test_json_formatter() {
        let formatter = JsonFormatter::new();

        // Test URL without status
        let url_data = UrlData::new("https://example.com".to_string());
        assert_eq!(
            formatter.format(&url_data, false),
            "{\"url\":\"https://example.com\"},"
        );
        assert_eq!(
            formatter.format(&url_data, true),
            "{\"url\":\"https://example.com\"}\n"
        );

        // Test URL with status
        let url_data_status =
            UrlData::with_status("https://example.com".to_string(), "200 OK".to_string());
        assert_eq!(
            formatter.format(&url_data_status, false),
            "{\"url\":\"https://example.com\",\"status\":\"200 OK\"},"
        );
    }

    #[test]
    fn test_csv_formatter() {
        let formatter = CsvFormatter::new();

        // Test URL without status
        let url_data = UrlData::new("https://example.com".to_string());
        assert_eq!(formatter.format(&url_data, false), "https://example.com,\n");

        // Test URL with status
        let url_data_status =
            UrlData::with_status("https://example.com".to_string(), "200 OK".to_string());
        assert_eq!(
            formatter.format(&url_data_status, true),
            "https://example.com,200 OK\n"
        );
    }

    #[test]
    fn test_formatter_clone() {
        let plain_formatter: Box<dyn Formatter> = Box::new(PlainFormatter::new());
        let cloned_formatter = plain_formatter.clone();

        let url_data = UrlData::new("https://example.com".to_string());
        assert_eq!(
            plain_formatter.format(&url_data, false),
            cloned_formatter.format(&url_data, false)
        );
    }
}
