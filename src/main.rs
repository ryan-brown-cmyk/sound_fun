use iced::widget::{button, column, row, text, grid, pick_list};  
use iced::{Element, Length, Theme, Task};

use crate::ui::building_blocks; 
mod ui;
mod audio;

// Starting from the bare minimum working:
// The ribbon as a row, and the Grid for all the different containers that I want to work with

#[derive(Debug, Clone)]
enum Menus {
    Main,
    Secondary 
} // These are working titles, the main screen is for the main application, and the secondary screen is currently for settings, but may change.


#[derive(Debug, Clone)]
struct State {
    current_state: Menus,
    current_theme: Theme,
    input_value: String,
    audio_track: Vec<f32>
}

#[derive(Debug, Clone)]  
enum Message {  
    SecondaryNav,  
    MainNav,  
    ThemeChanged(Theme),
    InputChanged(String),
    InputSubmitted,
    AudioLoad(Vec<f32>),
}  

impl Default for State {  
    fn default() -> Self {  
        State {  
            current_state: Menus::Main,  
            current_theme: Theme::TokyoNight,
            input_value: "".to_string(),
            audio_track: Vec::new(),
        }  
    }  
}  


fn update(state: &mut State, message: Message) -> Task<Message> {  // This is with a "detached" update method, which seems to be the popular method.
    match message {  
        Message::SecondaryNav => {  
            state.current_state = Menus::Secondary;  
            Task::none()
        }  
        Message::MainNav => {  
            state.current_state = Menus::Main;  
            Task::none()
        }  
        Message::ThemeChanged(theme) => {
             state.current_theme = theme;
             Task::none()
        } 
        Message::InputChanged(file_path) => {
            state.input_value = file_path;
            Task::none()

        }
        Message::InputSubmitted => {

            Task::perform(
                async move 
                { 
                    audio::file_management::open_audio("examples/test.wav".to_string(),  ".wav").await.unwrap().expect("ruh roh") 
                },
                |result| match result {
                    audio_content => Message::AudioLoad(audio_content)
                }

                
            )
        }
        Message::AudioLoad(audio_samples) => {
            state.audio_track = audio_samples;
            Task::none()
        }
        

    }  
}  

fn view<'a>(state: &'a State) -> Element<'a, Message> {  
    match &state.current_state {  
        Menus::Main => main_view(&state),  
        Menus::Secondary => simple_screen_view(&state),  
    }  
}  

fn theme(state: &State) -> Theme {  
    state.current_theme.clone()  
} 

fn main_view<'a>(state: &State) -> Element<'a, Message> {  
    column![  
        // Header Ribbon 
        row![  
            button("Secondary Nav").on_press(Message::SecondaryNav)  
        ]  
        .spacing(10)  
        .padding(10)  
        .height(Length::Shrink),  
          
        // Main Grid that contains the various applications / usages.
        grid![  
             
            building_blocks::file_input_container(  
                "Input Container",  
                "Type something...",  
                &state.input_value,  
                Message::InputChanged,
                Message::InputSubmitted
            ),  
            building_blocks::custom_container(  
                column![  
                    text("This container is empty, but could contain other things."),  
                    text("It acts as a normal container, just formatted uninously via the custom command"),  
                ],  
                "Test Container"  
            ),  
        ]  
        .columns(2)  
        .spacing(10)  
        .height(Length::Fill),  
    ]  
    .into()  
}

fn simple_screen_view<'a>(state: &'a State) -> Element<'a, Message> {  
    column![  
        // Header with back button  
        row![  
            button("Back").on_press(Message::MainNav),  
            text("Simple Screen").size(24),  
        ]  
        .spacing(10)  
        .padding(10)  
        .height(Length::Shrink),  
          
        // Setting Collumn (should have more, mostly just functional.) 
        column![  
        text!("Theme Selector").size(24),  
        pick_list(  
            &Theme::ALL[..],  
            Some(state.current_theme.clone()),  
            Message::ThemeChanged,  
        ),  
        text("Current theme:").size(16),  
        text(format!("{:?}", state.current_theme)),  
    ]  
    .spacing(20)  
    .padding(20)  
    
    ]  
    .into()  
}  





pub fn main() -> iced::Result {
    iced::application(State::default, update, view).theme(theme).run()
}



// *******
/*
So we have a minimum working GUI, with framework for the settings page (whatever that looks like), the main page, and some text inputs.

From now we just have to go through and actually like, make everything, but it gives us stuff to plug in to once we do make it, which is nice. 



*/
// ***** 
