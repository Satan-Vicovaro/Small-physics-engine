#[allow(non_snake_case)]
pub struct KeyPressedAndOptions {

    pub L_shift_down:bool,
    pub W_down:bool,
    pub A_down:bool,
    pub S_down:bool,
    pub D_down:bool,
    pub Q_down:bool,
    pub E_down:bool,
    pub previos_F4_down:bool,
    pub F4_down:bool,
    pub F3_down:bool,
    pub F2_down:bool,
    pub debug_enabled:bool,
    pub quit:bool,
    pub next_frame:bool,
}
impl KeyPressedAndOptions {
    pub fn new() -> Self {
        return KeyPressedAndOptions {
            L_shift_down:false,
            W_down:false,
            A_down:false,
            S_down:false,
            D_down:false,
            Q_down:false,
            E_down:false,
            previos_F4_down:false,
            F4_down:false,
            F3_down:false,
            F2_down:false,
            debug_enabled:false,
            quit:false,
            next_frame:false,
        }
    }
    pub fn toggle(bool:&mut bool) {
        if *bool {
            *bool = false;
        }
        else {
            *bool = true;
        }
    }
}

