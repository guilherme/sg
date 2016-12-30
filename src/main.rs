extern crate clap;
extern crate ncurses;

use clap::App as Cli;
use clap::Arg;
use ncurses::*;

fn main() {

  let matches = Cli::new("SG")
                        .version("0.1")
                        .author("Sebastian Glazebrook, Guilherme Reis Campos")
                        .about("The best fuzzy finder ever")
                        .arg(Arg::with_name("headless")
                             .short("h")
                             .long("headless")
                             .help("Run without a UI. Commonly used for testing"))
                        .arg(Arg::with_name("filter")
                             .short("f")
                             .long("filter")
                             .takes_value(true)
                             .value_name("FILTER")
                             .help("Filter string to filter the input"))
                        .arg(Arg::with_name("input")
                             .short("i")
                             .long("input")
                             .takes_value(true)
                             .value_name("INPUT")
                             .help("Input to filter, defaults to STDIN"))
                        .arg(Arg::with_name("return")
                             .short("r")
                             .long("return")
                             .takes_value(true)
                             .value_name("RETURN_TYPE")
                             .help("Configure what you want to return.\nDefault: 'selected-rows'\nAvailable options: 'all-rows', 'selected-rows'"))
                        .get_matches();

  initscr();
  noecho();
  keypad(stdscr(), true);

  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

  /* Status/help info. */
  refresh();

  /* Get the screen bounds. */
  let mut max_x = 0;
  let mut max_y = 0;
  /* get window bounds */
  getmaxyx(stdscr(), &mut max_y, &mut max_x);

  // TODO: colorize the lines
  /* Start in the center. */
  /* create a box */
  let start_y = (max_y) - 1;
  let start_x = (max_x) - 1;
  let win = newwin(start_y, start_x, 0, 0);
  scrollok(win, true);
  wrefresh(win);
  mvwprintw(win, start_y - 1, 1, "ABC");
  mvwprintw(win, start_y - 2, 1, "123");
  mvwprintw(win, start_y - 3, 1, "CDE");
  mvwprintw(win, start_y - 4, 1, "456");
  mvwprintw(win, start_y - 5, 1, "ABC");
  mvwprintw(win, start_y - 6, 1, "123");
  mvwprintw(win, start_y - 7, 1, "CDE");
  mvwprintw(win, start_y - 8, 1, "456");
  mvwprintw(win, start_y - 9, 1, "ABC");
  mvwprintw(win, start_y - 10, 1, "123");
  mvwprintw(win, start_y - 11, 1, "CDE");
  mvwprintw(win, start_y - 12, 1, "456");
  let mut currentLineX = 1;
  let mut currentLineY = start_y - 1;
  mvwchgat(win,currentLineY, 0, -1, A_STANDOUT(), 1);
  wrefresh(win);

  let mut string = &mut String::new();
  // colorize the status bar.
  let mut ch = getch();
  while ch != KEY_ENTER
  {
    match ch {
      KEY_UP => {
        // TODO: ADD BOUNDARIES
        mvwchgat(win,currentLineY,0,-1, A_NORMAL(), 1);
        currentLineY = currentLineY - 1;
        wmove(win, currentLineY, 0);
        mvwchgat(win,currentLineY,0,-1, A_STANDOUT(), 0);

      },
      KEY_DOWN => {
        // TODO: ADD BOUNDARIES
        mvwchgat(win,currentLineY,0,-1, A_NORMAL(), 1);
        currentLineY = currentLineY + 1;
        wmove(win,currentLineY, currentLineX);
        mvwchgat(win, currentLineY,0,-1, A_STANDOUT(), 0);
      },
      _ => {

      }
    }
    wrefresh(win);
    ch = getch();
  }

  endwin();
}
