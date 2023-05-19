use std::sync::{Arc, Weak};

use crate::{observable::Observable, observer::Observer};

pub struct Subject {
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    state: String,
}

impl Subject {
    pub fn new(state: &str) -> Self {
        Self {
            observers: vec![],
            state: state.into(),
        }
    }

    pub fn state(&self) -> &str {
        self.state.as_ref()
    }
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self>>;
    fn update(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade())
            .for_each(|o| o.observe(self));
    }
    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }
    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
}
