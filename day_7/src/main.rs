use std::{fs::File, io::Read};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Hand {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const WORST_HAND_ORD: [Hand; 7] = [
    Hand::HighCard,
    Hand::OnePair,
    Hand::TwoPair,
    Hand::ThreeOfAKind,
    Hand::FullHouse,
    Hand::FourOfAKind,
    Hand::FiveOfAKind,
];

impl From<[u8; 2]> for Hand {
    fn from(value: [u8; 2]) -> Self {
        match value {
            [5, 0] => Hand::FiveOfAKind,
            [4, _] => Hand::FourOfAKind,
            [3, 2] => Hand::FullHouse,
            [3, _] => Hand::ThreeOfAKind,
            [2, 2] => Hand::TwoPair,
            [2, _] => Hand::OnePair,
            [_, _] => Hand::HighCard,
        }
    }
}

mod part_1 {
    use std::{cmp::Ordering, collections::HashMap};

    use crate::{Hand, WORST_HAND_ORD};

    #[derive(Debug, PartialEq, Eq, Ord, Hash, Clone, Copy)]
    enum Card {
        A,
        K,
        Q,
        J,
        T,
        Num(u8),
    }

    impl From<char> for Card {
        fn from(value: char) -> Self {
            match value {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                num => Card::Num(num.to_string().parse::<u8>().unwrap()),
            }
        }
    }

    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let val_1: u8 = self.into();
            let val_2: u8 = other.into();

            val_1.partial_cmp(&val_2)
        }
    }

    impl From<&Card> for u8 {
        fn from(value: &Card) -> Self {
            match value {
                Card::A => 14,
                Card::K => 13,
                Card::Q => 12,
                Card::J => 11,
                Card::T => 10,
                Card::Num(num) => *num,
            }
        }
    }

    pub fn main(content: &str) {
        let all_hands = content
            .split("\n")
            .filter(|line| *line != "")
            .map(|line| {
                let mut parts = line.trim().split(" ");

                (parts.next().unwrap(), parts.next().unwrap())
            })
            .map(|(hand, bet)| {
                (
                    hand.chars().map(|val| val.into()).collect::<Vec<Card>>(),
                    bet.parse::<u32>().unwrap(),
                )
            })
            .map(|(hand, bet)| {
                let mut hand_combo = hand
                    .iter()
                    .fold(HashMap::new(), |mut acc: HashMap<&Card, u8>, card| {
                        match acc.get_mut(card) {
                            Some(count) => {
                                *count = *count + 1;
                            }
                            None => {
                                acc.insert(card, 1);
                            }
                        }
                        acc
                    })
                    .iter()
                    .map(|(_, count)| *count)
                    .collect::<Vec<u8>>();
                hand_combo.sort();
                hand_combo.reverse();

                let hand_combo: Hand = [
                    *hand_combo.get(0).unwrap(),
                    *hand_combo.get(1).unwrap_or(&0),
                ]
                .into();
                (hand_combo, (hand, bet))
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<Hand, Vec<(Vec<Card>, u32)>>, (key, value)| {
                    match acc.get_mut(&key) {
                        Some(hands) => {
                            hands.push(value);
                        }
                        None => {
                            acc.insert(key, vec![value]);
                        }
                    };

                    acc
                },
            );

        let sum: u32 = WORST_HAND_ORD
            .iter()
            .filter_map(|hand| all_hands.get(hand))
            .map(|hands| hands.clone())
            .flat_map(|mut hands| {
                hands.sort_by(|(val_1, _), (val_2, _)| {
                    val_1
                        .iter()
                        .zip(val_2)
                        .find_map(|(val_1, val_2)| match val_1.partial_cmp(val_2).unwrap() {
                            Ordering::Equal => None,
                            val => Some(val),
                        })
                        .unwrap()
                });

                // hands.reverse();

                hands
            })
            .enumerate()
            .map(|(index, (_hand, bet))| {
                // println!("{}:{hand:?}:{bet}", index+1);

                (index as u32 + 1) * bet
            })
            .sum();
        // get

        println!("{:#?}", sum);
    }
}

