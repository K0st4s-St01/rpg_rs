pub enum Effects {
    heal,
    burn,
    freeze,
    stun,
    raise_undead,
    absorve_health,
    absorve_mana,
    bleeding,
    shield,
    attack,
    armor,
    blind,
    invinsibillity
}

struct Effect{
    name:String,
    magnitude:i32,
    active:bool,
}
impl Effect {
    pub fn new(name:String,magnitude:i32,active:bool) -> Effect{
        Effect{
            name:name,
            magnitude:magnitude,
            active:active
        }
    }
}

enum Skill_category{
    one_handed,
    two_handed,
    block,
    heavy_armor,
    light_armor,
    sneak,
    athletics,
	marksman,
	alchemy,
	illusion,
	destruction,
	restoration,
	conjuration,
	anti_magic,
}
impl Skill_category {
    fn all_skill_categories() ->  [Skill_category;14] {
    [
        Skill_category::one_handed,
        Skill_category::two_handed,
        Skill_category::block,
        Skill_category::heavy_armor,
        Skill_category::light_armor,
        Skill_category::sneak,
        Skill_category::athletics,
        Skill_category::marksman,
        Skill_category::alchemy,
        Skill_category::illusion,
        Skill_category::destruction,
        Skill_category::restoration,
        Skill_category::conjuration,
        Skill_category::anti_magic,
    ]
}
}

pub struct Skill{
    level:i32,
    skill_category:Skill_category,
    required_proficiency:i32,
    effect:Effect
}
impl Skill {
    pub fn new(level:i32,skill_category:Skill_category,required_proficiency:i32,effect:Effect) -> Skill{
        Skill { 
            level: level,
            skill_category: skill_category, 
            required_proficiency: required_proficiency, 
            effect: effect 
            }
    }
}

pub struct Proficiency{
    category:Skill_category,
    value:i32,
}
impl Proficiency {
    pub fn new(category:Skill_category,value:i32) -> Proficiency{
        Proficiency { category: category, value: value }
    }
}
pub enum Class_name{
    knight,
    hunter,
    sorcerer,
    necromancer,
    assassing,
    witch_hunter
}

pub struct Class{
    name:Class_name,
    profiecencies:[Option<Proficiency>;14]
}
impl Class{
    pub fn new(name:Class_name)->Class{
        let mut proficiences:[Option<Proficiency>;14]=[const { None };14];
        let mut i =0;
        for skill_category in Skill_category::all_skill_categories(){
            proficiences[i] = Some(Proficiency::new(skill_category, 15));
            i+=1;
        }
        Class{
            name:name,
            profiecencies:proficiences,
        }
    }
}
