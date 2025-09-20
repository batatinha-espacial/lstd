use std::io::{IsTerminal, Stdout, Write};

use mlua::prelude::*;

struct LuaStdout(Stdout);

impl LuaUserData for LuaStdout {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("close", |_, _, _: ()| {
            Ok(())
        });

        methods.add_method("closed", |_, _, _: ()| {
            Ok(false)
        });

        methods.add_method_mut("flush", |_, stdout, _: ()| {
            stdout.0.flush().map_err(|_| LuaError::RuntimeError("couldn't flush".to_string()))
        });

        methods.add_method("readable", |_, _, _: ()| {
            Ok(false)
        });

        methods.add_method_mut("tty", |_, stdout, _: ()| {
            Ok(stdout.0.is_terminal())
        });

        methods.add_method("writable", |_, _, _: ()| {
            Ok(true)
        });

        methods.add_method_mut("write", |_, stdout, args: LuaVariadic<LuaString>| {
            let mut s: Vec<u8> = vec![];
            for i in args {
                s.append(&mut i.as_bytes().to_vec());
            }
            stdout.0.write_all(&s).map_err(|_| LuaError::RuntimeError("couldn't write".to_string()))
        });
    }
}

fn stdout(lua: &Lua, _: ()) -> LuaResult<LuaValue> {
    LuaStdout(std::io::stdout()).into_lua(lua)
}

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("stdout", lua.create_function(stdout)?)?;
    Ok(exports)
}