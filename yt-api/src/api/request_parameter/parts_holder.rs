use crate::api::request_parameter::collection::{RequestParametersSet, RequestParametersSetter};

#[derive(Clone)]
pub struct PartsHolder<T> {
    items: Vec<T>
}

impl<T> Default for PartsHolder<T> {
    fn default() -> Self {
        PartsHolder { items: Vec::default() }
    }
}

impl<T> RequestParametersSetter for PartsHolder<T> where T: AsRef<str> + Clone {
    fn set<'a>(&self, set: RequestParametersSet) -> RequestParametersSet {
        set.append_iter("part", self.items.clone())
    }
}

// impl<'a, T> From<&PartsHolder<T>> for RequestParameter where T: Clone + AsRef<str> {
//     fn from(ph: &PartsHolder<T>) -> Self {
//         RequestParameter::from_iterator("part", ph.items.clone())
//     }
// }

impl<T> PartsHolder<T> where T: PartialEq {
    pub fn toggle_part(&mut self, part: T) -> &mut Self {
        if self.items.contains(&part) {
            self.items.retain(|p| p != &part)
        } else {
            self.items.push(part)
        }
        self
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}