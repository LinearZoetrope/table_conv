extern crate rlua;

const LUA_SRC: &'static str = r#"
function a_table()
    return {
        hp = 150.0,
        max_hp = 150.0,
        damage = { min = 10.0, max = 15.0 },
        range = 20.0,
        move = true,
        trig = Not(DealDamage(12.5))
    }
end
"#;

// const LUA_PRELUDE: &'static str = r#"
// Not = {}
// Not.__index = Not

// setmetatable(Not, {
//   __call = function (cls, ...)
//     return cls.new(...)
//   end,
// })

// Not:_init(trigger)
//     self = __sky_new_not_trigger(trigger)
// end

// DealDamage = {}
// DealDamage.__index = DealDamage

// setmetatable(DealDamage, {
//   __call = function (cls, ...)
//     return cls.new(...)
//   end,
// })

// DealDamage:init(dmg)
//     self = __sky_new_deal_dmg_trigger(dmg)
// end
// "#;

use rlua::UserData;

#[derive(Clone, PartialEq)]
enum Trigger {
    Not(Box<Trigger>),
    DealDamage{dmg: f64},
}

impl UserData for Trigger {}

fn main() {
    use rlua::{Lua, Table};

    let lua = Lua::new();

    let new_not = lua.create_function(|_, trigger| {
        Ok(Trigger::Not(Box::new(trigger)))
    });

    lua.globals().set("Not", new_not).unwrap();

    let new_deal_dmg = lua.create_function(|_, dmg| {
        Ok(Trigger::DealDamage{dmg})
    });

    lua.globals().set("DealDamage", new_deal_dmg).unwrap();

    // lua.exec::<()>(LUA_PRELUDE, None).expect("Could not run prelude");

    lua.exec::<()>(LUA_SRC, None).expect("Could not run Lua");

    let table: Table = lua.eval("a_table()", None).expect("Could not eval table");

    let hp: f64 = table.get("hp").unwrap();
    assert_eq!(hp, 150.0);

    let max_hp: f64 = table.get("max_hp").unwrap();
    assert_eq!(max_hp, 150.0);

    let damage: Table = table.get("damage").unwrap();
    let min_dmg: f64 = damage.get("min").unwrap();
    let max_dmg: f64 = damage.get("max").unwrap();

    assert_eq!(min_dmg, 10.0);
    assert_eq!(max_dmg, 15.0);

    let range: f64 = table.get("range").unwrap();
    assert_eq!(range, 20.0);

    let mov: bool = table.get("move").unwrap();
    assert!(mov);

    let trigger: Trigger = table.get("trig").unwrap();

    let expected = Trigger::Not(Box::new(Trigger::DealDamage{dmg: 12.5}));
    assert!(expected == trigger);

    println!("Lua works!");
}