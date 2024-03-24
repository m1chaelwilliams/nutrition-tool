mod ioutils;
mod constants;
mod conversionutils;
mod specialtypes;
mod gui;
mod data;

use std::io::{self, Write};

use ioutils::*;
use conversionutils::*;
use specialtypes::*;
use gui::*;
use data::*;

// exporting
fn export_data(person: &Person, _diet: &Macros) -> io::Result<()> {

    let mut f = std::fs::File::create("result.txt")?;

    f.write_all(format!("{}\n", person.name).as_bytes())?;
    f.write_all(format!("{} kg\n", person.weight_kg).as_bytes())?;

    Ok(())
}

fn run_headless() -> io::Result<()> {
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
            println!("Exporting...");
            export_data(&person, &diet)?;
            println!("Exported");
        }
    }

    Ok(())
}

fn run() {
    iced::run("Nutrition Tool", App::update, App::view).unwrap()
}

fn main() -> std::io::Result<()> {
    println!("Choose an option:");
    println!("1. GUI | 2. Headless");
    if let Ok(response) = io_read_parse::<u8>() {
        if response == 1 {
            run();
        }
        if response == 2 {
            return run_headless();
        }
    };
    Ok(())
}