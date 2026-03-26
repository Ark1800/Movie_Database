/*
By: Andrew Campbell
Date: 2026-03-25
Program Details: Displays a Database of movies
*/

mod modules;

use macroquad::prelude::*;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Movie_Database".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    const VIRTUAL_WIDTH: f32 = 1024.0;
    const VIRTUAL_HEIGHT: f32 = 768.0;
    use_virtual_resolution(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
    //PRELOADEEDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD       
    let all_assets = vec![""]
      let tm = TextureManager::new();
    // Using custom loading screen appearance
    let loading_options = LoadingScreenOptions {
       title: Some("Space Invaders".to_string()),
       background_color: BLACK,
       bar_fill_color: RED,
       // Use default values for other options
       ..Default::default()
    };
    tm.preload_with_loading_screen(&all_assets, Some(loading_options)).await;
    loop {
        let img = StillImage::new(
        "assets/image_name.png",
        100.0,  // width
        200.0,  // height
        200.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
        next_frame().await;
    }
}
