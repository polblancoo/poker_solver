use poker::{Card, Evaluator, Rank, Suit};

fn main() {
    let eval = Evaluator::new();

    // HERO: QQ (ej: Qh Qd)
    let h1 = Card::new(Rank::Queen, Suit::Hearts);
    let h2 = Card::new(Rank::Queen, Suit::Diamonds);

    // BOARD: As 8d Qc (Ap 8D Qt) -> As 8d Qc
    let b1 = Card::new(Rank::Ace, Suit::Spades);
    let b2 = Card::new(Rank::Eight, Suit::Diamonds);
    let b3 = Card::new(Rank::Queen, Suit::Clubs);

    let mut hand = vec![h1, h2, b1, b2, b3];
    let score = eval.evaluate(&hand).unwrap();
    println!("Hero Score (Raw): {} - Str: {}", score, score);

    // Comparar con AA (Ah Ad)
    let v1 = Card::new(Rank::Ace, Suit::Hearts);
    let v2 = Card::new(Rank::Ace, Suit::Diamonds);
    let v_hand = vec![v1, v2, b1, b2, b3];
    let v_score = eval.evaluate(&v_hand).unwrap();
    println!("Villain Score (Raw): {} - Str: {}", v_score, v_score);


    if score < v_score {
        println!("Hero wins!");
    } else {
        println!("Villain wins!");
    }
}
