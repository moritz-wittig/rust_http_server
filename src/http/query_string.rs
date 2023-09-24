use std::collections::HashMap;

pub struct QueryString <'buf>{
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value <'buf> {
    Single(&'buf str), 
    Multipe(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf>{
    pub fn get(&self, key:&str) -> Option<&Value>{
        self.data.get(key)
    }
}

impl <'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        
        for sub_str in s.split("&") {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find("=") {
                key = &sub_str[..i];
                val = &sub_str[i + 1 ..];
            }

            data.entry(key)
                .and_modify(|existing:&mut Value| match existing {
                    Value::Single(prev_val) => {
                        // "*" enables us to de-reference and overwrite it with the 
                        // value following the = sign
                        *existing = Value::Multipe(vec![prev_val, val])
                    }
                    Value::Multipe(vec) => {
                        vec.push(val)
                    }
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}