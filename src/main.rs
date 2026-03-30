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
use crate::modules::listview::ListView;
use crate::modules::database::{create_database_client, create_table_from_struct, DatabaseTable};

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
    //LISTVIEWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6", "Item 7", "Item 8", "Item 9", "Item 10", "Item 11", "Item 12", "Item 13"];
    let mut lst_movies = ListView::new(&items, 555.0, 175.0, 30);
    lst_movies.with_colors(YELLOW, Some(BLACK), Some(LIGHTGRAY));
    lst_movies.set_width(450.0);
    lst_movies.with_max_visible_items(11);
    //DATABASEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
    let _ = create_table_from_struct("Movie Database Table").await; //use _ cause its never used again
    let client = create_database_client();
    let new_record1 = DatabaseTable { id: 0, title: "Avatar".to_string(), actor: "Sam Worthington, Zoe Saldana, Sigourney Weaver".to_string(), released: "December 18th 2009".to_string(), summary: "On the lush alien world of Pandora live the Na'vi, beings who appear primitive but are highly evolved. Because the planet's environment is poisonous, human/Na'vi hybrids, called Avatars, must link to human minds to allow for free movement on Pandora. Jake Sully (Sam Worthington), a paralyzed former Marine, becomes mobile again through one such Avatar and falls in love with a Na'vi woman (Zoe Saldana). As a bond with her grows, he is drawn into a battle for the survival of her world.".to_string()};
    let new_record2 = DatabaseTable { id: 0, title: "Avengers Endgame".to_string(), actor: "Robert Downey Jr., Chris Evans, Josh Brolin".to_string(), released: "April 26th 2019".to_string(), summary: "After half of all life is snapped away by Thanos, the Avengers are left scattered and divided. Now with a way to reverse the damage, the Avengers and their allies must assemble once more and learn to put differences aside in order to work together and set things right. Along the way, the Avengers realize that sacrifices must be made as they prepare for the ultimate final showdown with Thanos, which will result in the heroes fighting the biggest battle they have ever faced.".to_string()};
    let new_record3 = DatabaseTable { id: 0, title: "Spider-Man Homecoming".to_string(), actor: "Tom Holland, Zendaya, Robert Downey Jr.".to_string(), released: "July 5th 2017".to_string(), summary: "Thrilled by his experience with the Avengers, young Peter Parker returns home to live with his Aunt May. Under the watchful eye of mentor Tony Stark, Parker starts to embrace his newfound identity as Spider-Man. He also tries to return to his normal daily routine -- distracted by thoughts of proving himself to be more than just a friendly neighborhood superhero. Peter must soon put his powers to the test when the evil Vulture emerges to threaten everything that he holds dear.".to_string()};
    let _ = client.insert_record("Movie Database Table", &new_record1).await;
    let _ = client.insert_record("Movie Database Table", &new_record2).await;
    let _ = client.insert_record("Movie Database Table", &new_record3).await;
    let mut records: Vec<DatabaseTable> = Vec::new();
    if let Ok(result) = client.fetch_table("Movie Database Table").await {
        records = result;
        lst_movies.clear();
        //update_listview(&mut lst_movies, &records);
    } else {
        // Handle error
    }
    println!("Fetched Records:");
    for record in records {
        println!("ID: {}, Title: {}, Actors: {}, Released: {}, Summary: {}", record.id, record.title, record.actor, record.released, record.summary);
    }
    //VARIABLESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS(SSS
    const VIRTUAL_WIDTH: f32 = 1024.0;
    const VIRTUAL_HEIGHT: f32 = 768.0;
    use_virtual_resolution(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
    //PRELOADEEDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD
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
        txt_movietitle.draw();
        txt_releasedate.draw();
        txt_movieid.draw();
        txt_mainactors.draw();
        txt_summary.draw();
        txt_removeid.draw();
        txt_searchname.draw();
        lst_movies.draw();
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
        next_frame().await;
    }
}
