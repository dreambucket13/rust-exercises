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
                value_count: vec!(0; 13),
                suit_count: vec!(0; 4),
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

            //hand.value_count[(value as i32)] += 1;

        }

        //sort cards by value

        println!("{:?}",hand.cards);
        hand.cards.sort_unstable_by_key(|card| card.value);
        println!("{:?}",hand.cards);

        parsed_hands.push(hand);

    }

    //assign ranks

    println!("{:?}",parsed_hands[0]);
    assign_rank(&mut parsed_hands[0]);
    println!("{:?}",parsed_hands[0]);
    
    //push highest ranks to best hands

    //only for first test
    best_hands.push(parsed_hands[0].hand_string);

    best_hands


}

fn assign_rank(hand_to_score: &mut Hand){

    type HandDetectFunction  = fn(&mut Hand) -> bool;

    let hand_detect_function: [HandDetectFunction; 9] = [detect_straight_flush,
                                                        detect_four_of_a_kind,
                                                        detect_full_house,
                                                        detect_flush,
                                                        detect_straight,
                                                        detect_three_of_a_kind,
                                                        detect_two_pair,                                                            
                                                        detect_one_pair,
                                                        detect_high_card];

                                                        
    for index in 0..hand_detect_function.len(){
        
        if hand_detect_function[index](hand_to_score) == true{
            break;
        }

    }

}


fn detect_straight_flush(hand_to_score: &mut Hand) -> bool{
    //detect straight flush - contains five cards of sequential rank, all of the same suit

    if detect_flush(hand_to_score) && detect_straight(hand_to_score){
        hand_to_score.primary_rank = PrimaryRanks::StraightFlush;
        //push high card value to secondary rank (hand is already sorted)
        hand_to_score.secondary_rank.push(hand_to_score.cards[4].value);
        return true;
    }

    false

}

fn detect_four_of_a_kind(hand_to_score: &mut Hand) -> bool {
    
    false
}

fn detect_full_house(hand_to_score: &mut Hand) -> bool {

    false
    
}

fn detect_flush(hand_to_score: &mut Hand) -> bool {

    let mut prior_card = &hand_to_score.cards[0];

    for card in &hand_to_score.cards[1..]{

        if card.suit != prior_card.suit {
            return false;
        }

        prior_card = card;
    }

    hand_to_score.primary_rank = PrimaryRanks::Flush;
    return true;

}

fn detect_straight(hand_to_score: &mut Hand) -> bool {

    let mut prior_card = &hand_to_score.cards[0];


    for card in &hand_to_score.cards[1..]{

        if !prior_card.value.is_next(card.value) {
            return false;
        }

        prior_card = card;

    }

    //need special case for ace low straight flush?  simply check for 1 each of A,2,3,4,5
    
    hand_to_score.primary_rank = PrimaryRanks::Straight;
    return true;
}

fn detect_three_of_a_kind(hand_to_score: &mut Hand) -> bool{

    false
    
}

fn detect_two_pair(hand_to_score: &mut Hand) -> bool{

    false
    
}

fn detect_one_pair(hand_to_score: &mut Hand) -> bool{
    false
}

fn detect_high_card(hand_to_score: &mut Hand) -> bool{
    
    true
}


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

#[derive(Debug)]
struct Hand<'a> {

    hand_string: &'a str,
    cards: Vec<Card>,
    value_count: Vec<i32>,
    suit_count: Vec<i32>,
    primary_rank: PrimaryRanks,
    secondary_rank: Vec<CardValues>

}

#[derive(Debug, PartialEq, PartialOrd)]
struct Card {

    suit: Suits,
    value: CardValues
}

#[derive(Debug, PartialEq, PartialOrd)]
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

#[derive(Debug, PartialEq, PartialOrd)]
enum Suits{

    NoSuit,
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
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

impl CardValues {

    fn is_next(self: CardValues, next_card: CardValues) -> bool{

        let current_card_int = self as i32;
        let next_card_int = next_card as i32;

        if next_card_int - current_card_int == 1 {
            return true;
        } else {
            return false;
        }

    }


}