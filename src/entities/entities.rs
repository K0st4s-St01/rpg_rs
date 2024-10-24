use std::collections::HashMap;

use crate::{items::items::{Item, Item_category}, skills::skills::Class};

pub struct Entity{
    name:String,
    hp:i32,
    base_dmg:i32,
    base_armor:i32,
    class:Class,
    equipment:Equipment,
    bag:Bag
}
impl Entity{
    pub fn new(name:String,hp:i32,base_dmg:i32,base_armor:i32,equipment:Equipment,bag:Bag,class:Class)->Entity{
        Entity{
            name:name,
            hp:hp,
            base_armor:base_armor,
            base_dmg:base_dmg,
            equipment:equipment,
            bag:bag,
            class:class
        }
    }
    pub fn damage(self) -> i32{
        self.base_dmg+self.equipment.damage()
    }
}
pub enum Equip{
    LeftHand,
    RightHand,
    Helmet,
    Thorax,
    Gloves,
    Boots,

}
pub struct Equipment{
    items:HashMap<Equip,Option<Item>>
}
impl Equipment{
    pub fn damage(self) -> i32{
        let mut damage:i32 = 0;
        for value in self.items.into_values(){
            if value!= None{
                damage+=value.unwrap().damage();
            }
        }
        damage
    }
    pub fn armor(self) -> i32{
        let mut armor = 0;
        for value in  self.items.into_values(){
            if value != None{
                armor+=value.unwrap().armour();
            }
        }
        armor
    }
    
}
pub struct Bag{
    items:[[Item;4];5]
}