mod part_2 {
    use std::{cmp::Ordering, collections::HashMap};

    use crate::{Hand, WORST_HAND_ORD};

    #[derive(Debug, PartialEq, Eq, Ord, Hash, Clone, Copy)]
    enum Card {
        A,
        K,
        Q,
        J,
        T,
        Num(u8),
    }

    impl From<char> for Card {
        fn from(value: char) -> Self {
            match value {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                num => Card::Num(num.to_string().parse::<u8>().unwrap()),
            }
        }
    }

    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let val_1: u8 = self.into();
            let val_2: u8 = other.into();

            val_1.partial_cmp(&val_2)
        }
    }

    impl From<&Card> for u8 {
        fn from(value: &Card) -> Self {
            match value {
                Card::A => 14,
                Card::K => 13,
                Card::Q => 12,
                Card::T => 10,
                Card::Num(num) => *num,
                Card::J => 1,
            }
        }
    }

    pub fn main(content: &str) {
        let all_hands = content
            .split("\n")
            .filter(|line| *line != "")
            .map(|line| {
                let mut parts = line.trim().split(" ");

                (parts.next().unwrap(), parts.next().unwrap())
            })
            .map(|(hand, bet)| {
                (
                    hand.chars().map(|val| val.into()).collect::<Vec<Card>>(),
                    bet.parse::<u32>().unwrap(),
                )
            })
            .map(|(hand, bet)| {
                let mut hand_combo =
                    hand.iter()
                        .fold(HashMap::new(), |mut acc: HashMap<&Card, u8>, card| {
                            match acc.get_mut(card) {
                                Some(count) => {
                                    *count = *count + 1;
                                }
                                None => {
                                    acc.insert(card, 1);
                                }
                            }

                            acc
                        });

                // println!("{hand_combo:?}");

                let jacks = hand_combo.remove(&Card::J);

                let mut hand_combo = hand_combo
                    .iter()
                    .map(|(_, count)| *count)
                    .collect::<Vec<u8>>();
                hand_combo.sort();
                hand_combo.reverse();

                let hand_combo: Hand = [
                    *hand_combo.get(0).unwrap_or(&0) + jacks.unwrap_or(0),
                    *hand_combo.get(1).unwrap_or(&0),
                ]
                .into();

                // println!("{hand_combo:?}");
                (hand_combo, (hand, bet))
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<Hand, Vec<(Vec<Card>, u32)>>, (key, value)| {
                    match acc.get_mut(&key) {
                        Some(hands) => {
                            hands.push(value);
                        }
                        None => {
                            acc.insert(key, vec![value]);
                        }
                    };

                    acc
                },
            );

        let sum: u32 = WORST_HAND_ORD
            .iter()
            .filter_map(|hand| all_hands.get(hand))
            .map(|hands| hands.clone())
            .flat_map(|mut hands| {
                hands.sort_by(|(val_1, _), (val_2, _)| {
                    val_1
                        .iter()
                        .zip(val_2)
                        .find_map(|(val_1, val_2)| match val_1.partial_cmp(val_2).unwrap() {
                            Ordering::Equal => None,
                            val => Some(val),
                        })
                        .unwrap()
                });

                // hands.reverse();

                hands
            })
            .enumerate()
            .map(|(index, (_hand, bet))| {
                // println!("{}:{hand:?}:{bet}", index+1);

                (index as u32 + 1) * bet
            })
            .sum();
        // get

        println!("{:#?}", sum);
    }
}

fn main() {
    let mut f: File = File::open("input.txt").unwrap();

    let mut content = String::new();

    let _result = f.read_to_string(&mut content).unwrap();

    let content = content.replace("\r", "");

    part_1::main(&content);

    part_2::main(&content);
}
