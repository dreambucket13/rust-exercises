/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    
    //parse hands
    let mut best_hands: Vec<Hand> = Vec::new();
    let mut best_hand_strings: Vec<&'a str> = Vec::new();
    let mut parsed_hands:Vec<Hand> = Vec::new();

    for hand_input in 0..hands.len() {

        let raw_hand = hands[hand_input];
        let card_raw_strs = raw_hand.split(" ");

        let mut hand = 
            Hand {
                hand_string_ref: raw_hand,
                cards: Vec::new(),
                value_count: vec!(0; 14),
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
            
            //count the cards
            hand.value_count[(value as usize)] += 1;

        }

        println!("{:?}", hand.value_count);

        //sort cards by value

        println!("{:?}",hand.cards);
        hand.cards.sort_unstable_by_key(|card| card.value);
        //we want the highest value cards first
        hand.cards.reverse();
        println!("{:?}",hand.cards);

        parsed_hands.push(hand);

    }

    //assign ranks

    for hand_to_score in &mut parsed_hands {
        assign_rank(hand_to_score);
    }


    
    //push winning hands (can tie)


    //find highest rank
    for hand in parsed_hands {

        let mut highest_hand_rank = PrimaryRanks::NoRank;

        if hand.primary_rank > highest_hand_rank {
            //update the highest rank
            highest_hand_rank = hand.primary_rank;
            //clear the best hands vec since I found something better
            best_hands.clear();
            //push to best hands again
            best_hands.push(hand);

        } else if hand.primary_rank == highest_hand_rank {
            //if it's equal, could be a tie
            //TODO - handle ties, check secondary rank?
            best_hands.push(hand);
        }

    }

    tiebreak(&mut best_hands);

    for hand in best_hands {
        best_hand_strings.push(hand.hand_string_ref);
    }


    best_hand_strings


}

fn tiebreak(best_hands: &mut Vec<Hand> ){

    let mut highest_secondary_rank = CardValues::NoValue;

    let cards_in_hand = 5;

    //loop through secondary rank by at index 0, remove all that don't match, loop through index 1....

    for card_index in 0..cards_in_hand {

        let mut hands_to_remove: Vec<usize> = Vec::new(); 

        for hand_index in 0..best_hands.len(){
            
            if best_hands[hand_index].secondary_rank[card_index] >= highest_secondary_rank{

                highest_secondary_rank = best_hands[hand_index].secondary_rank[card_index];

            } else {

                hands_to_remove.push(hand_index);

            }
            
        }

        //remove hands if they don't pass tiebreak

        for hand in hands_to_remove{
            best_hands.remove(hand);
        }


    }


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
            //break out of loop once one of the functions returns true
            break;
        }

    }

}


fn detect_straight_flush(hand_to_score: &mut Hand) -> bool{
    //detect straight flush - contains five cards of sequential rank, all of the same suit

    if detect_flush(hand_to_score) && detect_straight(hand_to_score){
        hand_to_score.primary_rank = PrimaryRanks::StraightFlush;

        //have to override secondary rank, flushes are ranked by all cards
        hand_to_score.secondary_rank.clear();
        //push high card value to secondary rank (hand is already sorted)
        hand_to_score.secondary_rank.push(hand_to_score.cards[0].value);
        return true;
    }

    false

}

fn detect_four_of_a_kind(hand_to_score: &mut Hand) -> bool {

    if hand_to_score.value_count.contains(&4){

        hand_to_score.primary_rank = PrimaryRanks::FourOfAKind;

        //set secondary rank to the value of the remaining card (value count == 1)
        let index = hand_to_score.value_count.iter().position(|&x| x == 1).unwrap();
        hand_to_score.secondary_rank.push(usize_to_card_value(index));

        return true;
    }
    
    false
}

fn detect_full_house(hand_to_score: &mut Hand) -> bool {

    if hand_to_score.value_count.contains(&3) && hand_to_score.value_count.contains(&2){

        hand_to_score.primary_rank = PrimaryRanks::FullHouse;

        //set secondary rank to the rank of the triplet, then to the pair
        let mut index = hand_to_score.value_count.iter().position(|&x| x == 3).unwrap();
        hand_to_score.secondary_rank.push(usize_to_card_value(index));

        index = hand_to_score.value_count.iter().position(|&x| x == 2).unwrap();
        hand_to_score.secondary_rank.push(usize_to_card_value(index));

        return true;
    }

    false
    
}

fn detect_flush(hand_to_score: &mut Hand) -> bool {

    //this can be refactored with a suit count check 
    let mut prior_card = &hand_to_score.cards[0];

    for card in &hand_to_score.cards[1..]{

        if card.suit != prior_card.suit {
            return false;
        }

        prior_card = card;
    }

    hand_to_score.primary_rank = PrimaryRanks::Flush;

    //tiebreaker 
    for card in &hand_to_score.cards{
        hand_to_score.secondary_rank.push(card.value);
    }

    return true;

}

fn detect_straight(hand_to_score: &mut Hand) -> bool {

    //special case for ace low straight, simply check for 1 each of A,2,3,4,5
    //kicker is 5 not ace though
    if hand_to_score.value_count[CardValues::AceHigh as usize] == 1 &&
       hand_to_score.value_count[CardValues::TWO as usize] == 1 &&
       hand_to_score.value_count[CardValues::THREE as usize] == 1 &&
       hand_to_score.value_count[CardValues::FOUR as usize] == 1 &&
       hand_to_score.value_count[CardValues::FIVE as usize] == 1
    {
        hand_to_score.primary_rank = PrimaryRanks::Straight;
        hand_to_score.secondary_rank.push(CardValues::FIVE);
        return true;
    }

    //probably can refactor this to look for 5 1 counts in a row in the value_count array
    let mut prior_card = &hand_to_score.cards[0];

    for card in &hand_to_score.cards[1..]{

        if !prior_card.value.is_next(card.value) {
            return false;
        }

        prior_card = card;

    }

    hand_to_score.primary_rank = PrimaryRanks::Straight;
    //kicker is the max value, which is the first element since the hand is sorted
    hand_to_score.secondary_rank.push(hand_to_score.cards[0].value);
    return true;
}

