use crate::symbols::Symbol;

#[derive(Copy, Clone)]
pub struct State {
    pub ptr: i32,
    pub arr: [u8; 30000],
}

pub enum Event {
    EnterScope,
    LeaveScope,
}

pub fn simple_step(symb: &Symbol, state: &mut State) {
    match symb {
        Symbol::Add             => { (*state).arr[state.ptr as usize] += 1 },
        Symbol::Sub             => { (*state).arr[state.ptr as usize] -= 1 },
        Symbol::MoveRight       => { (*state).ptr += 1 },
        Symbol::MoveLeft        => { (*state).ptr -= 1 },
        _ => {  },
    };

    if (*state).ptr < 0 {
        (*state).ptr += (state.arr.len() as i32) - 1;
    }

    if (*state).ptr > 29999 {
        (*state).ptr -= (state.arr.len() as i32) - 1;
    }
}

pub fn scope_step(symb: &Symbol) -> Option<Event> {
    match symb {
        Symbol::LeftBracket     => { return Some(Event::EnterScope) },
        Symbol::RightBracket    => { return Some(Event::LeaveScope) },
        _ => { return None }
    }
}

pub fn enter_scope(code: &str, char_index: &mut usize, state: State){
    let initial_chr_index = *char_index;

    if state.arr[state.ptr as usize] != 0 {
        return;
    }

    let code_size: usize = code.len();
    let mut rel_scope_counter: i8 = 0;
    while *char_index < code_size{
        let chr: Option<char> = code.chars().nth(*char_index);
        match chr.unwrap_or(' ') {
            '[' => { rel_scope_counter += 1; }
            ']' => { rel_scope_counter -= 1; }
            _ => { }
        };

        if rel_scope_counter == 0 {
            break;
        }

        *char_index += 1;
    }

    panic!("Unmatched scope @{initial_chr_index}");
}

pub fn leave_scope(code: &str, char_index: &mut usize, state: State){
    let initial_chr_index = *char_index;

    if state.arr[state.ptr as usize] == 0 {
        return;
    }

    let code_size: usize = code.len();
    let mut rel_scope_counter: i8 = 0;
    while *char_index < code_size{
        let chr: Option<char> = code.chars().nth(*char_index);
        match chr.unwrap_or(' ') {
            '[' => { rel_scope_counter -= 1; }
            ']' => { rel_scope_counter += 1; }
            _ => { }
        };

        if rel_scope_counter == 0 {
            return;
        }

        *char_index -= 1;
    }

    panic!("Unmatched scope @{initial_chr_index}");
}

pub fn io_step(symb: &Symbol, state: State){
    let s: &str = &(state.arr[state.ptr as usize] as char).to_string();
    match *symb {
        Symbol::OutputChar => { print!("{s}"); }
        Symbol::InputChar => { todo!(); }
        _ => { }
    }
}