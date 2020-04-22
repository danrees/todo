use serde::{Serialize,Deserialize};
use std::fs::{ OpenOptions, File};
use std::io::{ BufRead, BufReader};

#[derive(Serialize,Deserialize,Debug, PartialEq)]
enum Status {
    Complete,
    Incomplete,
}

pub trait Store {
    fn create(&mut self, description: String) -> Item;
    // fn update(&mut self, item: Item);
    // fn delete(&mut self, item: Item);
    // fn get(&self,id: u64) -> Option<&Item>;
    fn list(&self) -> Vec<Item>;
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Item {
    id: u64,
    description: String,
    status: Status,
}

pub struct FileStore<'a> {
    next_id: u64,
    path: &'a str,
}

impl<'a> FileStore<'a> {
    pub fn new(path: &'a str) -> Self{
        FileStore{
            next_id: 0,
            path,
        }
    }
}

impl<'a> Store for FileStore<'a> {
    fn create(&mut self, description: String) -> Item {
        let file = OpenOptions::new().append(true).create(true).open(self.path).unwrap();
        self.next_id += 1;
        let item = Item{
            id: self.next_id,
            description,
            status: Status::Incomplete,
        };
        serde_json::to_writer(file,&item).unwrap();
        item
    }

    fn list(&self) -> Vec<Item> {
        let file = File::open(self.path).unwrap();
        let reader = BufReader::new(file);
        let mut items = Vec::new();
        for (_index,line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let item: Item = serde_json::from_str(line.as_str()).unwrap();
            items.push(item);
        }
        return items
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_filestore_create() {
        let data = tempdir().unwrap();
        let path = data.path().join("data.db");
            let path_loc = path.as_os_str().to_str().unwrap();

        let mut file_store = FileStore::new(path_loc);
        let _item = file_store.create(String::from("new todo"));
        let test_me = fs::read_to_string(path).unwrap();
        assert_eq!(test_me, String::from(r#"{"id":1,"description":"new todo","status":"Incomplete"}"#))
    }

    #[test]
    fn test_filestore_list() {
        let data = tempdir().unwrap();
        let path = data.path().join("data.db");
        let contents =r#"{"id":1,"description":"new todo","status":"Incomplete"}
        {"id":2,"description":"new todo 2","status":"Incomplete"}"#;
        let _result = fs::write(&path,contents);

        let file_store = FileStore::new(path.as_os_str().to_str().unwrap());
        let items = file_store.list();

        assert_eq!(*items.get(0).unwrap(), Item{status: Status::Incomplete, id: 1, description: String::from("new todo")})
    }
}