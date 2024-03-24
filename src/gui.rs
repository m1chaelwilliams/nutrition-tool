use iced::widget::{button, text, text_input, Column};
use crate::data::{Macros, Person};
use crate::{inches_to_cm, lbs_to_kg, specialtypes::*, Displayable};

#[derive (Debug, Clone)]
pub enum Message {
    NameChange(String),
    AgeChange(String),
    WeightChange(String),
    HeightChange(String),
    Submit,
}

#[derive (Default)]
pub struct App {
    _measure_system: MeasureSystem,
    name: String,
    age: String,
    weight: String,
    height: String,
    person: Person,
    _macros: Macros,
    calculated: bool,
    try_calculated: bool
}

impl App {
    pub fn parse_entries(&mut self) -> bool {
        let parsed_age: u8 = match self.age.parse::<u8>() {
            Ok(age) => age,
            Err(_) => return false
        };

        let parsed_weight: f32 = match self.weight.parse::<f32>() {
            Ok(weight) => weight,
            Err(_) => return false
        };

        let parsed_height: f32 = match self.height.parse::<f32>() {
            Ok(height) => height,
            Err(_) => return false
        };

        self.person = Person {
            name: self.name.clone(),
            weight_kg: lbs_to_kg(parsed_weight),
            height_cm: inches_to_cm(parsed_height),
            age: parsed_age,
            activity_lvl: ActivityLevel::Active,
            sex_assigned_at_birth: SexAtBirth::Male
        };

        self.person.display();

        true
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Submit => {
                self.calculated = self.parse_entries();
                self.try_calculated = true;
            },
            Message::NameChange(new_name) => self.name = new_name.to_string(),
            Message::AgeChange(new_age) => self.age = new_age.to_string(),
            Message::WeightChange(new_weight) => self.weight = new_weight.to_string(),
            Message::HeightChange(new_height) => self.height = new_height.to_string()
        }
    }

    pub fn view(&self) -> Column<Message> {
        let mut widgets: Vec<iced::Element<Message>> = Vec::new();

        widgets.push(
            text("Nutrition Tool")
                .size(30)
                .color(iced::Color::from_rgb(0.4, 0.6, 1.0))
                .into()
        );

        widgets.push(
            text_input("Name", &self.name)
                .on_input(|input: String| {Message::NameChange(input)})
                .into()
        );
        widgets.push(
            text_input("Enter Age:", &self.age)
                .on_input(|input: String| {Message::AgeChange(input)})
                .into()
        );
        widgets.push(
            text_input("Enter Weight (lbs):", &self.weight)
                .on_input(|input: String| {Message::WeightChange(input)})
                .into()
        );
        widgets.push(
            text_input("Enter Height (inches):", &self.height)
                .on_input(|input: String| {Message::HeightChange(input)})
                .into()
        );
        widgets.push(
            button(text("Calculate"))
                .on_press(Message::Submit)
                .into()
        );

        if self.try_calculated {
            if self.calculated {
                widgets.push(
                    text("Calculated")
                        .into()
                )
            } else {
                widgets.push(
                    text("Invalid responses*")
                        .color(iced::Color::from_rgb(1.0, 0.0, 0.0))
                        .into()
                )
            }
        }

        Column::from_vec(
            widgets
        )
            .padding(6)
            .width(600)
            .height(400)
    }
}