use std::collections::HashMap;

#[derive(Debug)]
pub struct Room {
    pub count: i32
}

impl Room {
   pub fn new() -> Self{
       return Room { count: 0 };
   }

   pub fn increment(&mut self) {
       self.count += 1;
   }
}


#[derive(Debug)]
pub struct RoomManager{
    pub rooms: HashMap<i32, Room>
}

impl RoomManager {
   pub fn new() -> Self{
       return RoomManager{rooms: HashMap::new()};
   }
}

