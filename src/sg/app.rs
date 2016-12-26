pub struct App {
    headless: bool,
}

impl App {

    pub fn new(headless: bool) -> Self {
        App { headless: headless }
    }

    pub fn start(&self) -> i32 {
        match self.headless {
            true => { println!("Running in headless mode!") }
            false => { println!("Running in UI mode!") }
        }
        0
    }

}
