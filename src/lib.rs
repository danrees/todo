use serde::{Serialize,Deserialize};
use std::fs::{ OpenOptions, File};
use std::io::{ BufRead, BufReader};
use uuid::Uuid;

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
    id: String,
    description: String,
    status: Status,
}

pub struct FileStore {
    path: String,
}

impl FileStore {
    pub fn new(path: String) -> Self{
        FileStore{
            path,
        }
    }
}

impl Store for FileStore {
    fn create(&mut self, description: String) -> Item {
        let file = OpenOptions::new().append(true).create(true).open(&self.path).unwrap();
        let id = Uuid::new_v4().to_string();
        let item = Item{
            id,
            description,
            status: Status::Incomplete,
        };
        serde_json::to_writer(file,&item).unwrap();
        item
    }

    fn list(&self) -> Vec<Item> {
        let file = File::open(&self.path).unwrap();
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
        let path = String::from(data.path().join("data.db").to_str().unwrap());


        let mut file_store = FileStore::new(path.clone());
        let item = file_store.create(String::from("new todo"));
        let test_me = fs::read_to_string(&path).unwrap();
        let expected = format!(r#"{{"id":"{}","description":"new todo","status":"Incomplete"}}"#, item.id);
        assert_eq!(test_me, expected)
    }

    #[test]
    fn test_filestore_list() {
        let data = tempdir().unwrap();
        let path = data.path().join("data.db");
        let contents =r#"{"id":"1","description":"new todo","status":"Incomplete"}
        {"id":"2","description":"new todo 2","status":"Incomplete"}"#;
        let _result = fs::write(&path,contents);

        let file_store = FileStore::new(String::from(path.to_str().unwrap()));
        let items = file_store.list();

        assert_eq!(*items.get(0).unwrap(), Item{status: Status::Incomplete, id: String::from("1"), description: String::from("new todo")})
    }
}