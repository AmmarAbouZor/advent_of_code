#![allow(warnings, unused)]

#[derive(Debug, PartialEq, Eq)]
struct Item {
    name: String,
    cost: u32,
    damage: i8,
    armor: i8,
}

impl From<&str> for Item {
    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let len = parts.len();

        Item {
            name: parts[0].into(),
            cost: parts[len - 3].parse().unwrap(),
            damage: parts[len - 2].parse().unwrap(),
            armor: parts[len - 1].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct ItemShop {
    weapons: Vec<Item>,
    armors: Vec<Item>,
    rings: Vec<Item>,
}

impl ItemShop {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let mut weapons = (1..6).map(|i| lines[i].into()).collect();
        let mut armors = (8..13).map(|i| lines[i].into()).collect();
        let mut rings = (15..lines.len()).map(|i| lines[i].into()).collect();

        Self {
            weapons,
            armors,
            rings,
        }
    }
}

fn get_item_shop() -> ItemShop {
    let input = r"Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3
";
    ItemShop::new(input)
}

#[derive(Debug, Clone)]
struct Player {
    hit_points: i16,
    damage: i8,
    armor: i8,
}

impl Player {
    fn new(hit_points: i16, damage: i8, armor: i8) -> Self {
        Self {
            hit_points,
            damage,
            armor,
        }
    }

    fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    fn fight(mut self, mut enemy: Player) -> bool {
        let mut index = 0;
        while self.is_alive() && enemy.is_alive() {
            if index % 2 == 0 {
                enemy.receive_hit(&self);
            } else {
                self.receive_hit(&enemy);
            }
            index += 1;
        }

        self.is_alive()
    }

    fn receive_hit(&mut self, enemy: &Player) {
        let total_damage = match enemy.damage - self.armor {
            damage if damage > 0 => damage,
            _ => 1,
        };
        self.hit_points -= total_damage as i16;
    }
}

fn get_boss() -> Player {
    Player::new(109, 8, 2)
}

struct StartStatusBuilder<'a> {
    weapon: &'a Item,
    armor: Option<&'a Item>,
    ring_1: Option<&'a Item>,
    ring_2: Option<&'a Item>,
}

impl<'a> StartStatusBuilder<'a> {
    fn new(
        weapon: &'a Item,
        armor: Option<&'a Item>,
        ring_1: Option<&'a Item>,
        ring_2: Option<&'a Item>,
    ) -> Self {
        Self {
            weapon,
            armor,
            ring_1,
            ring_2,
        }
    }

    fn get_optionals(&self) -> [Option<&Item>; 3] {
        [self.armor, self.ring_1, self.ring_2]
    }

    fn get_costs(&self) -> u32 {
        let mut costs = self.weapon.cost;

        costs
            + self
                .get_optionals()
                .into_iter()
                .filter_map(|opt| opt)
                .map(|item| item.cost)
                .sum::<u32>()
    }

    fn create_player(&self, hit_points: i16) -> Player {
        let mut damage = self.weapon.damage;
        let mut armor = self.weapon.armor;

        self.get_optionals()
            .into_iter()
            .filter_map(|opt| opt)
            .for_each(|item| {
                damage += item.damage;
                armor += item.armor;
            });

        Player::new(hit_points, damage, armor)
    }
}

fn part_1() {
    let mut costs = u32::MAX;

    let shop = get_item_shop();

    for weapon in shop.weapons.iter() {
        for armor in shop.armors.iter().map(|armor| Some(armor)).chain([None]) {
            for ring_1 in shop.rings.iter().map(|ring| Some(ring)).chain([None]) {
                for ring_2 in shop
                    .rings
                    .iter()
                    .map(|ring| Some(ring))
                    .chain([None])
                    .filter(|&ring| ring == None || ring != ring_1)
                {
                    let builder = StartStatusBuilder::new(weapon, armor, ring_1, ring_2);
                    let player = builder.create_player(100);
                    if player.fight(get_boss()) {
                        costs = costs.min(builder.get_costs());
                    }
                }
            }
        }
    }

    println!("min cost is {costs}");
}

fn part_2() {
    let mut costs = 0;

    let shop = get_item_shop();

    for weapon in shop.weapons.iter() {
        for armor in shop.armors.iter().map(|armor| Some(armor)).chain([None]) {
            for ring_1 in shop.rings.iter().map(|ring| Some(ring)).chain([None]) {
                for ring_2 in shop
                    .rings
                    .iter()
                    .map(|ring| Some(ring))
                    .chain([None])
                    .filter(|&ring| ring == None || ring != ring_1)
                {
                    let builder = StartStatusBuilder::new(weapon, armor, ring_1, ring_2);
                    let player = builder.create_player(100);
                    if !player.fight(get_boss()) {
                        costs = costs.max(builder.get_costs());
                    }
                }
            }
        }
    }

    println!("max cost is {costs}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fight() {
        let mut player = Player::new(8, 5, 5);
        let mut boss = Player::new(12, 7, 2);
        assert!(player.fight(boss));

        let mut player = Player::new(8, 1, 5);
        let mut boss = Player::new(8, 1, 12);
        assert!(player.fight(boss));

        let mut player = Player::new(8, 1, 5);
        let mut boss = Player::new(8, 1, 12);
        assert!(boss.fight(player));
    }
}
