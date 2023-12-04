use std::{collections::HashMap, cmp::max};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let games = parse(contents);

        let game = games.get(0).unwrap();

        assert_eq!(6, game.total(Color::Blue));
        assert_eq!(4, game.total(Color::Red));
        assert_eq!(2, game.total(Color::Green));
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub id: u16,
    pub cubes: Vec<Cubes>,
}

impl Game {
    pub fn total(&self, color: Color) -> u8 {
        let mut total = 0;
        for cube in &self.cubes {
            let c = cube.clone().cubes;
            let n = match c.get(&color) {
                Some(x) => *x,
                None => 0,
            };
            total = max(total, n);
        }
        total
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Green,
    Red,
    Blue,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cubes {
    cubes: HashMap<Color, u8>,
}

impl Cubes {
    pub fn total(&self) -> HashMap<Color, u8> {
        let mut total: HashMap<Color, u8> = HashMap::from([]);
        for cube in &self.cubes {
            let (color, count) = cube;
            // println!("color: {:?}, count: {}, total: {:?}", color, count, total);
            let prev_count = match total.get(color) {
                Some(x) => *x,
                None => 0,
            };
            // println!("prev count: {}", prev_count);
            let new_count = max(prev_count, *count);
            // println!("new count: {}", new_count);
            total.insert(*color, new_count);
        }
        return total;
    }
}

pub struct Configuration {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn parse(contents: &str) -> Vec<Game> {
    let mut games = Vec::new();

    // print!("running...");

    for line in contents.lines() {
        // println!("{}", line);
        if line.contains("Game ") {
            let game_str = line.strip_prefix("Game ");
            // println!("{:?}", game_str);
            let game: Vec<&str> = match game_str {
                Some(x) => x.split(":").collect(),
                None => break,
            };
            let mut game_id: u16 = 0;
            let game_picks = match game[..] {
                [_game_id, game_content] => {
                    // println!("Game: {}", _game_id);
                    game_id = _game_id.parse().unwrap();
                    game_content
                }
                _ => break,
            };
            // println!("game_picks: {}", game_picks);
            let picks = game_picks.split(";");
            let mut t = Vec::<Cubes>::new();
            for pick in picks {
                let cube_groups: Vec<&str> = pick.split(",").collect();
                let mut c = HashMap::new();
                for cubes in cube_groups {
                    // println!("cube: {}", cubes.trim());
                    let p: Vec<&str> = cubes.trim().split(" ").collect();
                    let [num, _color] = match p[..] {
                        [num, color] => [num, color],
                        _ => ["0", "beige"],
                    };
                    // println!("number: {}, color: {}", num, color);
                    let count: u8 = num.parse().unwrap();
                    let color = match _color {
                        "green" => Some(Color::Green),
                        "red" => Some(Color::Red),
                        "blue" => Some(Color::Blue),
                        _ => None,
                    };
                    c.insert(color.unwrap().clone(), count);
                    // println!("{:?}", c)
                }
                // println!("{}", pick);
                // println!("{:?}", c);
                let _c = Cubes {
                    cubes: HashMap::from(c),
                };
                t.append(&mut vec![_c]);
            }
            games.push(Game {
                id: game_id,
                cubes: Vec::from(t),
            })
        }
    }

    games
}
