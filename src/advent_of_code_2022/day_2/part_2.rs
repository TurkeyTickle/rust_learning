/*

*/

/*
    COLUMN 1 (OPPONENT'S MOVE)
    A: rock
    B: paper
    C: scissors

    COLUMN 2 (DESIRED OUTCOME)
    X: lose
    Y: draw
    Z: win

    SELF MOVE
    X: rock
    Y: paper
    Z: scissors

    SCORE VALUES
    1: rock
    2: paper
    3: scissors

    SCORE OF EACH ROUND IS SCORE VALUE PLUS
    0: loss
    3: draw
    6: win
*/

use std::{collections::HashMap, str::Lines, ptr::null};

static INPUT: &'static str = include_str!("input.txt");

pub fn get_self_total_score() -> u32 {
    let mut total_score = 0;
    let lines: Lines = INPUT.lines();

    let mut opponent_scores = HashMap::new();
    opponent_scores.insert(String::from("A"), 1);
    opponent_scores.insert(String::from("B"), 2);
    opponent_scores.insert(String::from("C"), 3);

    let mut self_scores = HashMap::new();
    self_scores.insert(String::from("X"), 1);
    self_scores.insert(String::from("Y"), 2);
    self_scores.insert(String::from("Z"), 3);

    for line in lines {
        let moves: String = line.split(" ").collect();
        let opponent_move = moves.get(..1).unwrap();
        let desired_outcome = moves.get(1..).unwrap();
        let self_move: &str;
        let mut win = false;
        let mut draw = false;

        if desired_outcome == "X" {
            if opponent_move == "A" {
                self_move = "Z";
            } else if opponent_move == "B" {
                self_move = "X"
            } else {
                self_move = "Y"
            }
        } else if desired_outcome == "Y" {
            if opponent_move == "A" {
                self_move = "X";
            } else if opponent_move == "B" {
                self_move = "Y"
            } else {
                self_move = "Z"
            }
        } else {
            if opponent_move == "A" {
                self_move = "Y";
            } else if opponent_move == "B" {
                self_move = "Z"
            } else {
                self_move = "X"
            }
        }

        let opponent_move_score = opponent_scores[opponent_move];
        let self_move_score = self_scores[self_move];

        if self_move == "X" && opponent_move == "C"
            || self_move == "Y" && opponent_move == "A"
            || self_move == "Z" && opponent_move == "B"
        {
            win = true;
        } else if self_move_score == opponent_move_score {
            draw = true;
        }

        if win {
            total_score += self_move_score + 6;
        } else if draw {
            total_score += self_move_score + 3;
        } else {
            total_score += self_move_score + 0;
        }
    }

    total_score
}