fn detect_three_of_a_kind(hand_to_score: &mut Hand) -> bool{

    if hand_to_score.value_count.contains(&3) {

        hand_to_score.primary_rank = PrimaryRanks::ThreeOfAKind;

        //Each three of a kind is ranked first by the rank of its triplet, 
        //then by the rank of its highest-ranking kicker, and finally by the rank of its lowest-ranking kicker

        let index = hand_to_score.value_count.iter().position(|&x| x == 3).unwrap();
        hand_to_score.secondary_rank.push(usize_to_card_value(index));

        //iterate through value count - if the count is 2, push only once

        //reverse valu count so we push high cards first
        for kicker_index in (0..hand_to_score.value_count.len()).rev() {

            if hand_to_score.value_count[kicker_index] == 2 {
                hand_to_score.secondary_rank.push(usize_to_card_value(kicker_index));
                break; //pushing only once
            }

            if hand_to_score.value_count[kicker_index] == 1 {
                hand_to_score.secondary_rank.push(usize_to_card_value(kicker_index));
            }

        }

        
        return true;
    }

    false
    
}

fn detect_two_pair(hand_to_score: &mut Hand) -> bool{

    //look for 2 values of two

    let mut pair_count = 0;

    let mut two_pair_indicies:Vec<usize> = Vec::new();
    let mut kicker_index: usize = 0;

    //reversing the order of value count
    for index in (0..hand_to_score.value_count.len()).rev() {

        if hand_to_score.value_count[index] == 2 {
            pair_count += 1;
            two_pair_indicies.push(index);
        }

        if hand_to_score.value_count[index] == 1 {
            kicker_index = index;
        }

    }

    if pair_count == 2 {
        hand_to_score.primary_rank = PrimaryRanks::TwoPair;

        for secondary_rank_index in 0..two_pair_indicies.len(){
            hand_to_score.secondary_rank.push(usize_to_card_value(secondary_rank_index));
        }

        //now push the single card
        hand_to_score.secondary_rank.push(usize_to_card_value(kicker_index));
    }

    false
    
}

fn detect_one_pair(hand_to_score: &mut Hand) -> bool{

    //for now, essentially the same as 2 pair.

    let mut pair_count = 0;

    let mut one_pair_index: usize = 0;
    let mut kicker_indecies: Vec<usize> = Vec::new();


    for index in 0..hand_to_score.value_count.len() {

        if hand_to_score.value_count[index] == 2 {
            pair_count += 1;
            one_pair_index = index;
        }

        if hand_to_score.value_count[index] == 1 {
            kicker_indecies.push(index);
        }

    }

    if pair_count == 1 {

        hand_to_score.primary_rank = PrimaryRanks::OnePair;

        //push the rank of the pair single card as first tiebreak
        hand_to_score.secondary_rank.push(usize_to_card_value(one_pair_index));


        //push remaining 3 cards in reverse order (the value count array starts at value 0)
        for secondary_rank_index in (0..kicker_indecies.len()).rev(){
            hand_to_score.secondary_rank.push(usize_to_card_value(secondary_rank_index));
        }


    }

    false
}

fn detect_high_card(hand_to_score: &mut Hand) -> bool{
    
    hand_to_score.primary_rank = PrimaryRanks::HighCard;


    //cards are already all sorted in decending order 
    for card in &hand_to_score.cards{
        hand_to_score.secondary_rank.push(card.value);
    }

    true
}


fn str_to_card_value(str: &str) -> CardValues {

    match str {

        _ if str.starts_with("2") => return CardValues::TWO,
        _ if str.starts_with("3") => return CardValues::THREE,
        _ if str.starts_with("4") => return CardValues::FOUR,
        _ if str.starts_with("5") => return CardValues::FIVE,
        _ if str.starts_with("6") => return CardValues::SIX,
        _ if str.starts_with("7") => return CardValues::SEVEN,
        _ if str.starts_with("8") => return CardValues::EIGHT,
        _ if str.starts_with("9") => return CardValues::NINE,
        _ if str.starts_with("J") => return CardValues::JACK,
        _ if str.starts_with("Q") => return CardValues::QUEEN,
        _ if str.starts_with("K") => return CardValues::KING,
        _ if str.starts_with("A") => return CardValues::AceHigh,
        _ => return CardValues::NoValue,

    };

}

fn usize_to_card_value(index: usize) -> CardValues {

    match index {

        1 => return CardValues::AceLow,
        2 => return CardValues::TWO,
        3 => return CardValues::THREE,
        4 => return CardValues::FOUR,
        5 => return CardValues::FIVE,
        6 => return CardValues::SIX,
        7 => return CardValues::SEVEN,
        8 => return CardValues::EIGHT,
        9 => return CardValues::NINE,
        10 => return CardValues::JACK,
        11 => return CardValues::QUEEN,
        12 => return CardValues::KING,
        13 => return CardValues::AceHigh,
        _ => return CardValues::NoValue,

    };

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

#[derive(Debug, Clone)]
struct Hand<'a> {

    hand_string_ref: &'a str,
    cards: Vec<Card>,
    value_count: Vec<i32>,
    suit_count: Vec<i32>,
    primary_rank: PrimaryRanks,
    secondary_rank: Vec<CardValues>

}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Card {

    suit: Suits,
    value: CardValues
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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