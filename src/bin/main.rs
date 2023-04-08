use chat_room::{RoomManager, Room};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let room_manager = Arc::new(Mutex::new(RoomManager::new()));
    create_room(room_manager.clone(), 1);
    create_room(room_manager.clone(), 2);
    create_room(room_manager.clone(), 3);

    let t1 = tokio::spawn(async move{
        for _ in 0..5 {
            let mut room = room_manager.lock().unwrap();
            let room = room.rooms.get_mut(&1).unwrap();
            room.increment();

        }
    });

    let t2 = tokio::spawn(async move{
        for _ in 0..5 {
            let mut room = room_manager.lock().unwrap();
            let room = room.rooms.get_mut(&1).unwrap();
            room.increment();

        }
    });

    let t3 = tokio::spawn(async move{
    });

    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();

}


pub fn create_room(room_manager: Arc<Mutex<RoomManager>>, index: i32){
    let mut room_manager = room_manager.lock().unwrap();
    let room = Room::new();
    room_manager.rooms.insert(index, room);
}
