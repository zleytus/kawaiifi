use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    title: String,
    value: String,
    subfields: Option<Vec<Field>>,
}

impl Field {
    pub fn new(title: impl Display, value: impl Display) -> Field {
        Field {
            title: title.to_string(),
            value: value.to_string(),
            subfields: None,
        }
    }

    pub fn with_subfields(
        title: impl Display,
        value: impl Display,
        subfields: Vec<Field>,
    ) -> Field {
        Field {
            title: title.to_string(),
            value: value.to_string(),
            subfields: Some(subfields),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn subfields(&self) -> Option<&Vec<Field>> {
        self.subfields.as_ref()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(subfields) = &self.subfields {
            let mut res = write!(f, "{}: {}", self.title, self.value);
            for field in subfields {
                res = write!(f, "\n- {}: {}", field.title, field.value);
            }
            res
        } else {
            write!(f, "{}: {}", self.title, self.value)
        }
    }
}
