use mlua::prelude::*;

fn assign(_: &Lua, (outt, int): (LuaTable, LuaTable)) -> LuaResult<()> {
    for i in int.pairs::<LuaValue, LuaValue>() {
        let (k, v) = i?;
        outt.set(k, v)?;
    }
    if let Some(m) = int.metatable() {
        outt.set_metatable(Some(m))?;
    }
    Ok(())
}

fn clone(lua: &Lua, t: LuaTable) -> LuaResult<LuaTable> {
    let t2 = lua.create_table()?;
    for i in t.pairs::<LuaValue, LuaValue>() {
        let (k, v) = i?;
        t2.set(k, v)?;
    }
    t2.set_metatable(t.metatable())?;
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
    t2.set_metatable(t.metatable())?;
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

fn some(lua: &Lua, (t, f): (LuaTable, LuaFunction)) -> LuaResult<bool> {
    let len = len(lua, t.clone())?;
    let mut i = 1 as LuaInteger;
    while i <= len {
        let e = t.get::<LuaValue>(i)?;
        let b = f.call::<bool>(e)?;
        if b {
            return Ok(true);
        }
        i += 1;
    }
    Ok(false)
}

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("assign", lua.create_function(assign)?)?;
    exports.set("clone", lua.create_function(clone)?)?;
    exports.set("deepclone", lua.create_function(deepclone)?)?;
    exports.set("len", lua.create_function(len)?)?;
    exports.set("push", lua.create_function(push)?)?;
    exports.set("some", lua.create_function(some)?)?;
    Ok(exports)
}