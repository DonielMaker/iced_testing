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
    search: bool,
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
                self.search = false;
                self.input = input
            }
            Message::Search(search) => {
                self.search = true;
                self.input = search
            }
            Message::Change(id, text) => {
                if self.search {return;}
                self.todos[id] = text;
                if self.todos[id].is_empty() {
                    self.update(Message::Remove(id))
                }

            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let input_bar = text_input("Enter new task...", &self.input)
            .on_input(|text| {
                if text.get(0..1) == Some("?") {
                    Message::Search(text)
                } else {
                    Message::Input(text)
                }
            })
            .on_submit(Message::Add(self.input.clone()));

        let tasks = self.get_tasks();

        column![input_bar, tasks].into()
    }

    fn get_tasks(&self) -> iced::Element<Message> {
        match self.search {
            true => keyed_column(
                self.todos
                .iter()
                .filter(|x| if let Some(query) = self.input.get(1..) {x.contains(query)} else {true})
                .enumerate()
                .map(|(id, task)| self.task(id, task))
            ).into(),

            false => keyed_column(
                self.todos
                .iter()
                .enumerate()
                .map(|(id, task)| self.task(id, task))
            ).into()

        }
    }

    /// Singular Task Widget (with id)
    fn task(&self, id: usize, task: &String) -> (usize, iced::Element<Message>) {
        (id, row![text_input("", task).width(iced::Length::Fill).on_input(move |text| Message::Change(id, text)), button("Del").on_press(Message::Remove(id))].into())
    }
}

