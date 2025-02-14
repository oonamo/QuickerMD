use std::collections::HashMap;

pub struct VariableParser<'var, Value> {
    variables: HashMap<&'var str, (Value, bool)>,
}

impl<'var, Value> VariableParser<'var, Value>
where
    Value: Copy + ToString,
{
    pub fn new(variables: Vec<(&'var str, Value)>) -> Self {
        let mut hash: HashMap<&str, (Value, bool)> = HashMap::with_capacity(variables.len());

        for (key, value) in variables.iter() {
            let tuple = (*value, false);
            hash.insert(*key, tuple);
        }

        Self { variables: hash }
    }

    pub fn parse_string_vec(&self, vec: &mut Vec<String>) {
        for item in vec.iter_mut() {
            if let Some(val) = self.variables.get(item.as_str()) {
                *item = val.0.to_string()
            }
        }
    }

    pub fn parse_with_tracker(&mut self, vec: &mut Vec<String>) {
        for item in vec.iter_mut() {
            if let Some(val) = self.variables.get_mut(item.as_str()) {
                *item = val.0.to_string();
                val.1 = true;
            }
        }
    }

    pub fn had_used_var(&self, key: &'var str) -> bool {
        if let Some((_, usued)) = self.variables.get(key) {
            return *usued;
        }

        false
    }

    #[cfg(test)]
    pub fn get_size(&self) -> usize {
        self.variables.keys().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_an_empty_parser() {
        let parse = VariableParser::<i32>::new(vec![]);
        assert!(parse.get_size() == 0)
    }

    #[test]
    fn can_create_and_parse_parser_with_int_variables() {
        let variable_vec = vec![("beans", 5), ("rice", 3), ("{{THINGS}}", 3)];
        let variable_vec_len = variable_vec.len();

        let parser = VariableParser::<i32>::new(variable_vec);

        assert!(parser.get_size() == variable_vec_len);

        let mut to_be_parsed = vec![
            "--var".to_string(),
            "beans".to_string(),
            "--something-else".to_string(),
            "{{THINGS}}".to_string(),
        ];

        parser.parse_string_vec(&mut to_be_parsed);

        println!("{:?}", to_be_parsed);

        assert!(to_be_parsed[0] == "--var");
        assert!(to_be_parsed[1] == "5");
        assert!(to_be_parsed[2] == "--something-else");
        assert!(to_be_parsed[3] == "3");
    }
}
