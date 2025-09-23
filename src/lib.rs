use std::io::Write;

use chromium_base64_rs::{decode::base64_decode, encode::base64_encode};
use mlua::prelude::*;

fn atob(lua: &Lua, s: LuaString) -> LuaResult<LuaString> {
    lua.create_string(base64_decode(&s.as_bytes()).as_slice())
}

fn btoa(lua: &Lua, s: LuaString) -> LuaResult<LuaString> {
    lua.create_string(base64_encode(&s.as_bytes()).as_slice())
}

fn chr(_: &Lua, u: LuaInteger) -> LuaResult<String> {
    match char::from_u32(u as u32) {
        Some(c) => Ok(c.to_string()),
        None => Err(LuaError::RuntimeError("invalid codepoint".to_string()))
    }
}

fn clock(_: &Lua, _: ()) -> LuaResult<LuaInteger> {
    let time = std::time::SystemTime::now();
    Ok(match time.duration_since(std::time::UNIX_EPOCH) {
        Ok(r) => r.as_secs() as LuaInteger,
        Err(e) => -(e.duration().as_secs() as LuaInteger)
    })
}

fn cwd(_: &Lua, _: ()) -> LuaResult<String> {
    Ok(std::env::current_dir().map_err(|_| LuaError::RuntimeError("unable to get cwd".to_string()))?.display().to_string())
}

fn eprint(_: &Lua, args: LuaVariadic<String>) -> LuaResult<()> {
    eprintln!("{}", args.join("\t"));
    Ok(())
}

fn eprintnnl(_: &Lua, args: LuaVariadic<String>) -> LuaResult<()> {
    eprint!("{}", args.join("\t"));
    let _ = std::io::stderr().flush();
    Ok(())
}

mod io;

fn ord(_: &Lua, s: String) -> LuaResult<LuaInteger> {
    if s.is_empty() {
        Err(LuaError::RuntimeError("empty string was passed".to_string()))
    } else {
        Ok(s.chars().next().unwrap() as u32 as LuaInteger)
    }
}

fn print(_: &Lua, args: LuaVariadic<String>) -> LuaResult<()> {
    println!("{}", args.join("\t"));
    Ok(())
}

fn printnnl(_: &Lua, args: LuaVariadic<String>) -> LuaResult<()> {
    print!("{}", args.join("\t"));
    let _ = std::io::stdout().flush();
    Ok(())
}

mod string;

mod table;

#[mlua::lua_module(name = "lstd")]
fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("atob", lua.create_function(atob)?)?;
    exports.set("btoa", lua.create_function(btoa)?)?;
    exports.set("chr", lua.create_function(chr)?)?;
    exports.set("clock", lua.create_function(clock)?)?;
    exports.set("cwd", lua.create_function(cwd)?)?;
    exports.set("eprint", lua.create_function(eprint)?)?;
    exports.set("eprintnnl", lua.create_function(eprintnnl)?)?;
    exports.set("io", io::module(lua)?)?;
    exports.set("ord", lua.create_function(ord)?)?;
    exports.set("print", lua.create_function(print)?)?;
    exports.set("printnnl", lua.create_function(printnnl)?)?;
    exports.set("string", string::module(lua)?)?;
    exports.set("table", table::module(lua)?)?;
    Ok(exports)
}