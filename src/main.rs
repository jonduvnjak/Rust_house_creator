#[derive(Debug)]
struct House {
    house_rooms: u32,
    room: Vec<Room>,
}
#[derive(Debug)]
struct Room {
    length: u32,
    width: u32,
}

impl House {
    fn new(house_rooms: u32, room: Vec<Room>) -> House {
        House { house_rooms, room }
    }

    // fn area(&self) -> u32 {
    //     self.room.length * self.room.width
    // }
}

fn main() {
    let house_rooms = 2;
    let room1 = Room {
        length: 3,
        width: 4,
    };

    let room2 = Room {
        length: 3,
        width: 7,
    };

    let house_one = House::new(house_rooms, vec![room1, room2]);
    println!("{:?}", house_one);
    // // println!(
    // //     "total area of house is {:?} metres squared",
    // //     house_one.area()
    // );
}
