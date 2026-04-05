/*
By: Andrew Campbell
Date: 2026-03-25
Program Details: Displays a Database of movies
*/
//TO DO
//1. search button function 
//2. instructions/title page

mod modules;

use std::vec;

use crate::modules::database::{DatabaseClient, DatabaseTable, create_database_client};
use crate::modules::label::Label;
use crate::modules::listview::ListView;
use crate::modules::preload_image::LoadingScreenOptions;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use macroquad::{prelude::*, text};
use crate::modules::messagebox::MessageBox;

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
    //DATABASEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
    let mut client = create_database_client();
    //MESSAGE BOXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
    let mut info_instructionsbox = MessageBox::info("Instructions", "Welcome!, To add a movie, fill in fields then click 'Add'. To update a movie, select it, edit fields and click 'Update'. To remove a movie, select it and click 'Remove'. Use the search box to find movies by title. Click 'Clear' to reset the input fields. Enjoy!");
    let mut info_updatebox = MessageBox::info("Updated!", "Movie updated successfully!");
    let mut info_addbox = MessageBox::info("Added!", "Movie added successfully!");
    let mut info_removebox = MessageBox::info("Removed!", "Movie removed successfully!");
    let mut search_message = String::from("No matching movie found."); //for changing message box in search box
    let mut info_searchbox = MessageBox::info("Search Result", &search_message);
    //LISTVIEWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
    let items: Vec<String> = vec![];
    let mut lst_movies = ListView::new(&items, 555.0, 175.0, 30);
    lst_movies.with_colors(YELLOW, Some(BLACK), Some(LIGHTGRAY));
    lst_movies.set_width(450.0);
    lst_movies.with_max_visible_items(11);
    //VARIABLESSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS(SSS
    const VIRTUAL_WIDTH: f32 = 1024.0;
    const VIRTUAL_HEIGHT: f32 = 768.0;
    //PRELOADEEDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD
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
        VIRTUAL_HEIGHT, // height
        0.0,            // x position
        0.0,            // y position
        true,           // Enable stretching
        1.0,            // Normal zoom (100%)
    )
    .await;
    bg.set_preload(tm.get_preload("assets/movie_db_bg.png").unwrap());
    //LABELSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut lbl_title = Label::new("Blackhole: Movie Database", 0.0, 60.0, 60);
    lbl_title.with_colors(YELLOW, Some(BLACK));
    lbl_title.with_fixed_size(VIRTUAL_WIDTH + 20.0, 100.0);
    lbl_title.with_alignment(modules::label::TextAlign::Center);
    let mut lbl_instructions = Label::new("Click I for instructions again!", 300.0, 120.0, 30);
    lbl_instructions.with_colors(RED, Some(BLACK));
    lbl_instructions.with_alignment(modules::label::TextAlign::Center);
    //BUTTONSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut btn_clear = TextButton::new(200.0, 550.0, 150.0, 50.0, "Clear", BLACK, RED, 30);
    btn_clear.with_text_color(YELLOW);
    btn_clear.with_round(10.0);
    let mut btn_add = TextButton::new(50.0, 650.0, 150.0, 100.0, "Add", BLACK, RED, 30);
    btn_add.with_text_color(YELLOW);
    btn_add.with_round(10.0);
    let mut btn_remove = TextButton::new(250.0, 650.0, 150.0, 100.0, "Remove", BLACK, RED, 30);
    btn_remove.with_text_color(YELLOW);
    btn_remove.with_round(10.0);
    let mut btn_update = TextButton::new(450.0, 650.0, 150.0, 100.0, "Update", BLACK, RED, 30);
    btn_update.with_text_color(YELLOW);
    btn_update.with_round(10.0);
    let mut btn_search = TextButton::new(650.0, 650.0, 150.0, 100.0, "Search", BLACK, RED, 30);
    btn_search.with_text_color(YELLOW);
    btn_search.with_round(10.0);
    let mut btn_exit = TextButton::new(850.0, 650.0, 150.0, 100.0, "Exit", BLACK, RED, 30);
    btn_exit.with_text_color(YELLOW);
    btn_exit.with_round(10.0);
    //TEXT INPUTSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let mut txt_searchname = TextInput::new(650.0, 600.0, 150.0, 50.0, 30.0);
    txt_searchname.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_searchname.set_prompt("Search Name");
    txt_searchname.set_prompt_color(PURPLE);
    let mut txt_movietitle = TextInput::new(50.0, 150.0, 200.0, 50.0, 30.0);
    txt_movietitle.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_movietitle.set_prompt("Movie Title");
    txt_movietitle.set_prompt_color(PURPLE);
    txt_movietitle.set_max_chars(14);
    let mut txt_mainactors = TextInput::new(300.0, 150.0, 200.0, 50.0, 30.0);
    txt_mainactors.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_mainactors.set_prompt("Main Actors");
    txt_mainactors.set_prompt_color(PURPLE);
    txt_mainactors.set_max_chars(14);
    let mut txt_releasedate = TextInput::new(50.0, 250.0, 200.0, 50.0, 30.0);
    txt_releasedate.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_releasedate.set_prompt("Release Date");
    txt_releasedate.set_prompt_color(PURPLE);
    txt_releasedate.set_max_chars(14);
    let mut lbl_movieid = Label::new("Movie ID: ", 305.0, 280.0, 30);
    lbl_movieid.with_fixed_size(200.0, 50.0);
    lbl_movieid.with_colors(YELLOW, Some(BLACK));
    let mut txt_summary = TextInput::new(50.0, 350.0, 450.0, 200.0, 30.0);
    txt_summary.with_colors(YELLOW, WHITE, BLACK, RED);
    txt_summary.set_prompt("Summary");
    txt_summary.set_prompt_color(PURPLE);
    txt_summary.set_multiline(true);
    txt_summary.set_max_chars(198);
    let mut text_inputs = vec![&mut txt_movietitle, &mut txt_mainactors, &mut txt_releasedate, &mut txt_summary, &mut txt_searchname,];
    client = update_listview(&mut lst_movies, create_database_client()).await;
    info_instructionsbox.show();
    loop {
        if !info_searchbox.is_visible() {
            info_searchbox.set_message("No matching movie found.");
        }
        if is_key_pressed(KeyCode::I) {
            info_instructionsbox.show();
        }
        //BG AND VARSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
        use_virtual_resolution(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
        bg.draw();
        //BUTTON CLEARRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR
        if btn_clear.click() {
            client = update_listview(&mut lst_movies, client).await;
            for input in &mut text_inputs {
                input.set_text("");
            }
            lbl_movieid.set_text("Movie ID: ");
        }
        //ADDINGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG
        if btn_add.click() {
            let title = text_inputs[0].get_text();
            let actors = text_inputs[1].get_text();
            let release_date = text_inputs[2].get_text();
            let summary = text_inputs[3].get_text();
            let new_record = DatabaseTable { id: 0, title: title, actor: actors, released: release_date, summary: summary};
            if let Ok(id) = client.insert_record("movie_table", &new_record).await {
                client = update_listview(&mut lst_movies, client).await;
                info_addbox.show();
            } else {
                println!("Error inserting record");
            }
        }
        //REMOVEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEeee
        if btn_remove.click() {
            let id = lbl_movieid.get_text().split_at(10).1.parse::<i64>().unwrap_or(0);
            if let Ok(deleted_count) = client.delete_record_by_id("movie_table", id).await {
                for text_input in &mut text_inputs {
                    text_input.set_text("");
                }
                lbl_movieid.set_text("Movie ID: ");
                client = update_listview(&mut lst_movies, client).await;
                info_removebox.show();
            } else {
                println!("Error deleting record");
            }
        }
        if btn_update.click() {
            //only run if fields are full for no error
            if text_inputs[0].get_text() != "" && text_inputs[1].get_text() != "" && text_inputs[2].get_text() != "" && text_inputs[3].get_text() != "" {
                let title = text_inputs[0].get_text();
                let actors = text_inputs[1].get_text();
                let release_date = text_inputs[2].get_text();
                let summary = text_inputs[3].get_text();
                let id = lbl_movieid.get_text().split_at(10).1.parse::<i64>().unwrap_or(0);
                if let Ok(updated_count) = client.update_record_by_id("movie_table", id, "title", &title).await {
                // updated_count is the number of records updated
                } else {
                    println!("Error updating record");
                }
                if let Ok(updated_count) = client.update_record_by_id("movie_table", id, "actor", &actors).await {
                // updated_count is the number of records updated
                } else {
                    println!("Error updating record");
                }
                if let Ok(updated_count) = client.update_record_by_id("movie_table", id, "released", &release_date).await {
                // updated_count is the number of records updated
                } else {
                    println!("Error updating record");
                }
                if let Ok(updated_count) = client.update_record_by_id("movie_table", id, "summary", &summary).await {
                // updated_count is the number of records updated
                } else {
                    println!("Error updating record");
                }
                info_updatebox.show();
                client = update_listview(&mut lst_movies, client).await;
            }
        }
        if btn_search.click() {
            text_inputs[0].set_text("");
            text_inputs[1].set_text("");
            text_inputs[2].set_text("");
            text_inputs[3].set_text("");
            lbl_movieid.set_text("Movie ID: ");
            if text_inputs[4].get_text() != "" {
                let search_title = text_inputs[4].get_text();
                if let Ok(records) = client.fetch_table("movie_table").await {
                    for record in &records {
                        if record.title.to_lowercase().contains(&search_title.to_lowercase()) {
                            search_message = format!("Found '{}' in the database.", record.title);
                            info_searchbox.set_message(search_message);
                            text_inputs[0].set_text(&record.title);
                            text_inputs[1].set_text(&record.actor);
                            text_inputs[2].set_text(&record.released);
                            text_inputs[3].set_text(&record.summary);
                            lbl_movieid.set_text(&format!("Movie ID: {}", record.id));
                            break; // Stop after finding the first match
                        }
                    }
                } else {
                    println!("Error fetching records from database");
                }
                info_searchbox.show();
            } else {
                info_searchbox.set_message("No matching movie found.");
            }
        }
        if btn_exit.click() {
            break;
        }
        //LISTVIEW SELECTEDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD
        if lst_movies.selected_item().is_some() && text_inputs[0].get_text() != *lst_movies.selected_item().unwrap() {
            let mut records: Vec<DatabaseTable> = Vec::new();
            if let Ok(result) = client.fetch_table("movie_table").await {
                records = result;
                for record in &records {
                    if record.title == *lst_movies.selected_item().unwrap() {
                        text_inputs[0].set_text(&record.title);
                        text_inputs[1].set_text(&record.actor);
                        text_inputs[2].set_text(&record.released);
                        text_inputs[3].set_text(&record.summary);
                        lbl_movieid.set_text(&format!("Movie ID: {}", record.id));
                    }
                }
            } else {
                println!("Error fetching records from database");
            }
            client = update_listview(&mut lst_movies, client).await; //remove selection so editing can occur
        }
        //DRAWSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
        lbl_title.draw();
        lbl_movieid.draw();
        lbl_instructions.draw();
        for input in &mut text_inputs {
                input.draw();
        }
        lst_movies.draw();
        info_instructionsbox.draw();
        info_instructionsbox.centered();
        info_updatebox.draw();
        info_updatebox.centered();
        info_addbox.draw();
        info_addbox.centered();
        info_searchbox.draw();
        info_searchbox.centered();
        info_removebox.draw();
        info_removebox.centered();
        next_frame().await;
    }
}


async fn update_listview(list_view: &mut ListView, client: DatabaseClient) -> DatabaseClient {
    list_view.clear();
    let mut records: Vec<DatabaseTable> = Vec::new();
    let mut titles: Vec<String> = Vec::new();
    let matt = client.fetch_table("movie_table").await;
    if let Ok(result) = matt {
        records = result;
        for record in &records {
            titles.push(record.title.clone());
        }
    } else {
       println!("Error fetching records from database {} ",matt.err().unwrap());
    }
    list_view.add_items(&titles);
    client
}