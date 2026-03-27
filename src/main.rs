/*
By: Andrew Campbell
Date: 2026-03-25
Program Details: Displays a Database of movies
*/

mod modules;

use macroquad::prelude::*;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use crate::modules::grid::draw_grid;

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
    let all_assets = vec!["assets/movie_db_bg.png"];
      let tm = TextureManager::new();
    // Using custom loading screen appearance
    let loading_options = LoadingScreenOptions {
       title: Some("Movie Database".to_string()),
       background_color: BLACK,
       bar_fill_color: RED,
       // Use default values for other options
       ..Default::default()
    };
    tm.preload_with_loading_screen(&all_assets, Some(loading_options)).await;
    //IMAGESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut bg = StillImage::new(
        "",
        VIRTUAL_WIDTH,  // width
        VIRTUAL_HEIGHT,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    bg.set_preload(tm.get_preload("assets/movie_db_bg.png").unwrap());
    //LABELSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut lbl_title = Label::new("Blackhole: Movie Database", 0.0, 60.0, 60);
    lbl_title.with_colors(YELLOW, Some(BLACK));
    lbl_title.with_fixed_size(VIRTUAL_WIDTH+20.0, 100.0);
    lbl_title.with_alignment(modules::label::TextAlign::Center);
    //BUTTONSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut btn_add = TextButton::new(
        50.0,
        650.0,
        150.0,
        100.0,
        "Add",
        BLACK,
        RED,
        30
    );
    btn_add.with_text_color(YELLOW); 
    btn_add.with_round(10.0);
    let mut btn_remove = TextButton::new(
        250.0,
        650.0,
        150.0,
        100.0,
        "Remove",
        BLACK,
        RED,
        30
    );
    btn_remove.with_text_color(YELLOW); 
    btn_remove.with_round(10.0);
    let mut btn_update = TextButton::new(
        450.0,
        650.0,
        150.0,
        100.0,
        "Update",
        BLACK,
        RED,
        30
    );
    btn_update.with_text_color(YELLOW); 
    btn_update.with_round(10.0);
    let mut btn_search = TextButton::new(
        650.0,
        650.0,
        150.0,
        100.0,
        "Search",
        BLACK,
        RED,
        30
    );
    btn_search.with_text_color(YELLOW); 
    btn_search.with_round(10.0);
    let mut btn_exit = TextButton::new(
        850.0,
        650.0,
        150.0,
        100.0,
        "Exit",
        BLACK,
        RED,
        30
    );
    btn_exit.with_text_color(YELLOW); 
    btn_exit.with_round(10.0);
    //TEXT INPUTSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut txt_removeid = TextInput::new(450.0, 600.0, 150.0, 50.0, 30.0);
    txt_removeid.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_removeid.set_prompt("Remove ID..");
    txt_removeid.set_prompt_color(PURPLE);
    let mut txt_searchname = TextInput::new(650.0, 600.0, 150.0, 50.0, 30.0);
    txt_searchname.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_searchname.set_prompt("Search Name");
    txt_searchname.set_prompt_color(PURPLE);
    let mut txt_movietitle = TextInput::new(50.0, 150.0, 200.0, 50.0, 30.0);
    txt_movietitle.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_movietitle.set_prompt("Movie Title");
    txt_movietitle.set_prompt_color(PURPLE);
    let mut txt_mainactors = TextInput::new(300.0, 150.0, 200.0, 50.0, 30.0);
    txt_mainactors.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_mainactors.set_prompt("Main Actors");
    txt_mainactors.set_prompt_color(PURPLE);
    let mut txt_releasedate = TextInput::new(50.0, 250.0, 200.0, 50.0, 30.0);
    txt_releasedate.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_releasedate.set_prompt("Release Date");
    txt_releasedate.set_prompt_color(PURPLE);
    let mut txt_movieid = TextInput::new(300.0, 250.0, 200.0, 50.0, 30.0);
    txt_movieid.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_movieid.set_prompt("Movie ID");
    txt_movieid.set_prompt_color(PURPLE);
    let mut txt_summary = TextInput::new(50.0, 350.0,450.0, 200.0, 30.0);
    txt_summary.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_summary.set_prompt("Summary");
    txt_summary.set_prompt_color(PURPLE);
    loop {
        bg.draw();
        lbl_title.draw();
        draw_grid(50.0, BROWN);
        txt_movietitle.draw();
        txt_releasedate.draw();
        txt_movieid.draw();
        txt_mainactors.draw();
        txt_summary.draw();
        if btn_add.click() {
            println!("Add Movie button clicked!");
        }
        if btn_remove.click() {
            println!("Remove Movie button clicked!");
        }
        if btn_update.click() {
            println!("Update Movie button clicked!");
        }
        if btn_search.click() {
            println!("Search Movies button clicked!");
        }
        if btn_exit.click() {
            break;
        }
        txt_removeid.draw();
        txt_searchname.draw();
        next_frame().await;
    }
}
