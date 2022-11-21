#![allow(warnings, unused)]

use crate::utls::read_lines_from_file;
use std::{cmp::max, collections::HashMap};

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl From<String> for Ingredient {
    fn from(text: String) -> Self {
        let parts: Vec<&str> = text.split(' ').collect();
        let name = parts[0].to_string();
        let capacity: i64 = parts[2].trim_end_matches(',').parse().unwrap();
        let durability: i64 = parts[4].trim_end_matches(',').parse().unwrap();
        let flavor: i64 = parts[6].trim_end_matches(',').parse().unwrap();
        let texture: i64 = parts[8].trim_end_matches(',').parse().unwrap();
        let calories: i64 = parts[10].trim_end_matches(',').parse().unwrap();

        Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

fn get_ingredients() -> Vec<Ingredient> {
    read_lines_from_file(r"src/year_15/day_15.txt")
        .into_iter()
        .map(|line| line.into())
        .collect()
}

struct CookiePart<'a> {
    ingredient: &'a Ingredient,
    spoons: i64,
}

impl<'a> CookiePart<'a> {
    fn new(ingredient: &'a Ingredient, spoons: i64) -> Self {
        Self { ingredient, spoons }
    }

    fn capacity(&self) -> i64 {
        self.ingredient.capacity * self.spoons
    }
    fn durability(&self) -> i64 {
        self.ingredient.durability * self.spoons
    }
    fn flavor(&self) -> i64 {
        self.ingredient.flavor * self.spoons
    }
    fn texture(&self) -> i64 {
        self.ingredient.texture * self.spoons
    }
    fn calories(&self) -> i64 {
        self.ingredient.calories * self.spoons
    }
}

fn calc_score(parts: Vec<CookiePart>) -> i64 {
    let mut ing_scores = [0; 4];
    for part in parts {
        ing_scores[0] = ing_scores[0] + part.capacity();
        ing_scores[1] = ing_scores[1] + part.durability();
        ing_scores[2] = ing_scores[2] + part.flavor();
        ing_scores[3] = ing_scores[3] + part.texture();
    }

    if ing_scores.iter().all(|&score| score > 0) {
        ing_scores.iter().product()
    } else {
        0
    }
}
fn calc_calories(parts: &Vec<CookiePart>) -> i64 {
    let mut calories = 0;
    for part in parts {
        calories = calories + part.calories();
    }

    calories
}

fn get_high_score(ings: Vec<Ingredient>) -> i64 {
    let mut max_score = 0;

    for l_1 in 0..100 {
        for l_2 in 0..100 - l_1 {
            for l_3 in 0..100 - l_2 - l_1 {
                let l_4 = 100 - l_3 - l_2 - l_1;
                let parts = vec![
                    CookiePart::new(&ings[0], l_1),
                    CookiePart::new(&ings[1], l_2),
                    CookiePart::new(&ings[2], l_3),
                    CookiePart::new(&ings[3], l_4),
                ];
                max_score = max(max_score, calc_score(parts));
            }
        }
    }

    max_score
}

fn get_high_score_with_calories(ings: Vec<Ingredient>) -> i64 {
    let mut max_score = 0;

    for l_1 in 0..100 {
        for l_2 in 0..100 - l_1 {
            for l_3 in 0..100 - l_2 - l_1 {
                let l_4 = 100 - l_3 - l_2 - l_1;
                let parts = vec![
                    CookiePart::new(&ings[0], l_1),
                    CookiePart::new(&ings[1], l_2),
                    CookiePart::new(&ings[2], l_3),
                    CookiePart::new(&ings[3], l_4),
                ];
                if calc_calories(&parts) == 500 {
                    max_score = max(max_score, calc_score(parts));
                }
            }
        }
    }
    max_score
}

fn part_1() {
    let ingredients = get_ingredients();
    let high_score = get_high_score(ingredients);
    println!("high score is {high_score}");
}

fn part_2() {
    let ingredients = get_ingredients();
    let high_score = get_high_score_with_calories(ingredients);
    println!("high score with calories is {high_score}");
}
pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_ingr() -> Vec<Ingredient> {
        vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8"
                .to_owned()
                .into(),
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
                .to_owned()
                .into(),
        ]
    }

    #[test]
    fn test_calc_score() {
        let ing = get_test_ingr();
        let parts = vec![CookiePart::new(&ing[0], 44), CookiePart::new(&ing[1], 56)];
        assert_eq!(calc_score(parts), 62842880);
    }
}
