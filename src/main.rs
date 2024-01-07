use darth_tools::{
    card::{Number, Suit, Card},
    poker::PokerTrait,
    DarthTools,
};

fn main() {
    let cards = [
        DarthTools::new_poker_card(Suit::Hearts, Number::Ten),
        DarthTools::new_poker_card(Suit::Hearts, Number::Jack),
        DarthTools::new_poker_card(Suit::Hearts, Number::Queen),
        DarthTools::new_poker_card(Suit::Hearts, Number::King),
        DarthTools::new_poker_card(Suit::Hearts, Number::Ace),
        DarthTools::new_poker_card(Suit::Clubs, Number::Two),
        DarthTools::new_poker_card(Suit::Diamonds, Number::Three),
    ];
    cards.iter().for_each(|c| {
        let json =  c.to_json();
        let parse = Card::from_json(json).unwrap();
        println!("{:#?}", parse)
    })
}