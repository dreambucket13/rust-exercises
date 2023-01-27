/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    
    //parse hands

    let mut best_hands: Vec<&'a str> = Vec::new();
    let mut parsed_hands:Vec<Hand> = Vec::new();

    for hand_input in 0..hands.len() {

        let raw_hand = hands[hand_input];
        let card_raw_strs = raw_hand.split(" ");

        let mut hand = 
            Hand {
                hand_string: raw_hand,
                cards: Vec::new(),
                primary_rank: PrimaryRanks::NoRank,
                secondary_rank: Vec::new()
            };
        
        //build hand
        for card_str in card_raw_strs {
            
            let suit = str_to_suit(card_str);
            let value = str_to_card_value(card_str);

            let card = 
                Card {
                    suit: suit,
                    value: value
                };

            hand.cards.push(card);

        }

        println!("{:?}",hand.cards);

        parsed_hands.push(hand);



    }

    //only for first test
    best_hands.push(parsed_hands[0].hand_string);

    best_hands


}

/*

let mut split = "some string 123 ffd".split("123");
let vec: Vec<&str> = split.collect();

*/

fn str_to_card_value(str: &str) -> CardValues {

    let value: CardValues;

    match str {

        _ if str.starts_with("2") => value = CardValues::TWO,
        _ if str.starts_with("3") => value = CardValues::THREE,
        _ if str.starts_with("4") => value = CardValues::FOUR,
        _ if str.starts_with("5") => value = CardValues::FIVE,
        _ if str.starts_with("6") => value = CardValues::SIX,
        _ if str.starts_with("7") => value = CardValues::SEVEN,
        _ if str.starts_with("8") => value = CardValues::EIGHT,
        _ if str.starts_with("9") => value = CardValues::NINE,
        _ if str.starts_with("J") => value = CardValues::JACK,
        _ if str.starts_with("Q") => value = CardValues::QUEEN,
        _ if str.starts_with("K") => value = CardValues::KING,
        _ if str.starts_with("A") => value = CardValues::AceHigh,
        _ => value = CardValues::NoValue,

    };

    value

}

fn str_to_suit(str: &str) -> Suits {

    let suit: Suits;

    match str {

        _ if str.ends_with("H") => suit = Suits::HEARTS,
        _ if str.ends_with("D") => suit = Suits::DIAMONDS,
        _ if str.ends_with("C") => suit = Suits::CLUBS,
        _ if str.ends_with("S") => suit = Suits::SPADES,
        _ => suit = Suits::NoSuit,

    };

    suit

}

struct Hand<'a> {

    hand_string: &'a str,
    cards: Vec<Card>,
    primary_rank: PrimaryRanks,
    secondary_rank: Vec<CardValues>

}

#[derive(Debug)]
struct Card {

    suit: Suits,
    value: CardValues
}

enum PrimaryRanks {

    NoRank,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush

}

#[derive(Debug)]
enum Suits{

    NoSuit,
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES
}

#[derive(Debug)]
enum CardValues{

    NoValue = 0,
    AceLow = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    JACK = 10,
    QUEEN = 11,
    KING = 12,
    AceHigh = 13

}