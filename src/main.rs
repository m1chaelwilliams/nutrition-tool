mod ioutils;
mod constants;
mod conversionutils;
mod specialtypes;

use std::io::{self, Write};

use ioutils::*;
use constants::*;
use conversionutils::*;
use specialtypes::*;

trait Displayable {
    fn display(&self);
}

#[derive (Default)]
struct Person {
    name: String,
    weight_kg: f32,
    height_cm: f32,
    age: u8,
    activity_lvl: ActivityLevel,
    sex_assigned_at_birth: SexAtBirth
}

impl Person {
    pub fn get_bmr(&self) -> f32 {
        match self.sex_assigned_at_birth {
            SexAtBirth::Male => self.get_bmr_men(),
            SexAtBirth::Female => self.get_bmr_women()
        }
    }

    fn get_bmr_men(&self) -> f32 {
        88.362 + 
        (13.397 * self.weight_kg) + 
        (4.799 * self.height_cm) - 
        (5.677 * self.age as f32)
    }
    fn get_bmr_women(&self) -> f32 {
        447.593 + 
        (9.247 * self.weight_kg) + 
        (3.098 * self.height_cm) - 
        (4.330 * self.age as f32)
    }

    pub fn get_tdee(&self, bmr: f32) -> f32 {
        match self.activity_lvl {
            ActivityLevel::Sedentary => bmr * 1.2,
            ActivityLevel::LightlyActive => bmr * 1.375,
            ActivityLevel::Active => bmr * 1.55,
            ActivityLevel::VeryActive => bmr * 1.725,
            ActivityLevel::SuperActive => bmr * 1.9
        }
    }
}

impl Displayable for Person {
    fn display(&self) {
        println!();
        println!("Name: {}", self.name);
        println!("weight: {} kg", self.weight_kg);
        println!("height: {} cm", self.height_cm);
        println!("age: {} yrs", self.age);
        println!("activity lvl: {}", match self.activity_lvl {
            ActivityLevel::Sedentary => "Sedentary",
            ActivityLevel::LightlyActive => "Lightly active",
            ActivityLevel::Active => "Active",
            ActivityLevel::VeryActive => "Very active",
            ActivityLevel::SuperActive => "Super active"
        });
        println!("Sex assigned at birth: {}", match self.sex_assigned_at_birth {
            SexAtBirth::Male => "Male",
            SexAtBirth::Female => "Female"
        });
        println!();
    }
}

#[derive (Default)]
struct Macros {
    calories: u32,
    protein: u32,
    carbs: u32,
    fat: u32
}

impl Displayable for Macros {
    fn display(&self) {
        println!();
        println!("Calories: {}", self.calories);
        println!("protein: {}", self.protein);
        println!("carbs: {}", self.carbs);
        println!("fat: {}", self.fat);
        println!();
    }
}

impl Macros {

    pub fn calculate_macros(&mut self, target_calories: f32, weight_kg: f32) {

        self.calories = target_calories as u32;

        // 2g protein / kg weight
        self.protein = 2*weight_kg as u32;

        // calculate fat as 25% of caloric intake
        self.fat = (self.calories as f32 * 0.25) as u32 / FAT_CALORIES;

        // get remaining calories
        let mut remaining_calories = self.calories;
        remaining_calories -= self.fat * FAT_CALORIES;
        remaining_calories -= self.protein * PROTEIN_CALORIES;

        // fill remaining calories with carbs
        self.carbs = remaining_calories / CARB_CALORIES;

    }
}

// exporting
fn export_data(person: &Person, _diet: &Macros) -> io::Result<()> {

    let mut f = std::fs::File::create("result.txt")?;

    f.write_all(format!("{}\n", person.name).as_bytes())?;
    f.write_all(format!("{} kg\n", person.weight_kg).as_bytes())?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut person: Person = Person::default();
    let mut diet = Macros::default();

    println!("What measuring system are you comfortable with?");
    println!("1. Imperial | 2. Metric");
    let system_choice = io_read_parse::<u8>()?;
    let system = match system_choice {
        1 => MeasureSystem::Imperial,
        2 => MeasureSystem::Metric,
        _ => MeasureSystem::Imperial
    };

    // populate person with user input
    println!("Enter name: ");
    person.name = io_read_strip()?;
    person.weight_kg = match system {
        MeasureSystem::Imperial => {
            println!("Enter weight in lbs:");
            lbs_to_kg(io_read_parse::<f32>()?)
        },
        MeasureSystem::Metric => {
            println!("Enter weight in kg:");
            io_read_parse::<f32>()?
        }
    };
    person.height_cm = match system {
        MeasureSystem::Imperial => {
            println!("Enter height in inches:");
            inches_to_cm(io_read_parse::<f32>()?)
        },
        MeasureSystem::Metric => {
            println!("Enter height in cm:");
            io_read_parse::<f32>()?
        }
    };
    println!("Enter age in years:");
    person.age = io_read_parse::<u8>()?;
    println!("Enter your activity level (1-5):");
    person.activity_lvl = match io_read_parse::<u8>()? {
        1 => ActivityLevel::Sedentary,
        2 => ActivityLevel::LightlyActive,
        3 => ActivityLevel::Active,
        4 => ActivityLevel::VeryActive,
        5 => ActivityLevel::SuperActive,
        _ => ActivityLevel::Active
    };
    println!("Enter sex assigned at birth:");
    println!("1. Male | 2. Female");
    person.sex_assigned_at_birth = match io_read_parse::<u8>()? {
        1 => SexAtBirth::Male,
        2 | _ => SexAtBirth::Female
    };

    // display inputted data
    person.display();
    println!("Is this correct? (y/n)");
    if let Ok(response) = io_read_strip() {
        if response != "y" {
            println!("Shuttin down...");
            return Ok(());
        }
    }

    println!("Calculating your diet...");
    let bmr = person.get_bmr();
    let tdee = person.get_tdee(bmr);
    println!("Your BMR (Basal Metabolic rate): {}", bmr);
    println!("Your TDEE (Total Daily Energy Expenditure): {}", tdee);

    println!("Would you like to see your diet? (y/n)");
    if let Ok(response) = io_read_strip() {
        if response == "y" {
            diet.calculate_macros(tdee, person.weight_kg);
            diet.display();
        }
    }

    println!("Would you like to export the data? (y/n)");
    if let Ok(response) = io_read_strip() {
        if response == "y" {
            export_data(&person, &diet)?;
        }
    }

    Ok(())
}