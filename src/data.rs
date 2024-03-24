use crate::specialtypes::*;
use crate::constants::*;

pub trait Displayable {
    fn display(&self);
}

#[derive (Default)]
pub struct Person {
    pub name: String,
    pub weight_kg: f32,
    pub height_cm: f32,
    pub age: u8,
    pub activity_lvl: ActivityLevel,
    pub sex_assigned_at_birth: SexAtBirth
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
pub struct Macros {
    pub calories: u32,
    pub protein: u32,
    pub carbs: u32,
    pub fat: u32
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