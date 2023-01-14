pub struct Food {
    pub postion: (i32, i32),
}

impl Food {
    pub fn new(x: i32, y: i32) -> Food {
        Food {
            postion: (x, y),
        }
    } 
}