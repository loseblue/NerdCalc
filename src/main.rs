use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
    NerdCalc::run(Settings{
        window:iced::window::Settings{
            size: (500, 300),
            min_size: Some((500, 300)),
            max_size: Some((500, 300)),
            always_on_top: true,
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}


struct NerdCalc;

#[warn(non_snake_case)]
impl Application for NerdCalc {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (NerdCalc, Command<Self::Message>) {
        (NerdCalc, Command::none())
    }

    fn title(&self) -> String {
        String::from("NerdCalc")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "NerdCalc, world!".into()
    }
}
