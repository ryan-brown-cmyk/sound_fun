use iced::widget::{container, text, text_input, column};  
use iced::{Element, Length};  

pub fn custom_container<'a, Message: 'a>(  
    content: impl Into<Element<'a, Message>>,  
    title: &'a str,  
) -> Element<'a, Message> {  


   
    container(  
        column![
            text(title).size(20),  
            container(content).padding(10),
        ]
    )  
    .padding(15)  
    .height(Length::Shrink)  
    .width(Length::Fill)  
    .style(container::rounded_box)  
    .into()  
}  


// This needs to be changed to accept the message type on call. I think, however, the general plan is good to start with.
// Look at the active call. Going to start looking into sound.
pub fn file_input_container<'a, Message: Clone + 'a>(  
    title: &'a str,  
    placeholder: &str,  
    value: &str,  
    on_input: impl Fn(String) -> Message + 'a,  
) -> Element<'a, Message> {  
    custom_container(  
        text_input(placeholder, value)  
            .on_input(on_input)  
            .width(Length::Fill),  
        title  
    )  
}
