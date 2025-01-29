use iced::widget::{text, Text, Column, column, text_input, button, Button, row, Row};

pub fn main() -> iced::Result {
    // iced::run("A cool counter", Counter::update, Counter::view)
    // iced::run("Todo List", TodoList::update, TodoList::view)
    // iced::run("Single Task", Task::update, Task::view)
    iced::application("Single Task", Task::update, Task::view).theme(|_| iced::Theme::CatppuccinMacchiato).run()
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

enum StateMessage {
    Add,
    Remove,
    Input
}

#[derive(Debug, Clone)]
enum TaskMessage {
    Edit(String)
}
struct State {
    search: String,
    todos: Vec<Task>,
}

#[derive(Clone)]
struct Task {
    content: String,
    id: uuid::Uuid,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            content: String::from("This is a new Task"),
            id: uuid::Uuid::new_v4()
        }
    }
}

impl Task {
    fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Edit(new_content) => {
                self.content = new_content; 
            }
        }
    }

    fn view(&self) -> Row<TaskMessage>{
        let text = text(&self.content).width(iced::Length::Fill).align_x(iced::Center).align_y(iced::Center);

        row![text, button("del")]
    }
}

// #[derive(Debug, Clone)]
// enum Message {
//     Add(String),
//     Remove(i32),
//     Input(String),
// }
//
// #[derive(Default)]
// struct TodoList {
//     search: String,
//     items: Vec<(String, i32)>, 
// }
//
//
// impl TodoList {
//     fn add(&mut self, text: String) {
//         self.update(Message::Add(text))
//     }
//
//     fn remove(&mut self, id: i32) {
//         self.update(Message::Remove(id))
//     }
//
//     fn update(&mut self, action: Message) {
//         match action {
//             Message::Add(text) => {
//                 self.items.push((text, rand::random::<i32>()));
//             }
//             Message::Remove(id) => {
//                 if let Some(index) = self.items.iter().position(|(_, x)| *x == id) {
//                     self.items.remove(index);
//                 }
//             }
//             Message::Input(search) => {
//                 self.search = search;
//             }
//         } 
//     }
//
//     fn view(&self) -> Column<Message>{
//         let search_bar: iced::widget::TextInput<Message> = text_input("Search...", &self.search).on_input(Message::Input).on_submit(Message::Add(self.search.clone()));
//
//         let mut list: Column<Message> = Column::new();
//         for item in self.items.clone() {
//             let button: Button<Message> = button(item.0).on_press(Message::Remove(item.1));
//             list = list.push(button)
//         }
//
//
//         column![
//             search_bar,
//             list
//         ]
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {
//         let mut list = TodoList {search: String::new(), items: Vec::new()};
//
//         list.add("BlaBla".into());
//
//         assert_eq!("BlaBla", list.items[0].0);
//
//         list.remove(list.items[0].1);
//
//         assert!(list.items.is_empty());
//     }
// }
