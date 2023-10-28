pub struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    pub fn new(first: String, last: String) -> Person {
        Person {
            first_name: first,
            last_name: last
        }
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }
}