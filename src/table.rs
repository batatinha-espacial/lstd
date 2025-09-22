use mlua::prelude::*;

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
    exports.set("len", lua.create_function(len)?)?;
    exports.set("push", lua.create_function(push)?)?;
    Ok(exports)
}