use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
static PART_2_RANKS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let part_1 = get_total_winnings(&contents);
    let part_2 = get_total_winnings_part_2(&contents);
    println!("part 1 {} part 2 {}", part_1, part_2);
}

fn by_rank(cards: [char; 13], a: &Bet, b: &Bet) -> Ordering {
    let type_order = a.hand.hand_type.cmp(&b.hand.hand_type);
    if type_order != Ordering::Equal {
        return type_order;
    }
    for index in 0..6 {
        let card_a_score = cards.iter().position(|&c| c == a.hand.cards[index]);
        let card_b_score = cards.iter().position(|&c| c == b.hand.cards[index]);
        if card_a_score == card_b_score {
            continue;
        }
        if card_a_score < card_b_score {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
    return Ordering::Equal;
}

fn get_total_winnings_part_2(input: &str) -> i32 {
    let mut sorted_winners = split_lines(input)
        .iter()
        .map(|s| Bet::with_jokers(s))
        .collect::<Vec<Bet>>();
    sorted_winners.sort_by(|a, b| by_rank(PART_2_RANKS, a, b));
    //println!("{:?}", sorted_winners);
    sorted_winners
        .iter()
        .enumerate()
        .fold(0, |total, (rank, bet)| {
            //println!(
            //    "total: {} bet: {} cards: {:?}",
            //    total, bet.bid, bet.hand.cards
            //);
            total + (i32::try_from(rank).ok().unwrap() + 1) * bet.bid
        })
}

fn get_total_winnings(input: &str) -> i32 {
    const PART_1_RANKS: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    let mut sorted_winners = split_lines(input)
        .iter()
        .map(|s| Bet::new(s))
        .collect::<Vec<Bet>>();
    sorted_winners.sort_by(|a, b| by_rank(PART_1_RANKS, a, b));
    sorted_winners
        .iter()
        .enumerate()
        .fold(0, |total, (rank, bet)| {
            total + (i32::try_from(rank).ok().unwrap() + 1) * bet.bid
        })
}

fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").filter(|s| *s != "").collect()
}

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd)]
enum HandType {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq)]
struct Bet {
    hand: Hand,
    bid: i32,
}

impl Bet {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        return Bet {
            hand: parse_hand(parts[0]),
            bid: parts[1].parse::<i32>().unwrap(),
        };
    }
    fn with_jokers(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        return Bet {
            hand: parse_hand_joker(parts[0]),
            bid: parts[1].parse::<i32>().unwrap(),
        };
    }
}

fn card_counts_to_hand(card_chars: [char; 5], cards: HashMap<char, i16>) -> Hand {
    if cards.values().any(|count| count == &2) && cards.values().any(|count| count == &3) {
        return Hand {
            hand_type: HandType::FullHouse,
            cards: card_chars,
        };
    }
    if cards
        .values()
        .filter(|count| count == &&2)
        .collect::<Vec<&i16>>()
        .len()
        == 2
    {
        return Hand {
            hand_type: HandType::TwoPair,
            cards: card_chars,
        };
    }
    if cards.values().any(|count| count == &5) {
        return Hand {
            hand_type: HandType::FiveOfAKind,
            cards: card_chars,
        };
    }
    if cards.values().any(|count| count == &4) {
        return Hand {
            hand_type: HandType::FourOfAKind,
            cards: card_chars,
        };
    }
    if cards.values().any(|count| count == &3) {
        return Hand {
            hand_type: HandType::ThreeOfAKind,
            cards: card_chars,
        };
    }
    if cards.values().any(|count| count == &2) {
        return Hand {
            hand_type: HandType::Pair,
            cards: card_chars,
        };
    }
    return Hand {
        hand_type: HandType::HighCard,
        cards: card_chars,
    };
}

fn parse_hand_joker(input: &str) -> Hand {
    const JOKER: char = 'J';
    let jokers = input.chars().filter(|&c| c == JOKER).count();
    let card_chars: [char; 5] = input.chars().collect::<Vec<char>>().try_into().unwrap();
    let mut cards: HashMap<char, i16> =
        input
            .chars()
            .filter(|&c| c != JOKER)
            .fold(HashMap::new(), |mut cards, card| {
                let card_count = cards.entry(card).or_insert(0);
                *card_count += 1;
                cards
            });
    if cards.is_empty() {
        return Hand {
            cards: card_chars,
            hand_type: HandType::FiveOfAKind,
        };
    }
    let most_common_card = cards
        .iter()
        .reduce(|acc, cur| {
            if cur.1 > acc.1 {
                return cur;
            }
            acc
        })
        .unwrap();
    let (key, _) = most_common_card;
    let count = cards.entry(*key).or_insert(0);
    *count += i16::try_from(jokers).ok().unwrap();
    card_counts_to_hand(card_chars, cards)
}

fn parse_hand(input: &str) -> Hand {
    let card_chars: [char; 5] = input.chars().collect::<Vec<char>>().try_into().unwrap();
    let cards: HashMap<char, i16> = input.chars().fold(HashMap::new(), |mut cards, card| {
        let card_count = cards.entry(card).or_insert(0);
        *card_count += 1;
        cards
    });
    card_counts_to_hand(card_chars, cards)
}

#[test]
fn total_winnings_no_duplicates() {
    let input = r#"32T3K 765
KTJJT 220
QQQJA 483"#;
    // 483 * 3 + 220 * 2 + 765
    let total_winnings = get_total_winnings(input);

    assert_eq!(total_winnings, 2654);
}

