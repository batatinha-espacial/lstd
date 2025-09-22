use mlua::prelude::*;

fn clone(lua: &Lua, t: LuaTable) -> LuaResult<LuaTable> {
    let t2 = lua.create_table()?;
    for i in t.pairs::<LuaValue, LuaValue>() {
        let (k, v) = i?;
        t2.set(k, v)?;
    }
    Ok(t2)
}

fn deepclone(lua: &Lua, t: LuaTable) -> LuaResult<LuaTable> {
    let t2 = lua.create_table()?;
    for i in t.pairs::<LuaValue, LuaValue>() {
        let (k, v) = i?;
        match v {
            LuaValue::Table(v) => {
                t2.set(k, deepclone(lua, v)?)?;
            },
            _ => t2.set(k, v)?,
        }
    }
    Ok(t2)
}

fn len(_: &Lua, t: LuaTable) -> LuaResult<LuaInteger> {
    let mut i = 0usize;
    loop {
        match t.get::<LuaValue>(i+1)? {
            LuaNil => break,
            _ => {
                i += 1;
            },
        }
    }
    Ok(i as LuaInteger)
}

fn push(lua: &Lua, (t, args): (LuaTable, LuaVariadic<LuaValue>)) -> LuaResult<LuaInteger> {
    let mut len = len(lua, t.clone())?;
    for i in args {
        len += 1;
        t.set(len, i)?;
    }
    Ok(len)
}

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("clone", lua.create_function(clone)?)?;
    exports.set("deepclone", lua.create_function(deepclone)?)?;
    exports.set("len", lua.create_function(len)?)?;
    exports.set("push", lua.create_function(push)?)?;
    Ok(exports)
}