use enigo::*;

pub fn system_inputs() {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(500, 200);
    enigo.mouse_down(MouseButton::Left);
    enigo.mouse_move_relative(100, 100);
    enigo.mouse_up(MouseButton::Left);
    enigo.key_sequence("gg");
}
