pub mod items;

enum Item_category {
   one_handed_weapon,
   two_handed_weapon,
   shield,
   staff,
   helmet,
   thorax,
   boots,
   consumable, 
   bow,
   arrows
}

pub struct Item{
    name:String,
    category:Item_category,
    armour:i32,
    damage:i32,
    health:i32,
}
impl Item{
    fn new(name:String,category:Item_category,armour:i32,damage:i32,health:i32) -> Item{
        Item { name: name, category: category, armour: armour, damage: damage, health: health }
    }
}