pub struct Duel {
    // Whether the duel is currently on
    pub is_on: bool,
    // Number of turns taken since the start of the duel
    pub turn_count: u32,
    // List of errors to be displayed to the user at the start of a turn
    pub err_buffer: Vec<String>,
    // List of messages to be displayed to the user at the start of a turn
    pub msg_buffer: Vec<String>,
}

impl Duel {
    pub fn new() -> Duel {
        Duel {
            is_on: true,
            turn_count: 0,
            err_buffer: Vec::new(),
            msg_buffer: Vec::new(),
        }
    }
}

impl Duel {
    pub fn flush_msg(&mut self) {
        for msg in self.msg_buffer.drain(..) {
            println!("{}", msg);
        }
    }

    pub fn flush_err(&mut self) {
        for msg in self.err_buffer.drain(..) {
            println!("{}", msg);
        }
    }
}
