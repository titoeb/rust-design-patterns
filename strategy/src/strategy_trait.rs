use std::collections::HashMap;
type Data = HashMap<String, usize>;

trait Formatter {
    fn format(&self, data: &Data) -> String;
}

fn test_data() -> HashMap<String, usize> {
    let mut data = HashMap::new();
    data.insert("one".to_string(), 1);
    data.insert("two".to_string(), 2);
    data
}

struct Report;
impl Report {
    fn generate<T: Formatter>(g: T) -> String {
        g.format(&test_data())
    }
}

struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data) -> String {
        data.into_iter()
            .map(|(k, v)| format!("{} {}\n", k, v))
            .collect::<String>()
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data) -> String {
        format!(
            "[{}]",
            data.into_iter()
                .map(|(k, v)| format!(r#"{{"{}":"{}"}},"#, k, v))
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_text() {
        let s = Report::generate(Text);
        assert!(s.contains("one 1"));
        assert!(s.contains("two 2"));
    }
    #[test]
    fn to_json() {
        let s = Report::generate(Json);
        assert!(s.contains(r#"{"one":"1"}"#));
        assert!(s.contains(r#"{"two":"2"}"#));
    }
}
