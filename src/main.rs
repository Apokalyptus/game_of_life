use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::{thread, time};

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

fn init_playfield(v_playfield: &mut [u8]) {
    for (_i, elem) in v_playfield.iter_mut().enumerate() {
        if rand::random::<f32>() < 0.3 {  // 30% Wahrscheinlichkeit für lebende Zellen
            *elem = 1;
        } else {
            *elem = 0;
        }
    }
}

/*
/* GLIDER */
fn init_playfield(v_playfield: &mut [u8]) {
    //clear
    for i in 0..WIDTH*HEIGHT {
        v_playfield[i] = 0;
    }
    // box
    let middle = (HEIGHT/2)*(WIDTH/2)-WIDTH/2;
    v_playfield[middle] = 1;
    v_playfield[middle+WIDTH+1] = 1;
    v_playfield[middle+2*WIDTH] = 1;
    v_playfield[middle+2*WIDTH-1] = 1;
    v_playfield[middle+2*WIDTH+1] = 1;
}
*/

/*
fn init_playfield(v_playfield: &mut [u8]) {
    //clear
    for i in 0..WIDTH*HEIGHT {
        v_playfield[i] = 0;
    }
    // vertical 3
    let middle = (HEIGHT/2)*(WIDTH/2)-WIDTH/2;
    v_playfield[middle] = 1;

    
}
*/

fn no_of_surrounders(position: usize, v_playfield: &mut [u8]) -> usize {
    let mut count: usize = 0;

    let tlpos = 0;
    let trpos = WIDTH - 1;
    let llpos = ((HEIGHT - 1) * WIDTH) - 1;
    let lrpos = HEIGHT * WIDTH - 1;
    // Top Left Pos
    if position == tlpos {
        if v_playfield[position + 1] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH + 1] == 1 {
            count += 1
        };
        return count;
    }

    // First Line without TL pos and TR pos
    if position > tlpos && position < trpos {
        if v_playfield[position - 1] == 1 {
            count += 1
        };
        if v_playfield[position + 1] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH - 1] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH + 1] == 1 {
            count += 1
        };
        return count;
    }

    // Top Right Pos
    if position == trpos {
        if v_playfield[position - 1] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH - 1] == 1 {
            count += 1
        };
        return count;
    }

    // Lower Left Pos
    if position == llpos {
        if v_playfield[position + 1] == 1 {
            count += 1
        };
        if v_playfield[position - WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position - WIDTH + 1] == 1 {
            count += 1
        };
        return count;
    }

    // Lower line without Lower Left pos and Lower right pos
    if position > llpos && position < lrpos {
        if v_playfield[position - 1] == 1 {
            count += 1
        };
        if v_playfield[position + 1] == 1 {
            count += 1
        };
        if v_playfield[position - WIDTH - 1] == 1 {
            count += 1
        };
        if v_playfield[position - WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position - WIDTH + 1] == 1 {
            count += 1
        };
        return count;
    }

    // Lower Right Pos
    if position == lrpos {
        if v_playfield[position - 1] == 1 {
            count += 1
        };
        if v_playfield[position - WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position - WIDTH - 1] == 1 {
            count += 1
        };
        return count;
    }

    // left row
    if position % WIDTH == 0 {
        if v_playfield[position - WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position - WIDTH + 1] == 1 {
            count += 1
        };
        if v_playfield[position + 1] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH + 1] == 1 {
            count += 1
        };
        return count;
    }

    // right row
    if position % WIDTH == WIDTH - 1 {
        if v_playfield[position - WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position - WIDTH - 1] == 1 {
            count += 1
        };
        if v_playfield[position - 1] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH] == 1 {
            count += 1
        };
        if v_playfield[position + WIDTH - 1] == 1 {
            count += 1
        };
        return count;
    }

    // any other position
    if v_playfield[position - 1] == 1 {
        count += 1
    };
    if v_playfield[position + 1] == 1 {
        count += 1
    };
    if v_playfield[position - WIDTH] == 1 {
        count += 1
    };
    if v_playfield[position + WIDTH] == 1 {
        count += 1
    };
    if v_playfield[position + WIDTH + 1] == 1 {
        count += 1
    };
    if v_playfield[position + WIDTH - 1] == 1 {
        count += 1
    };
    if v_playfield[position - WIDTH + 1] == 1 {
        count += 1
    };
    if v_playfield[position - WIDTH - 1] == 1 {
        count += 1
    };
    return count;
}

fn play_one_round(v_playfield: &mut [u8]) {
    let mut cloned_payfiled = [0u8; WIDTH * HEIGHT];
    let mut number_surrounder: usize = 0;
    cloned_payfiled[..].clone_from_slice(v_playfield);
   
    for (_i, elem) in v_playfield.iter_mut().enumerate() {
        number_surrounder = no_of_surrounders(_i, &mut cloned_payfiled);
        
        match number_surrounder {
            2 => if *elem == 0 { *elem = 0 }, // Zelle behält ihren Status
            3 => *elem = 1, // Zelle wird lebendig oder bleibt lebendig
            _ => *elem = 0, // Zelle stirbt (bei < 2 oder > 3 Nachbarn)
        }
    }
}

fn main() -> Result<(), String> {
    let mut round_counter: u32 = 0;
    let mut point_in_playfiled: i32 = 0;
    let mut v_playfiled: [u8; WIDTH * HEIGHT] = [0 as u8; WIDTH * HEIGHT];
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

  
    let mut event_pump = sdl_context.event_pump()?;

    init_playfield(&mut v_playfiled);

    'running: loop {
        round_counter += 1;
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
   
/*
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                point_in_playfiled = v_playfiled[x * y] as i32;
                if point_in_playfiled == 1 {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                    println!("found 1 {} {}",x,y);
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                }

            }
        }
*/
        for i in 0 .. WIDTH*HEIGHT {
            point_in_playfiled = v_playfiled[i] as i32;
            let x = i % WIDTH;
            let y = i / WIDTH;
            if point_in_playfiled == 1 {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
           } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
            }

        }

        canvas.present();
        play_one_round(&mut v_playfiled);
        thread::sleep(time::Duration::from_millis(50));
        //::std::thread::sleep(time::Duration::from_millis(2));
     
    }

    Ok(())
}
