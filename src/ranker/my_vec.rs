use anyhow::Result;
pub struct MyVec {
    vec: Vec<(String, u64)>
}

impl MyVec {
    pub fn new() -> Self {
        Self {vec: Vec::new()}
    }

    pub fn insert(&mut self, elem: (String, u64)){
        for kv in self.vec.iter_mut() {
            if kv.0 == elem.0 {
                kv.1 += elem.1;
                return;
            }
        }

        let mut is_inserted = false;
        for i in 0..self.vec.len() {
            if self.vec[i].1 < elem.1 {
                self.vec.insert(i, elem);
                is_inserted = true;
                if self.vec.len() > 100 {
                    self.vec.pop();
                }
                return;
            }
        }
        if (! is_inserted) && self.vec.len() < 100{
            self.vec.push(elem);
        }

    }

    pub fn unpack(self) -> Vec<(String, u64)> {
        self.vec
    }
}