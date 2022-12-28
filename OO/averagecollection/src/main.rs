pub struct Something {
    list: Vec<u32>,
    average: f32,
}

impl Something {

    fn add(&mut self, x: u32) {
        self.list.push(x);
    }

    fn average(&self) {
        self.average();
    }

    fn average_update(&mut self) {

        let total: u32 = self.list.iter().sum();

        let average = total as f32 / self.list.len() as f32;
    }

    fn remove(&mut self) -> Option<u32> {
        let r = self.list.pop();

        match r {
            Some(v) => {
                self.average_update();
                Some(v) 
            },
            None => None,
        }
    }
}

fn main() {}
