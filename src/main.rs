use iced::widget::{text, Text, Column, column, text_input, button, Button, row, Row};

pub fn main() -> iced::Result {
    // iced::run("A cool counter", Counter::update, Counter::view)
    // iced::application("Single Task", Task::update, Task::view).theme(|_| iced::Theme::CatppuccinMacchiato).run()
    iced::application("Single Task", State::update, State::view).theme(|_| iced::Theme::CatppuccinMacchiato).run()
}

// #[derive(Default)]
// struct Counter {
//     value: i64,
// }
//
// #[derive(Debug, Clone, Copy)]
// enum Message {
//     Increment,
//     Decrement,
// }
//
// impl Counter {
//     fn update(&mut self, message: Message) {
//         match message {
//             Message::Increment => {
//                 self.value += 1;
//             }
//             Message::Decrement => {
//                 self.value -= 1;
//             }
//         }
//     }
//
//     fn view(&self) -> Column<Message> {
//         column![
//             button("Increment").on_press(Message::Increment),
//             text(self.value).size(50),
//             button("Decrement").on_press(Message::Decrement)
//         ]
//         .padding(20)
//         .align_x(Center)
//     }
// }

#[derive(Clone, Default, Debug)]
struct Task {
    content: String,
    id: uuid::Uuid,
}

impl Task {
    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(new_content) => {
                self.content = new_content; 
            }
            _ => {}
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let content = text_input("", &self.content).width(iced::Length::Fixed(400.0)).on_input(Message::Edit);

        let del = button("del").on_press(Message::Remove(self.id));

        row![content, del].into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Add(String),
    Remove(uuid::Uuid),
    Input(String),
    Edit(String),
}

#[derive(Default, Debug)]
struct State {
    search: String,
    todos: Vec<Task>,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Add(content) => {
                self.todos.push(Task {content, id: uuid::Uuid::new_v4()}) 
            }
            Message::Remove(id) => {
                if let Some(index) = self.todos.iter().position(|x| x.id == id) {
                    self.todos.remove(index);
                }
            }
            Message::Input(search) => {
                self.search = search
            }
            _ => {}
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let search_bar: iced::widget::TextInput<Message> = text_input("Search...", &self.search).on_input(Message::Input).on_submit(Message::Add(self.search.clone()));

        let tasks: Vec<(usize, &Task)> = self.todos.iter().enumerate().map(|(i, task)| (i, task)).collect();

        let tasks: iced::widget::keyed::Column<usize, Message> = iced::widget::keyed_column(tasks.iter().map(|(key, task)| (*key, task.view())));

        column![search_bar, tasks].into()
    }
}
