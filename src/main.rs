use iced::widget::{text,  column, text_input, row, TextInput, button, keyed, keyed_column};

pub fn main() -> iced::Result {
    iced::application("Single Task", State::update, State::view).theme(|_| iced::Theme::CatppuccinMacchiato).run()
}

#[derive(Debug, Clone)]
enum Message {
    Add(String),
    Remove(usize),
    Input(String),
    Search(String),
    Change(usize, String),
}

#[derive(Default, Debug)]
struct State {
    input: String,
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
            Message::Input(input) => {
                self.input = input
            }
            Message::Search(search) => {
                self.search = search
            }
            Message::Change(id, text) => {
                self.todos[id] = text
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let input_bar: TextInput<Message> = text_input("Enter new task...", &self.input).on_input(Message::Input).on_submit(Message::Add(self.input.clone()));
        let search_bar: TextInput<Message> = text_input("Search...", &self.search).on_input(Message::Search);

        let tasks = get_tasks(self);

        column![input_bar, search_bar, tasks].into()
    }
}

fn get_tasks(todo: &State) -> iced::Element<Message> {
        if todo.search.is_empty() {
            keyed_column(
                todo.todos
                    .iter()
                    .enumerate()
                    .map(|(key, task)| {
                        (key, row![text_input("", task).width(iced::Length::Fill).on_input(move |text| Message::Change(key, text)), button("Del").on_press(Message::Remove(key))].into())
                    })
            ).into()
        } else {
            keyed_column(
                todo.todos
                    .iter()
                    .filter(|x| x.contains(&todo.search))
                    .enumerate()
                    .map(|(key, task)| {
                        (key, row![text_input("", task).width(iced::Length::Fill).on_input(move |text| Message::Change(key, text)), button("Del").on_press(Message::Remove(key))].into())
                    })
            ).into()
        }
}
