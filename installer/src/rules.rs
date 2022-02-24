use std::fmt::*;

#[derive(Debug)]
pub struct Rules<'a> {
    pub titleids: &'a Vec<String>,
    pub name: &'a String,
    pub path: &'a String,
    pub description: &'a String,
    pub version: &'a u8
}

impl<'a> Rules<'a> {
    pub fn new(ids: &'a Vec<String>, name: &'a String, path: &'a String, desc: &'a String, ver: &'a u8) -> Self {
        return Self {
            titleids: ids,
            name: name,
            path: path,
            description: desc,
            version: ver
        };
    }
}

impl Display for Rules<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[Definition]\ntitleIds = ").unwrap();
        for i in 0..self.titleids.len() {
            let id = self.titleids.get(i).unwrap();
            if i != 2 {
                write!(f, "{},", id).unwrap();
            } else {
                write!(f, "{}", id).unwrap();
            }
        }
        write!(f, "\nname = {}\n", self.name).unwrap();
        write!(f, "path = \"{}\"\n", self.path).unwrap();
        write!(f, "description = {}\n", self.description).unwrap();
        return write!(f, "version = {}", self.version);
    }
}