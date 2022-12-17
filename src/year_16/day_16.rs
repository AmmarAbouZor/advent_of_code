#[derive(Debug)]
struct Data {
    data: Vec<bool>,
}

impl Data {
    fn create(input: &str) -> Self {
        let data = input
            .chars()
            .map(|ch| match ch {
                '0' => false,
                '1' => true,
                _ => unreachable!(),
            })
            .collect();

        Self { data }
    }

    fn increase(&mut self) {
        let count = self.data.len();

        self.data.reserve(count + 1);

        self.data.push(false);

        for i in (0..count).rev() {
            self.data.push(!self.data[i]);
        }
    }

    fn increase_to(&mut self, size: usize) {
        while self.data.len() < size {
            self.increase();
        }

        self.data.drain(size..);
    }

    fn calc_checksum(&mut self) -> String {
        let calc_from = |input: &[bool]| {
            input
                .chunks(2)
                .map(|ch| ch[0] == ch[1])
                .collect::<Vec<bool>>()
        };

        let is_odd = |num: usize| num & 1 != 0;

        let mut checksum = calc_from(&self.data);

        while !is_odd(checksum.len()) {
            checksum = calc_from(&checksum);
        }

        checksum
            .into_iter()
            .map(|b| match b {
                false => '0',
                true => '1',
            })
            .collect()
    }
}

fn part_1() {
    let mut data = Data::create("00101000101111010");
    data.increase_to(272);
    let checksum = data.calc_checksum();

    println!("part_1: checksum is {checksum}");
}

fn part_2() {
    let mut data = Data::create("00101000101111010");
    data.increase_to(35651584);
    let checksum = data.calc_checksum();

    println!("part_2: checksum is {checksum}");
}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_checksum() {
        let mut data = Data::create("10000");
        data.increase_to(20);
        assert_eq!(data.calc_checksum(), "01100");
    }
}
