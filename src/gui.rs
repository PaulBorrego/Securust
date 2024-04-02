use iced::{widget, Alignment, Element, Font, Pixels, Theme,};
use iced::widget::{button, column, text,text_input,Space};
use iced::{Application, Command, Settings, executor, window};
use std::fs::{File, OpenOptions};
use std::io::Write;
use orion::aead;


fn write_to_file(mut u: Vec<u8>,mut p: Vec<u8>) ->  Result<File, std::io::Error> {
    let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open("info/info.txt")
            .unwrap();

    let s = aead::SecretKey::default();
    u.resize(32,32); //make the username and password take up 32 bytes
    p.resize(32, 32);
    
    let mut temp = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("info/temp.txt")
        .unwrap();

    file.write_all(&u)?;
    file.write_all(&p)?;
    file.write_all(s.unprotected_as_bytes())?;
    write!(file,"\n")?;

    temp.write_all(&u)?;
    temp.write_all(&p)?;
    temp.write_all(s.unprotected_as_bytes())?;
    write!(temp,"\n")?;

    Ok(file)
}


pub fn main() -> iced::Result {
    let ferry = Some(window::icon::from_file("img/ferry.png").unwrap());
    let settings = Settings {
        window: window::Settings {
            size: iced::Size { width: 600.0f32, height: 260.0f32 },
            resizable: true,
            decorations: true,
            level: window::Level::AlwaysOnTop,
            position: window::Position::Centered,
            icon: ferry,  
            ..Default::default()
        },
        ..Default::default()
    };
    TextBox::run(settings)
} 

#[derive(Debug,Clone)]
pub enum Message {
    UserName(String),
    Password(String),
    ENTER,
}
#[derive(Clone,Debug)]
pub struct TextBox {
    user: String,
    pass: String,
    error: String,
}

impl Application for TextBox {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;


    fn new(_flags: ()) -> (TextBox, Command<Self::Message>) {
        (TextBox {
            user: String::new(),
            pass: String::new(),
            error: String::new(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Login")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UserName(a) => self.user = a,
            Message::Password(a) => self.pass = a,
            Message::ENTER => {
                if !self.pass.is_empty() {
                    write_to_file(self.user.clone().into_bytes(), self.pass.clone().into_bytes()).expect("File Failure");
                    return window::close(window::Id::MAIN);
                }
                else {
                    self.error = String::from("No Password Given");
                }
            },
        }
        Command::none()
    }

    fn theme(&self) -> Theme {
        widget::theme::Theme::Dark
    }


    fn view(&self) -> Element<'_, Self::Message> {
        let user = text_input("Empty", &self.user,)
        .on_input(Message::UserName)
        .padding(10)
        .size(20);
    
        let pass = text_input("", &self.pass,)
        .on_input(Message::Password)
        .padding(10)
        .size(20)
        .secure(true)
        .icon(text_input::Icon { 
            font: Font::default(), 
            code_point: '🔒', 
            size: Some(Pixels(28.0)), 
            spacing: 10.0, side: 
            text_input::Side::Right,
        });
        
        let a = column![
            text("Username").size(18),
            user,
            text("Password").size(18),
            pass,
            Space::new(0, 10),
            button("Confirm").on_press(Message::ENTER),
            Space::new(0,10),
            text(format!("{}",self.error)).size(18),
            // text(format!("Password: {} ",self.final_password)).size(18),
            // text(format!("Username: {} ",self.final_username)).size(18),

        ]
        .padding(10)
        .align_items(Alignment::Start);
    
        a.into()
    }    
    
}