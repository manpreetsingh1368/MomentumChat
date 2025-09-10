use iced::{
    executor, Application, Command, Length, Subscription, theme, Alignment,
};
use iced::widget::{
    button, Button, column, Column, container, Container, text, Text, text_input, TextInput,
};
use iced::time;
use std::time::Duration;

pub fn main() -> iced::Result {
    AuthApp::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    ToggleMode,
    Tick,
    UsernameChanged(String),
    PasswordChanged(String),
    UsernameFocused(bool),
    PasswordFocused(bool),
    LoginPressed,
    SignupPressed,
}

struct FloatingInput {
    value: String,
    focused: bool,
    state: text_input::State,
    label: String,
}

impl FloatingInput {
    fn new(label: &str) -> Self {
        FloatingInput {
            value: String::new(),
            focused: false,
            state: text_input::State::new(),
            label: label.to_string(),
        }
    }

    fn view(&mut self, on_change: impl Fn(String) -> Message + 'static, on_focus: impl Fn(bool) -> Message + 'static) -> iced::Element<Message> {
        let label_color = if self.focused {
            iced::Color::from_rgb(0.2, 0.6, 1.0)
        } else {
            iced::Color::from_rgb(0.5, 0.5, 0.5)
        };

        column![
            text(&self.label).size(16).color(label_color),
            TextInput::new(&mut self.state, "", &self.value, on_change)
                .padding(10)
                .on_focus(on_focus)
                .on_unfocus(on_focus)
                .width(Length::Fixed(300.0)),
        ]
        .spacing(2)
        .into()
    }
}

struct AuthApp {
    login_mode: bool,
    anim_opacity: f32,
    anim_direction: f32,

    username_input: FloatingInput,
    password_input: FloatingInput,

    login_button: button::State,
    signup_button: button::State,
    toggle_button: button::State,
}

impl Application for AuthApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (AuthApp, Command<Message>) {
        (
            AuthApp {
                login_mode: true,
                anim_opacity: 1.0,
                anim_direction: 0.0,

                username_input: FloatingInput::new("Username"),
                password_input: FloatingInput::new("Password"),

                login_button: button::State::new(),
                signup_button: button::State::new(),
                toggle_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Auth App".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleMode => {
                self.anim_direction = if self.login_mode { -1.0 } else { 1.0 };
            }
            Message::Tick => {
                if self.anim_direction != 0.0 {
                    self.anim_opacity += self.anim_direction * 0.05;
                    if self.anim_opacity <= 0.0 {
                        self.anim_opacity = 0.0;
                        self.login_mode = !self.login_mode;
                        self.anim_direction = 1.0;
                    } else if self.anim_opacity >= 1.0 {
                        self.anim_opacity = 1.0;
                        self.anim_direction = 0.0;
                    }
                }
            }
            Message::UsernameChanged(val) => self.username_input.value = val,
            Message::PasswordChanged(val) => self.password_input.value = val,
            Message::UsernameFocused(focused) => self.username_input.focused = focused,
            Message::PasswordFocused(focused) => self.password_input.focused = focused,
            Message::LoginPressed => {
                println!("Logging in user: {}", self.username_input.value);
            }
            Message::SignupPressed => {
                println!("Signing up user: {}", self.username_input.value);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<Message> {
        let content = if self.login_mode {
            column![
                self.username_input.view(Message::UsernameChanged, Message::UsernameFocused),
                self.password_input.view(Message::PasswordChanged, Message::PasswordFocused),
                Button::new(&mut self.login_button, text("Login"))
                    .on_press(Message::LoginPressed)
                    .padding(10),
            ]
            .spacing(20)
        } else {
            column![
                self.username_input.view(Message::UsernameChanged, Message::UsernameFocused),
                self.password_input.view(Message::PasswordChanged, Message::PasswordFocused),
                Button::new(&mut self.signup_button, text("Sign Up"))
                    .on_press(Message::SignupPressed)
                    .padding(10),
            ]
            .spacing(20)
        };

        let toggle_text = if self.login_mode {
            "Don't have an account? Sign Up"
        } else {
            "Already have an account? Login"
        };

        let toggle_button = Button::new(&mut self.toggle_button, text(toggle_text))
            .on_press(Message::ToggleMode)
            .padding(5);

        container(
            column![content.opacity(self.anim_opacity), toggle_button]
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Alignment::Center)
                .spacing(30),
        )
        .center_x()
        .center_y()
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(30)).map(|_| Message::Tick)
    }
}
