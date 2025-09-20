use mlua::prelude::*;

static ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

static ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

static ASCII_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn capitalize(_: &Lua, s: String) -> LuaResult<String> {
    let mut chars = s.chars();
    Ok(match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    })
}

fn center(_: &Lua, (s, width, fill): (String, LuaInteger, Option<String>)) -> LuaResult<String> {
    let len = s.chars().count();
    if width as usize <= len {
        return Ok(s.to_string());
    }

    let pad = width as usize - len;
    let left = pad / 2;
    let right = pad - left;
    let fill = match fill {
        None => ' ',
        Some(fill) => match fill.len() {
            0 => ' ',
            _ => fill.chars().next().unwrap(),
        },
    };
    Ok(format!(
        "{}{}{}",
        fill.to_string().repeat(left),
        s,
        fill.to_string().repeat(right)
    ))
}

fn contains(_: &Lua, (string, search): (String, String)) -> LuaResult<bool> {
    Ok(string.contains(&search))
}

fn count(_: &Lua, (s, sub): (String, String)) -> LuaResult<LuaInteger> {
    if sub.is_empty() {
        Ok(s.chars().count() as LuaInteger + 1)
    } else {
        Ok(s.matches(&sub).count() as LuaInteger)
    }
}

static DIGITS: &str = "0123456789";

fn endswith(_: &Lua, (s, suffix): (String, String)) -> LuaResult<bool> {
    Ok(s.ends_with(&suffix))
}

fn expandtabs(_: &Lua, (s, tabsize): (String, Option<LuaInteger>)) -> LuaResult<String> {
    let tabsize = match tabsize {
        Some(tabsize) => tabsize as usize,
        None => 8,
    };
    let mut result = String::new();
    let mut col = 0;

    for ch in s.chars() {
        if ch == '\t' {
            let spaces = tabsize - (col % tabsize);
            for _ in 0..spaces {
                result.push(' ');
            }
            col += spaces;
        } else {
            result.push(ch);
            col += 1;
        }
        if ch == '\n' {
            col = 0;
        }
    }

    Ok(result)
}

fn find(_: &Lua, (s, sub): (String, String)) -> LuaResult<LuaInteger> {
    if sub.is_empty() {
        Ok(1)
    } else {
        Ok(match s.find(&sub) {
            None => 0,
            Some(i) => s[..i].chars().count() as LuaInteger + 1,
        })
    }
}

fn index(_: &Lua, (s, i): (String, LuaInteger)) -> LuaResult<String> {
    match s.chars().nth(i as usize - 1) {
        None => Err(LuaError::RuntimeError("index out of bounds".to_string())),
        Some(c) => Ok(c.to_string()),
    }
}

fn isascii(_: &Lua, s: String) -> LuaResult<bool> {
    Ok(s.is_ascii())
}

fn len(_: &Lua, s: String) -> LuaResult<LuaInteger> {
    Ok(s.chars().count() as LuaInteger)
}

fn max(_: &Lua, s: String) -> LuaResult<String> {
    Ok(s.chars().max().unwrap_or('\0').to_string())
}

fn min(_: &Lua, s: String) -> LuaResult<String> {
    Ok(s.chars().min().unwrap_or('\0').to_string())
}

fn rep(_: &Lua, (s, n): (String, LuaInteger)) -> LuaResult<String> {
    Ok(s.repeat(n as usize))
}

fn slice(_: &Lua, (s, i, j): (String, LuaInteger, LuaInteger)) -> LuaResult<String> {
    let s = s.chars().collect::<Vec<char>>();
    match s.get((i as usize - 1)..(j as usize - 1)) {
        None => Err(LuaError::RuntimeError("index out of bounds".to_string())),
        Some(s) => Ok(s.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("")),
    }
}

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("asciiletters", ASCII_LETTERS)?;
    exports.set("asciilowercase", ASCII_LOWERCASE)?;
    exports.set("asciiuppercase", ASCII_UPPERCASE)?;
    exports.set("capitalize", lua.create_function(capitalize)?)?;
    exports.set("center", lua.create_function(center)?)?;
    exports.set("contains", lua.create_function(contains)?)?;
    exports.set("count", lua.create_function(count)?)?;
    exports.set("digits", DIGITS)?;
    exports.set("endswith", lua.create_function(endswith)?)?;
    exports.set("expandtabs", lua.create_function(expandtabs)?)?;
    exports.set("find", lua.create_function(find)?)?;
    exports.set("isascii", lua.create_function(isascii)?)?;
    exports.set("index", lua.create_function(index)?)?;
    exports.set("len", lua.create_function(len)?)?;
    exports.set("max", lua.create_function(max)?)?;
    exports.set("min", lua.create_function(min)?)?;
    exports.set("rep", lua.create_function(rep)?)?;
    exports.set("slice", lua.create_function(slice)?)?;
    Ok(exports)
}