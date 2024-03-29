pub mod structs;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn todo_factory(item_type: String, item_title: String) -> Result<ItemTypes, &'static str> {
    if item_type == "pending" {
        let pending_item = Pending::new(item_title);
        Ok(ItemTypes::Pending(pending_item))
    } else if item_type == "done" {
        let done_item = Done::new(item_title);
        Ok(ItemTypes::Done(done_item))
    } else {
        Err("this is not accepted")
    }
}
