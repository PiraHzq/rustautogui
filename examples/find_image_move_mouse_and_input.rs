use rustautogui;

fn main() {

    // initialize autogui
    let mut gui = rustautogui::RustAutoGui::new(false);

    // load the image searching for. Region is Option<(startx, starty, width, height)> of search. Matchmode FFT or Segmented (not implemented before 1.0 version), max segments, only important for Segmented match mode
    gui.load_and_prepare_template("test.png", Some((0,0, 500, 300)), rustautogui::MatchMode::FFT, &None);

    // automatically move mouse to found template position in this case it was browser url input field
    gui.find_image_on_screen_and_move_mouse(0.9, 1.0);

    // click the url input field
    gui.left_click();

    // input url
    gui.keyboard_input("test.hr", &false);

    // press enter
    gui.keyboard_command("return");
}