use std::{borrow::Borrow, collections::HashMap, hash::Hash};

pub struct VariableParser<Key, Value> {
    variables: HashMap<Key, (Value, bool)>,
}

impl<'var, Value> VariableParser<&'var str, Value>
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
}
