#![allow(dead_code)]

use std::collections::HashMap;

pub struct Data<T> {
    pub id: usize,
    pub data: T,
}

pub struct Database<T> {
    data_map: HashMap<usize, T>,
    next_id: usize,
}

impl<T> Default for Database<T> {
    fn default() -> Self {
        Database {
            data_map: HashMap::<usize, T>::new(),
            next_id: 0,
        }
    }
}

impl<T> Database<T>
where
    T: Clone,
{
    pub fn len(&self) -> usize {
        self.data_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data_map.len() == 0
    }

    pub fn as_list(&self) -> Vec<Data<T>> {
        let mut list = Vec::new();
        for key in self.data_map.keys() {
            list.push(Data {
                id: *key,
                data: self.get(key).unwrap().clone(),
            });
        }
        list.sort_by(|a, b| a.id.cmp(&b.id));
        list
    }

    pub fn add(&mut self, data: T) -> usize {
        let id = self.get_and_inc_id();
        self.data_map.insert(id, data);
        id
    }

    pub fn get(&self, id: &usize) -> Option<&T> {
        self.data_map.get(id)
    }

    pub fn remove(&mut self, id: &usize) -> bool {
        self.data_map.remove(id).is_some()
    }

    pub fn update(&mut self, id: &usize, data: T) -> bool {
        self.data_map.insert(*id, data).is_some()
    }

    fn get_and_inc_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct TestData {
        v: i32,
    }

    #[test]
    fn test_add() {
        let mut db = Database::<TestData>::default();
        let id = db.add(TestData { v: 1337 });
        assert_eq!(db.get(&id).unwrap().v, 1337);
    }

    #[test]
    fn test_remove() {
        let mut db = Database::<TestData>::default();
        let id = db.add(TestData { v: 1337 });
        assert_eq!(db.get(&id).is_none(), false);
        db.remove(&id);
        assert_eq!(db.get(&id).is_none(), true);
    }

    #[test]
    fn test_update() {
        let mut db = Database::<TestData>::default();
        let id = db.add(TestData { v: 1337 });
        assert_eq!(db.get(&id).unwrap().v, 1337);
        db.update(&id, TestData { v: 666 });
        assert_eq!(db.get(&id).unwrap().v, 666);
    }

    #[test]
    fn test_id_increments_per_add() {
        let mut db = Database::<TestData>::default();
        for expected_id in 0..100 as usize {
            let generated_id = db.add(TestData { v: 0 });
            assert_eq!(generated_id, expected_id);
        }
        for id in 0..100 as usize {
            db.remove(&id);
        }
        for expected_id in 100..200 as usize {
            let generated_id = db.add(TestData { v: 0 });
            assert_eq!(generated_id, expected_id);
        }
    }

    #[test]
    fn test_len_and_empty() {
        let mut db = Database::<TestData>::default();
        assert!(db.is_empty());
        for _ in 0..100 {
            db.add(TestData { v: 0 });
        }
        assert_eq!(db.len(), 100);
        assert!(!db.is_empty());
        for id in 0..50 as usize {
            db.remove(&id);
        }
        assert_eq!(db.len(), 50);
        assert!(!db.is_empty());
        for id in 50..100 as usize {
            db.remove(&id);
        }
        assert!(db.is_empty());
    }

    #[test]
    fn test_as_list_is_sorted() {
        let mut db = Database::<TestData>::default();
        for _ in 0..100 {
            db.add(TestData { v: 0 });
        }
        assert_eq!(db.len(), 100);
        for id in 25..50 as usize {
            db.remove(&id);
        }
        let list = db.as_list();
        assert_eq!(list.len(), 75);
        let mut last_id = None;
        for d in list {
            if last_id.is_some() {
                assert!(d.id > last_id.unwrap());
            }
            last_id = Some(d.id);
        }
    }
}
