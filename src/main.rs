mod interpreter;
mod symbols;

use interpreter::State;
use interpreter::Event;

use symbols::Symbol;

fn main() {
    let code: &str = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
    +.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
    ]<+.";
    let mut state: State = State {
        ptr: 0,
        arr: [0; 30000],
    };

    let code_length: usize = code.len();
    let mut curr_char_index: usize = 0;

    while curr_char_index < code_length {
        let c: char = code.chars().nth(curr_char_index).unwrap_or(' ');

        let symb: Symbol = symbols::char_to_symbol(c);
        
        interpreter::io_step(&symb, state);
        interpreter::simple_step(&symb, &mut state);
        
        let ev: Option<Event> = interpreter::scope_step(&symb);
        if ev.is_some() { 
            match ev.unwrap() {
                Event::EnterScope => {
                    interpreter::enter_scope(code, &mut curr_char_index, state);
                }
                Event::LeaveScope => {
                    interpreter::leave_scope(code, &mut curr_char_index, state);
                }
            };
        };
        curr_char_index += 1;
    }

    println!("\nCode finished")
}
