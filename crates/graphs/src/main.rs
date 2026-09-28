use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub struct Graph {
    matrix: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn parse_vec_str(input: Vec<&str>) -> Result<Self, String> {
        let mut matrix: HashMap<String, Vec<String>> = HashMap::new();
        for line in input {
            let line = Self::parse(line)?;
            let vec: &mut Vec<String> = matrix.entry(line.0).or_default();
            vec.push(line.1);
        }

        Ok(Self { matrix })
    }

    pub fn parse(input: &str) -> Result<(String, String), String> {
        if input.is_empty() {
            return Err("Empty input".to_string());
        }

        if !input.contains("->") {
            return Err("Incomplete input".to_string());
        }
        let input = input.trim();
        let input = input.split("->").collect::<Vec<&str>>();
        println!("{:?}", input);
        if input.len() != 2 {
            return Err("Incomplete input".to_string());
        }

        let a = input[0].trim();
        let b = input[1].trim();
        Ok((String::from(a), String::from(b)))
    }

    pub fn output(&self) {
        for (k, v) in &self.matrix {
            let line = v.join(" | ");
            println!("{}: {}", k, line);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    #[test]
    fn parse_line() {
        let line1 = "A -> B";
        let line2 = "C -> D";
        let line3 = "F -> A";
        let line4 = "D -> L";

        assert_eq!(Graph::parse(line1), Ok(("A".to_string(), "B".to_string())));
        assert_eq!(Graph::parse(line2), Ok(("C".to_string(), "D".to_string())));
        assert_eq!(Graph::parse(line3), Ok(("F".to_string(), "A".to_string())));
        assert_eq!(Graph::parse(line4), Ok(("D".to_string(), "L".to_string())));
    }

    #[test]
    fn parse_vec() {
        let lines = vec!["A -> B", "C -> D", "F -> A", "D -> B", "F -> B"];

        let graph = Graph::parse_vec_str(lines);
        println!("{:?}", graph);
        assert!(graph.is_ok());
        let graph = graph.unwrap();
        graph.output();
    }
}
