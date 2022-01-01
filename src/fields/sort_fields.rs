use std::{collections::HashMap, rc::Rc};

use mongodb::bson::{doc, Document};

use crate::RequestError;

pub struct SortField {
    pub name: String,
    pub alias: Option<String>,
}

impl From<&str> for SortField {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl From<String> for SortField {
    fn from(s: String) -> Self {
        Self {
            name: s,
            alias: None,
        }
    }
}

impl From<(String, String)> for SortField {
    fn from(s: (String, String)) -> Self {
        Self {
            name: s.0,
            alias: Some(s.1),
        }
    }
}

impl From<(&str, &str)> for SortField {
    fn from(s: (&str, &str)) -> Self {
        (s.0.to_string(), s.1.to_string()).into()
    }
}

struct SortValue<'a> {
    field: &'a SortField,
    pub direction: i64,
}

pub struct SortFields {
    fields: Vec<SortField>,
    lookup: HashMap<String, usize>,
}

impl SortFields {
    pub fn new() -> Self {
        Self {
            fields: vec![],
            lookup: HashMap::new(),
        }
    }

    pub fn from(fields: impl IntoIterator<Item = SortField>) -> Self {
        //let mut s = Self::new();

        let mut _fields = vec![];
        let mut _lookup = HashMap::new();

        for field in fields {
            let index = _fields.len();
            _fields.push(field);

            let sort_field = _fields.last().unwrap();
            _lookup.insert(sort_field.name.clone(), index);

            if let Some(ref alias) = sort_field.alias {
                _lookup.insert(alias.clone(), index);
            }
        }

        Self {
            fields: _fields,
            lookup: _lookup,
        }
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&SortField> {
        if let Some(index) = self.lookup.get(key.as_ref()) {
            self.fields.get(*index)
        } else {
            None
        }
    }

    pub fn sort_options(&self, value: &str) -> Result<Document, RequestError> {
        let mut sort = doc! {};

        for v in value.split('+') {
            let sort_value = self.parse_sort_field(v)?;

            let direction: i64 = sort_value.direction.into();
            sort.insert(sort_value.field.name.clone(), direction);
        }

        Ok(sort)
    }

    fn parse_sort_field(&self, value: &str) -> Result<SortValue, RequestError> {
        let (dir, name) = if value.starts_with('-') {
            (0_i64, &value[1..])
        } else {
            (1_i64, value)
        };

        let sort_field = self.get(name).ok_or_else(|| {
            RequestError::builder()
                .message(format!("Invalid sort field: {}", name))
                .build()
        })?;

        Ok(SortValue {
            field: sort_field,
            direction: dir,
        })
    }
}

#[macro_export]
macro_rules! sortfields {
    () => {
        SortFields::new(&[])
    };
    ($($x : expr), + $(,) ?) => {
        crate::fields::SortFields::from([$($x.into()), +])
    };
}
