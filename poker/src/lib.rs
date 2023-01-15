/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {:?}, which hand wins?", hands)
}

fn char_to_card_value(char: &str) -> CardValues {

    let value: CardValues;

    match char {

        _ if char.starts_with("2") => value = CardValues::TWO,
        _ if char.starts_with("3") => value = CardValues::THREE,
        _ if char.starts_with("4") => value = CardValues::FOUR,
        _ if char.starts_with("5") => value = CardValues::FIVE,
        _ if char.starts_with("6") => value = CardValues::SIX,
        _ if char.starts_with("7") => value = CardValues::SEVEN,
        _ if char.starts_with("8") => value = CardValues::EIGHT,
        _ if char.starts_with("9") => value = CardValues::NINE,
        _ if char.starts_with("J") => value = CardValues::JACK,
        _ if char.starts_with("Q") => value = CardValues::QUEEN,
        _ if char.starts_with("K") => value = CardValues::KING,
        _ if char.starts_with("A") => value = CardValues::AceHigh,
        _ => value = CardValues::NoValue,

    };

    value

}

struct HandRank<'a> {

    hand: &'a str,
    primary_rank: PrimaryRanks,
    secondary_rank: Vec<CardValues>

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

enum Suits{
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES
}

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