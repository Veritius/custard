use mlua::Lua;

pub struct State {
    pub lua: Lua,
}

impl State {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
        }
    }
}