use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::{thread, time};

const CELL_SIZE: u32 = 4;
const GRID_WIDTH: usize = 256;
const GRID_HEIGHT: usize = 192;
const WIN_WIDTH: u32 = GRID_WIDTH as u32 * CELL_SIZE;
const WIN_HEIGHT: u32 = GRID_HEIGHT as u32 * CELL_SIZE;

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
    for i in 0..GRID_WIDTH * GRID_HEIGHT {
        v_playfield[i] = 0;
    }
    // box
    let middle = (GRID_HEIGHT / 2) * GRID_WIDTH + (GRID_WIDTH / 2);
    v_playfield[middle] = 1;
    v_playfield[middle + GRID_WIDTH + 1] = 1;
    v_playfield[middle + 2 * GRID_WIDTH] = 1;
    v_playfield[middle + 2 * GRID_WIDTH - 1] = 1;
    v_playfield[middle + 2 * GRID_WIDTH + 1] = 1;
}
*/

/*
fn init_playfield(v_playfield: &mut [u8]) {
    //clear
    for i in 0..GRID_WIDTH * GRID_HEIGHT {
        v_playfield[i] = 0;
    }
    // vertical 3
    let middle = (GRID_HEIGHT / 2) * GRID_WIDTH + (GRID_WIDTH / 2);
    v_playfield[middle] = 1;

    
}
*/

fn no_of_surrounders(position: usize, v_playfield: &[u8]) -> usize {
    let x = position % GRID_WIDTH;
    let y = position / GRID_WIDTH;
    let mut count: usize = 0;

    for dy in [-1i32, 0, 1] {
        for dx in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx >= GRID_WIDTH || ny >= GRID_HEIGHT {
                continue;
            }
            if v_playfield[ny * GRID_WIDTH + nx] == 1 {
                count += 1;
            }
        }
    }

    count
}

fn play_one_round(v_playfield: &mut [u8]) {
    let mut cloned_playfield = [0u8; GRID_WIDTH * GRID_HEIGHT];
    cloned_playfield[..].clone_from_slice(v_playfield);
   
    for (_i, elem) in v_playfield.iter_mut().enumerate() {
        let number_surrounder = no_of_surrounders(_i, &cloned_playfield);
        
        match number_surrounder {
            2 => if *elem == 0 { *elem = 0 }, // Zelle behält ihren Status
            3 => *elem = 1, // Zelle wird lebendig oder bleibt lebendig
            _ => *elem = 0, // Zelle stirbt (bei < 2 oder > 3 Nachbarn)
        }
    }
}

fn main() -> Result<(), String> {
    let mut v_playfield: [u8; GRID_WIDTH * GRID_HEIGHT] = [0 as u8; GRID_WIDTH * GRID_HEIGHT];
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", WIN_WIDTH, WIN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA8888,
            GRID_WIDTH as u32,
            GRID_HEIGHT as u32,
        )
        .map_err(|e| e.to_string())?;
    let mut frame_buffer = vec![0u8; GRID_WIDTH * GRID_HEIGHT * 4];

    let mut event_pump = sdl_context.event_pump()?;

    init_playfield(&mut v_playfield);

    'running: loop {
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
                point_in_playfield = v_playfield[x + y * WIDTH] as i32;
                if point_in_playfield == 1 {
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
        for i in 0..(GRID_WIDTH * GRID_HEIGHT) {
            let alive = v_playfield[i] == 1;
            let offset = i * 4;
            if alive {
                frame_buffer[offset] = 255;
                frame_buffer[offset + 1] = 255;
                frame_buffer[offset + 2] = 255;
                frame_buffer[offset + 3] = 255;
            } else {
                frame_buffer[offset] = 0;
                frame_buffer[offset + 1] = 0;
                frame_buffer[offset + 2] = 0;
                frame_buffer[offset + 3] = 255;
            }
        }

        texture
            .update(None, &frame_buffer, GRID_WIDTH * 4)
            .map_err(|e| e.to_string())?;
        canvas.clear();
        let dst = Rect::new(0, 0, WIN_WIDTH, WIN_HEIGHT);
        canvas.copy(&texture, None, Some(dst))?;

        canvas.present();
        play_one_round(&mut v_playfield);
        thread::sleep(time::Duration::from_millis(50));
        //::std::thread::sleep(time::Duration::from_millis(2));
     
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{play_one_round, GRID_HEIGHT, GRID_WIDTH};

    fn idx(x: usize, y: usize) -> usize {
        y * GRID_WIDTH + x
    }

    fn alive_positions(grid: &[u8]) -> Vec<usize> {
        grid.iter()
            .enumerate()
            .filter_map(|(i, &v)| if v == 1 { Some(i) } else { None })
            .collect()
    }

    #[test]
    fn blinker_oscillates() {
        let mut grid = [0u8; GRID_WIDTH * GRID_HEIGHT];
        let cx = GRID_WIDTH / 2;
        let cy = GRID_HEIGHT / 2;

        grid[idx(cx - 1, cy)] = 1;
        grid[idx(cx, cy)] = 1;
        grid[idx(cx + 1, cy)] = 1;

        play_one_round(&mut grid);

        let expected = [
            idx(cx, cy - 1),
            idx(cx, cy),
            idx(cx, cy + 1),
        ];
        let alive = alive_positions(&grid);
        assert_eq!(alive.len(), expected.len());
        for pos in expected {
            assert!(grid[pos] == 1);
        }

        play_one_round(&mut grid);

        let expected = [
            idx(cx - 1, cy),
            idx(cx, cy),
            idx(cx + 1, cy),
        ];
        let alive = alive_positions(&grid);
        assert_eq!(alive.len(), expected.len());
        for pos in expected {
            assert!(grid[pos] == 1);
        }
    }

    #[test]
    fn glider_moves_diagonally() {
        let mut grid = [0u8; GRID_WIDTH * GRID_HEIGHT];
        let x0 = GRID_WIDTH / 2;
        let y0 = GRID_HEIGHT / 2;

        let start = [
            (x0 + 1, y0 + 0),
            (x0 + 2, y0 + 1),
            (x0 + 0, y0 + 2),
            (x0 + 1, y0 + 2),
            (x0 + 2, y0 + 2),
        ];
        for (x, y) in start {
            grid[idx(x, y)] = 1;
        }

        for _ in 0..4 {
            play_one_round(&mut grid);
        }

        let expected = [
            (x0 + 2, y0 + 1),
            (x0 + 3, y0 + 2),
            (x0 + 1, y0 + 3),
            (x0 + 2, y0 + 3),
            (x0 + 3, y0 + 3),
        ];

        let alive = alive_positions(&grid);
        assert_eq!(alive.len(), expected.len());
        for (x, y) in expected {
            assert!(grid[idx(x, y)] == 1);
        }
    }
}
