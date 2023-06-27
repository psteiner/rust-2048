use crate::prelude::*;

static SCREEN_WIDTH: i32 = 800;
static SCREEN_HEIGHT: i32 = 600;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// colors
static BG_COLOR: Color = Color::RGB(0xee, 0xe4, 0xda);
static FG_COLOR: Color = Color::RGB(0x77, 0x6e, 0x65);
static CHAR_COLOR: Color = Color::RGB(0xee, 0x33, 0x66);
static CONTAINER_COLOR: Color = Color::RGBA(0x77, 0x6e, 0x65, 200);
static CELL_COLORS: &[Color] = &[
    Color::RGBA(0xee, 0xe4, 0xda, 120),
    Color::RGB(0xed, 0xe0, 0xc8),
    Color::RGB(0xf2, 0xb1, 0x79),
    Color::RGB(0xf5, 0x95, 0x64),
    Color::RGB(0xf6, 0x7c, 0x5f),
    Color::RGB(0xf6, 0x5e, 0x3b),
    Color::RGB(0xed, 0xcf, 0x72),
    Color::RGB(0xed, 0xcc, 0x61),
    Color::RGB(0xed, 0xc8, 0x50),
    Color::RGB(0xed, 0xc5, 0x3f),
    Color::RGB(0xed, 0xc2, 0x2e),
    Color::RGB(0x3c, 0x3a, 0x32),
];
static SUPER_CELL_COLOR: Color = Color::RGB(0xcc, 0x33, 0xff);

// Font
#[cfg(target_os = "macos")]
static UNDER_MACOSX: bool = true;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "freebsd"))]
static UNDER_MACOSX: bool = false;

// #[allow(unused_must_use)]
fn draw_game(
    gm: &mut GameManager,
    canvas: &mut WindowCanvas,
    font: &Font,
    (x, y, w, h): (u32, u32, u32, u32),
) -> Result<(), String> {
    assert_eq!(w, h);
    // BEST in 500x500
    let size = gm.size as u32;
    let container_padding: u32 = 50 / (size as u32 + 1);
    let cell_width = (w - container_padding * (size + 1)) / size as u32;
    assert!(cell_width > 40); // Min width
    // canvas.box_(
    //     x as i16,
    //     y as i16,
    //     (x + w) as i16,
    //     (y + h) as i16,
    //     CONTAINER_COLOR,
    // )?;
    gm.grid.each_cell(|j, i, tile_opt| {
        let i = i as u32;
        let j = j as u32;
        let val = match tile_opt {
            Some(ref tile) => tile.value,
            None => 0,
        };
        let c = if val == 0 {
            0 // or will be +Infinity
        } else {
            (val as f64).log2() as usize
        };
        let color =
            CELL_COLORS.get(c).map(|&co| co).unwrap_or(SUPER_CELL_COLOR);
        let bx = (x + container_padding * (j + 1) + cell_width * j) as i16;
        let by = (y + container_padding * (i + 1) + cell_width * i) as i16;
        canvas.box_(
            bx,
            by,
            bx + cell_width as i16,
            by + cell_width as i16,
            color,
        );
        // canvas.string(bx, by, format!("({}, {})", j, i), CHAR_COLOR); // DEBUG
        if val != 0 {
            let (tex, tw, th) = {
                let wd = format!("{}", val);
                let (w, h) = font.size(&wd[..]).ok().expect("size of str");
                let text = font
                    .canvasder(
                        &wd[..],
                        sdl2::ttf::canvasderMode::Blended(FG_COLOR),
                    )
                    .ok()
                    .expect("canvasdered surface");
                (
                    canvas
                        .create_texture_from_surface(&text)
                        .ok()
                        .expect("create texture"),
                    w,
                    h,
                )
            };

            let ratio = if tw > cell_width {
                cell_width as f64 / tw as f64
            } else if th > cell_width {
                cell_width as f64 / th as f64
            } else {
                1.0
            };

            let tw = (tw as f64 * ratio) as u32;
            let th = (th as f64 * ratio) as u32;

            canvas.copy(
                &tex,
                None,
                Rect::new(
                    bx as i32 + cell_width as i32 / 2 - tw as i32 / 2,
                    by as i32 + cell_width as i32 / 2 - th as i32 / 2,
                    tw,
                    th,
                )
                .ok()
                .expect("a rect"),
            );
        }
    });
    Ok(())
}

