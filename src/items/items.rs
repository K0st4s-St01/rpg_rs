#[derive(PartialEq, Eq, PartialOrd, Ord,Clone,Copy)]
pub enum Item_category {
   one_handed_weapon,
   two_handed_weapon,
   shield,
   staff,
   helmet,
   thorax,
   boots,
   consumable, 
   bow,
   gloves,
   arrows
}
#[derive(PartialEq, Eq, PartialOrd, Ord,Clone)]
pub struct Item{
    name:String,
    category:Item_category,
    armour:i32,
    damage:i32,
    health:i32,
}
impl Item{
    pub fn new(name:String,category:Item_category,armour:i32,damage:i32,health:i32) -> Item{
        Item { name: name, category: category, armour: armour, damage: damage, health: health }
    }
    pub fn damage(&self) -> i32{
        self.damage
    }
    pub fn armour(&self) -> i32{
        self.armour
    }
}