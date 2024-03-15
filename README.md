![demo_0](gif/demo_0.gif)
<h1 align="center">
Firework-rs
<br>
</h1>

Firework-rs is a cross-platform ascii-art firework simulator in terminal. Run the binary or use the library to create your own firework, and just enjoy the beautiful fireworks in your terminal!

## Features

 - Colorful ASCII art firework
 - Smooth animation
 - Customizable fireworks
 - Simple particle system letting you make fireworks but not only fireworks

## Try Out a Demo

Install [rust](https://www.rust-lang.org/tools/install) if you havn't.

Then, simply run the following commands:

```
git clone https://github.com/Wayoung7/firework-rs.git
cd firework-rs
cargo run --release -- --demo 0
```

or to install globally on your computer:

```
cargo install firework-rs
firework --demo 0
```

The binary now has **4 demos**, from **0** to **3**. 

## Exit

To exit the program, simply press `ESC`

## Command Line Arguments

```
USAGE:
firework [OPTIONS] --demo <DEMO-NUMBER>

Options:
    -l, --looping
        Make the fireworks show loop infinitely.

    -g, --gradient
        Enable the color gradient of fireworks.
        If this is enabled, it is recommanded that your terminal is non-transparent and has black background color to get better visual effects.

    -d, --demo <DEMO-NUMBER>
        Select which demo to run.

    -h, --help
        Print help (see a summary with '-h' and full with '--help').

    -V, --version
        Print version.
```

### Example Commands

If you have installed the binary:

```
firework -l -g --demo 2
```

If you have not installed the binary:

First `cd` into the project root directory, and then run:

```
cargo run --release -- -l -g --demo 2
```

## Use the Library

This package not only has a demo binary for you to enjoy terminal fireworks, but also provides you with a simple library **firework_rs** to play with your own fireworks.

To add this crate to your rust project, run:

```
cargo add firework_rs
```

in your project root directory.

To make a firework, you can simply use the following structure:

```
fn main() -> Result<()> {
    // Terminal stuff, no need to change
    let mut stdout = stdout();
    let (_width, _height) = terminal::size()?;
    let mut is_running = true;

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut time = SystemTime::now();
    let mut term = Terminal::default();

    // Init and add fireworks
    let mut fm = FireworkManager::default().add_firework(gen());

    // Main loop, no need to change
    while is_running {
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                event::Event::Key(e) => {
                    if e.code == KeyCode::Esc {
                        is_running = false;
                    }
                }
                event::Event::Resize(_, _) => {
                    fm.reset();
                    term.reinit();
                }
                _ => {}
            };
        }

        let delta_time = SystemTime::now().duration_since(time).unwrap();
        fm.update(time, delta_time);
        time = SystemTime::now();

        term.render(&fm);
        term.print(&mut stdout);

        if delta_time < Duration::from_secs_f32(0.05) {
            let rem = Duration::from_secs_f32(0.05) - delta_time;
            sleep(rem);
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

// Your actuall firework design goes here, see docs for more information
fn gen() -> Firework {
    let colors = vec![
        ...
    ];
    let particles = ...
    let config = ...

    Firework {
        ...
    }
}
```

### Examples

The package provide several examples under `examples/` showing some features of the library, and give you some inspiration.

To run examples, `cd` into the this project directory, and simply type:

```
cargo run --example <EXAMPLE-NAME>
```

**Example-name** contains:

fountain

![fountain](gif/fountain.gif)

vortex

![vortex](gif/vortex.gif)

heart

![heart](gif/heart.gif)

## Compatibility

### Operating System

This program can be run on Windows / Mac OS / Linux.

### Terminal
This crate uses [crossterm](https://github.com/crossterm-rs/crossterm) as backend. Terminals that crossterm supports will also be supported by this crate.

This crate supports all UNIX terminals and Windows terminals down to Windows 7. however, not all of the terminals have been tested and has good viusal quality. 

It is recommanded to use terminal that has GPU rendering acceleration, like [Kitty](https://github.com/kovidgoyal/kitty) and [Alacritty](https://github.com/alacritty/alacritty). Make sure your terminal does not have extra color theme or adjustment. If you enable gradient in the program, make sure the terminal window is **non-transparent** and has **black background**.

## Help

Feel free to open an issue or contact me if you find any bugs.




