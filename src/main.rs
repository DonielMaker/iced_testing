use iced::widget::{text,  column, text_input, row, TextInput, button, keyed, keyed_column};

pub fn main() -> iced::Result {
    // iced::run("A cool counter", Counter::update, Counter::view)
    // iced::application("Single Task", Task::update, Task::view).theme(|_| iced::Theme::CatppuccinMacchiato).run()
    iced::application("Single Task", State::update, State::view).theme(|_| iced::Theme::CatppuccinMacchiato).run()
}

// #[derive(Clone, Default, Debug)]
// struct Task {
//     content: String,
//     id: uuid::Uuid,
// }
//
// impl Task {
//     fn view(&self) -> iced::Element<Message> {
//         let content = text_input("", &self.content).width(iced::Length::Fixed(400.0)).on_input(Message::Edit());
//
//         let del = button("del").on_press(Message::Remove(self.id));
//
//         row![content, del].into()
//     }
// }

#[derive(Debug, Clone)]
enum Message {
    Add(String),
    Remove(usize),
    Input(String),
}

#[derive(Default, Debug)]
struct State {
    search: String,
    todos: Vec<String>,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Add(content) => {
                self.todos.push(content);
            }
            Message::Remove(index) => {
                self.todos.remove(index);
            }
            Message::Input(search) => {
                self.search = search
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let search_bar: TextInput<Message> = text_input("Search...", &self.search).on_input(Message::Input).on_submit(Message::Add(self.search.clone()));

        let tasks: keyed::Column<usize, Message> = keyed_column(
            self.todos
                .iter()
                .enumerate()
                .map(|(key, task)| {
                    (key, row![text(task).width(iced::Length::Fill), button("Del").on_press(Message::Remove(key))].into())
                })
        );

        column![search_bar, tasks].into()
    }
}