#[test]
fn example_works() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    let total_winnings = get_total_winnings(input);

    assert_eq!(total_winnings, 6440);
}

#[test]
fn example_works_part_2() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    let total_winnings = get_total_winnings_part_2(input);

    assert_eq!(total_winnings, 5905);
}

#[test]
fn example_2_debugging() {
    let bet = Bet::with_jokers("KTJJT 220");

    assert_eq!(bet.hand.hand_type, HandType::FourOfAKind);
}

#[test]
fn parsing_a_hand_high_card() {
    let hand = parse_hand("23456");
    assert_eq!(hand.hand_type, HandType::HighCard);
}

#[test]
fn parsing_a_hand_pair() {
    let hand = parse_hand("22456");
    assert_eq!(hand.hand_type, HandType::Pair);
}

#[test]
fn parsing_a_hand_three_of_a_kind() {
    let hand = parse_hand("22256");
    assert_eq!(hand.hand_type, HandType::ThreeOfAKind);
}

#[test]
fn parsing_a_hand_two_pair() {
    let hand = parse_hand("22556");
    assert_eq!(hand.hand_type, HandType::TwoPair);
}

#[test]
fn parsing_a_hand_full_house() {
    let hand = parse_hand("22555");
    assert_eq!(hand.hand_type, HandType::FullHouse);
}

#[test]
fn parsing_a_hand_four_of_a_kind() {
    let hand = parse_hand("25555");
    assert_eq!(hand.hand_type, HandType::FourOfAKind);
}

#[test]
fn parsing_a_hand_five_of_a_kind() {
    let hand = parse_hand("55555");
    assert_eq!(hand.hand_type, HandType::FiveOfAKind);
}

#[test]
fn parsing_a_hand_pair_joker() {
    let hand = parse_hand_joker("2J456");
    assert_eq!(hand.hand_type, HandType::Pair);
}

#[test]
fn parsing_a_hand_three_of_a_kind_joker() {
    let hand = parse_hand_joker("2J256");
    assert_eq!(hand.hand_type, HandType::ThreeOfAKind);
}

#[test]
fn parsing_a_hand_four_of_a_kind_joker() {
    let hand = parse_hand_joker("2J226");
    assert_eq!(hand.hand_type, HandType::FourOfAKind);
}

#[test]
fn parsing_a_hand_full_house_joker() {
    let hand = parse_hand_joker("2J266");
    assert_eq!(hand.hand_type, HandType::FullHouse);
}

#[test]
fn parsing_example_hand_part_2() {
    let hand = parse_hand_joker("32T3K");
    assert_eq!(hand.hand_type, HandType::Pair);
}

#[test]
fn parsing_example_hand_2_part_2() {
    let hand = parse_hand_joker("KK677");
    assert_eq!(hand.hand_type, HandType::TwoPair);
}

#[test]
fn parsing_example_hand_3_part_2() {
    assert_eq!(parse_hand_joker("T55J5").hand_type, HandType::FourOfAKind);
    assert_eq!(parse_hand_joker("KTJJT").hand_type, HandType::FourOfAKind);
    assert_eq!(parse_hand_joker("QQQJA").hand_type, HandType::FourOfAKind);
}

#[test]
fn five_of_a_kind_jokers() {
    assert_eq!(parse_hand_joker("JJJJJ").hand_type, HandType::FiveOfAKind);
}

#[test]
fn sorting_part_2() {
    let mut bets = [
        Bet {
            hand: Hand {
                cards: ['3', '2', 'T', '3', 'K'],
                hand_type: HandType::Pair,
            },
            bid: 765,
        },
        Bet {
            hand: Hand {
                cards: ['T', '5', '5', 'J', '5'],
                hand_type: HandType::FourOfAKind,
            },
            bid: 684,
        },
        Bet {
            hand: Hand {
                cards: ['K', 'K', '6', '7', '7'],
                hand_type: HandType::TwoPair,
            },
            bid: 28,
        },
        Bet {
            hand: Hand {
                cards: ['K', 'T', 'J', 'J', 'T'],
                hand_type: HandType::FourOfAKind,
            },
            bid: 220,
        },
        Bet {
            hand: Hand {
                cards: ['Q', 'Q', 'Q', 'J', 'A'],
                hand_type: HandType::FourOfAKind,
            },
            bid: 483,
        },
    ];
    let expected = [
        Bet {
            hand: Hand {
                cards: ['3', '2', 'T', '3', 'K'],
                hand_type: HandType::Pair,
            },
            bid: 765,
        },
        Bet {
            hand: Hand {
                cards: ['K', 'K', '6', '7', '7'],
                hand_type: HandType::TwoPair,
            },
            bid: 28,
        },
        Bet {
            hand: Hand {
                cards: ['T', '5', '5', 'J', '5'],
                hand_type: HandType::FourOfAKind,
            },
            bid: 684,
        },
        Bet {
            hand: Hand {
                cards: ['Q', 'Q', 'Q', 'J', 'A'],
                hand_type: HandType::FourOfAKind,
            },
            bid: 483,
        },
        Bet {
            hand: Hand {
                cards: ['K', 'T', 'J', 'J', 'T'],
                hand_type: HandType::FourOfAKind,
            },
            bid: 220,
        },
    ];
    bets.sort_by(|a, b| by_rank(PART_2_RANKS, a, b));
    assert_eq!(bets, expected);
}