fn draw_title(
    canvas: &mut canvasder::canvasderer,
    font: &sdl2_ttf::Font,
) -> SdlResult<()> {
    let (tex2, w, h) = {
        let wd = "Rust - 2048";
        let (w, h) = font.size(&wd[..])?;
        let text = font.canvasder(
            wd,
            sdl2_ttf::canvasderMode::Blended {
                foreground: FG_COLOR,
            },
        )?;
        (canvas.create_texture_from_surface(&text).unwrap(), w, h)
    };
    canvas.copy(
        &tex2,
        None,
        Rect::new(SCREEN_WIDTH / 2 - w as i32 / 2, 20, w, h)
            .ok()
            .expect("a rect"),
    );
    Ok(())
}

// FIXME: tooooooo many type conversion
fn draw_popup(
    canvas: &mut canvasder::canvasderer,
    font: &sdl2_ttf::Font,
    msg: &str,
) -> SdlResult<()> {
    let (tex, w, h) = {
        let (w, h) = font.size(&msg[..])?;
        let text = font.canvasder(
            msg,
            sdl2_ttf::canvasderMode::Blended {
                foreground: FG_COLOR,
            },
        )?;
        (canvas.create_texture_from_surface(&text)?, w, h)
    };
    canvas
        .rounded_box(
            (SCREEN_WIDTH / 2 - w as i32 / 2) as i16,
            (SCREEN_HEIGHT / 2 - h as i32 / 2) as i16,
            (SCREEN_WIDTH / 2 + w as i32 / 2) as i16,
            (SCREEN_HEIGHT / 2 + h as i32 / 2) as i16,
            5,
            BG_COLOR,
        )
        .unwrap();
    canvas
        .rounded_rectangle(
            (SCREEN_WIDTH / 2 - w as i32 / 2) as i16,
            (SCREEN_HEIGHT / 2 - h as i32 / 2) as i16,
            (SCREEN_WIDTH / 2 + w as i32 / 2) as i16,
            (SCREEN_HEIGHT / 2 + h as i32 / 2) as i16,
            5,
            FG_COLOR,
        )
        .unwrap();
    canvas.copy(
        &tex,
        None,
        Rect::new(
            SCREEN_WIDTH / 2 - w as i32 / 2,
            SCREEN_HEIGHT / 2 - h as i32 / 2,
            w,
            h,
        )
        .ok()
        .expect("a rect"),
    );
    Ok(())
}

// #[allow(non_shorthand_field_patterns)]
pub fn run(game_size: usize) -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let win = video_subsys
        .window("Rust - 2048", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = win.into_canvas().accelerated().build().unwrap();
    let mut fpsm = sdl2::gfx::framerate::FPSManager::new();
    fpsm.set_framerate(50)?;

    let font_path = Path::new(if UNDER_MACOSX == true {
        "/System/Library/Fonts/HelveticaNeueDeskInterface.ttc"
    } else {
        "res/OpenDyslexic-Regular.ttf"
    });

    let mut font = ttf_context.load_font(font_path, 128)?;

    let mut gm = GameManager::new(game_size);

    let mut playing = false;
    let mut celebrating = false;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        'event: for event in event_pump.poll_iter() {
            fpsm.delay();
            canvas.set_draw_color(BG_COLOR);
            canvas.clear();
            // == main drawing ==
            draw_title(&mut canvas, &font).unwrap();
            canvas.string(
                0i16,
                0i16,
                format!("frames: {}", fpsm.get_frame_count()).as_ref(),
                CHAR_COLOR,
            )?;

            canvas.string(
                200,
                90,
                format!("your score: {}", gm.score).as_ref(),
                CHAR_COLOR,
            )?;

            draw_game(
                &mut gm,
                &mut canvas,
                &font,
                ((SCREEN_WIDTH / 2 - 400 / 2) as u32, 100, 400, 400),
            )?;

            if celebrating || (playing && !gm.moves_available()) {
                // can't move
                draw_popup(
                    &mut canvas,
                    &font,
                    format!("Score: {}! Max Cell: {}", gm.score, "NaN")
                        .as_ref(),
                )?;
                playing = false;
                celebrating = true;
            } else if !playing && !celebrating {
                draw_popup(&mut canvas, &font, "Press SPACE to start!")
                    .unwrap();
            }

            // == main drawing ends ==
            canvas.present();
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } if playing => {
                    gm.move_to(Direction::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } if playing => {
                    gm.move_to(Direction::Right);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } if playing => {
                    gm.move_to(Direction::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } if playing => {
                    gm.move_to(Direction::Down);
                }
                Event::KeyDown { keycode: key, .. } => {
                    if key == Some(Keycode::Escape) {
                        break 'main;
                    } else if key == Some(Keycode::Space) && !playing {
                        playing = true;
                        celebrating = false;
                        gm.setup();
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    println!("mouse btn down at ({},{})", x, y);
                }

                _ => {}
            }
        }
    }
    Ok(())
}
