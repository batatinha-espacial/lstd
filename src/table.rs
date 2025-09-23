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

fn concat(lua: &Lua, t: LuaVariadic<LuaTable>) -> LuaResult<LuaTable> {
    let t2 = lua.create_table()?;
    let mut index = 1 as LuaInteger;
    for i in t {
        let len = len(lua, i.clone())?;
        let mut j = 1 as LuaInteger;
        while j <= len {
            t2.set(index, i.get::<LuaValue>(j)?)?;
            index += 1;
            j += 1;
        }
    }
    Ok(t2)
}

fn contains(lua: &Lua, (t, v): (LuaTable, LuaValue)) -> LuaResult<bool> {
    let len = len(lua, t.clone())?;
    let mut i = 1 as LuaInteger;
    while i <= len {
        let e = t.get::<LuaValue>(i)?;
        if e.equals(&v)? {
            return Ok(true);
        }
        i += 1;
    }
    Ok(false)
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

fn every(lua: &Lua, (t, f): (LuaTable, LuaFunction)) -> LuaResult<bool> {
    let len = len(lua, t.clone())?;
    let mut i = 1 as LuaInteger;
    while i <= len {
        let e = t.get::<LuaValue>(i)?;
        let b = f.call::<bool>(e)?;
        if !b {
            return Ok(false);
        }
        i += 1;
    }
    Ok(true)
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

fn map(lua: &Lua, (t, f): (LuaTable, LuaFunction)) -> LuaResult<LuaTable> {
    let t2 = lua.create_table()?;
    let len = len(lua, t.clone())?;
    let mut i = 1 as LuaInteger;
    while i <= len {
        let e = t.get::<LuaValue>(i)?;
        let e = f.call::<LuaValue>(e)?;
        t2.set(i, e)?;
        i += 1;
    }
    Ok(t2)
}

fn map_this(lua: &Lua, (t, f): (LuaTable, LuaFunction)) -> LuaResult<()> {
    let t2 = map(lua, (t.clone(), f))?;
    assign(lua, (t, t2))?;
    Ok(())
}

fn pop(lua: &Lua, t: LuaTable) -> LuaResult<LuaValue> {
    let len = len(lua, t.clone())?;
    if len == 0 {
        return Ok(LuaNil);
    }
    let e = t.get::<LuaValue>(len)?;
    t.set(len, LuaNil)?;
    Ok(e)
}

fn push(lua: &Lua, (t, args): (LuaTable, LuaVariadic<LuaValue>)) -> LuaResult<LuaInteger> {
    let mut len = len(lua, t.clone())?;
    for i in args {
        len += 1;
        t.set(len, i)?;
    }
    Ok(len)
}

fn reverse(lua: &Lua, t: LuaTable) -> LuaResult<()> {
    let count = len(lua, t.clone())?;
    let mut i = 0 as LuaInteger;
    while i < count/2 {
        let temp = t.get::<LuaValue>(count-i)?;
        t.set(count-i, t.get::<LuaValue>(i+1)?)?;
        t.set(i+1, temp)?;
        i += 1;
    }
    Ok(())
}

fn reverse_clone(lua: &Lua, t: LuaTable) -> LuaResult<LuaTable> {
    let t = clone(lua, t)?;
    reverse(lua, t.clone())?;
    Ok(t)
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

fn sort(lua: &Lua, (t, f): (LuaTable, LuaFunction)) -> LuaResult<()> {
    let len = len(lua, t.clone())?;
    let a = t;
    let b = lua.create_table()?;
    {
        let mut k = 0;
        while k < len {
            b.set(k+1, a.get::<LuaValue>(k+1)?)?;
            k += 1;
        }
    }
    sort_topdownsplitmerge_(a, 0,len, b, f)?;
    Ok(())
}

fn sort_topdownmerge_(b: LuaTable, ibegin: LuaInteger, imiddle: LuaInteger, iend: LuaInteger, a: LuaTable, f: LuaFunction) -> LuaResult<()> {
    let mut i = ibegin;
    let mut j = imiddle;
    let mut k = ibegin;
    while k < iend {
        if i < imiddle && (j >= iend || f.call::<LuaInteger>((a.get::<LuaValue>(i+1)?, a.get::<LuaValue>(j+1)?))? <= 0) {
            b.set(k+1, a.get::<LuaValue>(i+1)?)?;
            i += 1;
        } else {
            b.set(k+1, a.get::<LuaValue>(j+1)?)?;
            j += 1;
        }
        k += 1;
    }
    Ok(())
}

fn sort_topdownsplitmerge_(b: LuaTable, ibegin: LuaInteger, iend: LuaInteger, a: LuaTable, f: LuaFunction) -> LuaResult<()> {
    if iend - ibegin <= 1 {
        return Ok(());
    }
    let imiddle = (iend + ibegin) / 2;
    sort_topdownsplitmerge_(a.clone(), ibegin, imiddle, b.clone(), f.clone())?;
    sort_topdownsplitmerge_(a.clone(), imiddle, iend, b.clone(), f.clone())?;
    sort_topdownmerge_(b, ibegin, imiddle, iend, a, f)?;
    Ok(())
}

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("assign", lua.create_function(assign)?)?;
    exports.set("concat", lua.create_function(concat)?)?;
    exports.set("contains", lua.create_function(contains)?)?;
    exports.set("clone", lua.create_function(clone)?)?;
    exports.set("deepclone", lua.create_function(deepclone)?)?;
    exports.set("every", lua.create_function(every)?)?;
    exports.set("len", lua.create_function(len)?)?;
    exports.set("map", lua.create_function(map)?)?;
    exports.set("map_this", lua.create_function(map_this)?)?;
    exports.set("pop", lua.create_function(pop)?)?;
    exports.set("push", lua.create_function(push)?)?;
    exports.set("reverse", lua.create_function(reverse)?)?;
    exports.set("reverse_clone", lua.create_function(reverse_clone)?)?;
    exports.set("some", lua.create_function(some)?)?;
    exports.set("sort", lua.create_function(sort)?)?;
    Ok(exports)
}