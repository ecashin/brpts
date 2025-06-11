use leptos::prelude::*;
use leptos::{logging, mount::mount_to_body};
use rand::{rng, seq::SliceRandom};

const RANK_CODE: [(usize, &str); 13] = [
    (2, "2"),
    (3, "3"),
    (4, "4"),
    (5, "5"),
    (6, "6"),
    (7, "7"),
    (8, "8"),
    (9, "9"),
    (10, "\u{2789}"), // circled 10
    (11, "J"),
    (12, "Q"),
    (13, "K"),
    (14, "A"),
];

const SUIT_CODE: [(usize, &str); 4] = [
    (1, "\u{2663}"), // clubs
    (2, "\u{2666}"), // diamonds
    (3, "\u{2665}"), // hearts
    (4, "\u{2660}"), // spades
];

const N_PLAYER: usize = 13;

#[derive(Copy, Clone)]
struct Card {
    rank: usize,
    suit: usize,
}

fn new_deck() -> Vec<Card> {
    let mut cards = vec![];
    for suit in SUIT_CODE.iter().map(|(code, _)| *code) {
        for rank in RANK_CODE.iter().map(|(code, _)| *code) {
            cards.push(Card { suit, rank });
        }
    }
    cards
}

fn new_shuffled_deck() -> Vec<Card> {
    let mut cards = new_deck();
    cards.shuffle(&mut rng());
    cards
}

fn sort_hand(cards: &[Card]) -> Vec<Card> {
    let mut sorted = vec![];
    for suit in suit_codes().iter() {
        for rank in rank_codes().iter().rev() {
            for card in cards {
                if card.rank == *rank && card.suit == *suit {
                    sorted.push(*card);
                }
            }
        }
    }
    sorted
}

fn rank_codes() -> Vec<usize> {
    RANK_CODE.iter().map(|(code, _)| *code).collect()
}

fn rank_repr(rank: usize) -> String {
    RANK_CODE
        .iter()
        .find(|(code, _repr)| *code == rank)
        .unwrap()
        .1
        .to_string()
}

fn suit_codes() -> Vec<usize> {
    SUIT_CODE.iter().map(|(code, _)| *code).collect()
}

fn suit_repr(suit: usize) -> String {
    SUIT_CODE
        .iter()
        .find(|(code, _repr)| *code == suit)
        .unwrap()
        .1
        .to_string()
}

fn hand_repr(sorted: &[Card]) -> String {
    let mut parts = vec![];
    for suit in suit_codes().iter().rev() {
        let suit_cards: Vec<_> = sorted.iter().filter(|card| card.suit == *suit).collect();
        parts.push(format!(
            "{} {}",
            suit_repr(*suit),
            if suit_cards.is_empty() {
                "_".to_string()
            } else {
                suit_cards
                    .iter()
                    .map(|card| rank_repr(card.rank))
                    .collect::<Vec<String>>()
                    .join("")
            }
        ));
    }
    parts.join(" ")
}

fn face_card_points(cards: &[Card]) -> usize {
    cards.iter().map(|card| card.rank.saturating_sub(10)).sum()
}

fn new_hand() -> Vec<Card> {
    let cards: Vec<_> = new_shuffled_deck().into_iter().take(N_PLAYER).collect();
    let sorted = sort_hand(&cards);
    sorted
}

fn long_suit_points(cards: &[Card]) -> usize {
    let mut points = 0;
    for suit in suit_codes() {
        let len: usize = cards.iter().filter(|c| c.suit == suit).map(|_| 1).sum();
        points += len.saturating_sub(4);
    }
    points
}

fn short_suit_points(cards: &[Card]) -> usize {
    let mut points = 0;
    for suit in suit_codes() {
        let len: usize = cards.iter().filter(|c| c.suit == suit).map(|_| 1).sum();
        points += match len {
            2 => 1,
            1 => 2,
            0 => 3,
            _ => 0,
        };
    }
    points
}

#[component]
fn PointsRow(label: String, #[prop(into)] points: usize) -> impl IntoView {
    view! {
        <tr>
            <td>{label}</td>
            <td>{points}</td>
        </tr>
    }
}

#[component]
fn App() -> impl IntoView {
    let (cards, set_cards) = signal(new_hand());
    let (hide, set_hide) = signal(true);
    let fc_pts = Memo::new(move |_| face_card_points(&cards.get()));
    let ls_pts = Memo::new(move |_| long_suit_points(&cards.get()));
    let ss_pts = Memo::new(move |_| short_suit_points(&cards.get()));
    let fcp = move || {
        view! {
            <PointsRow label={"Face Card Points".to_string()} points=fc_pts.get()
            />
        }
    };
    let lsp = move || {
        view! {
            <PointsRow label={"Long Suit Points".to_string()} points=ls_pts.get()
            />
        }
    };
    let ssp = move || {
        view! {
            <PointsRow label={"Short Suit Points".to_string()} points=ss_pts.get()
            />
        }
    };
    let total_points = move || {
        view! {
            <PointsRow label={"Total".to_string()} points={
                fc_pts.get() + ls_pts.get() + ss_pts.get()
            }
            />
        }
    };
    let card_display = move || {
        view! {
            <p>
                {hand_repr(&cards.get())}
            </p>
        }
    };
    let reveal_button = move || {
        let text = if hide.get() { "reveal" } else { "next hand" };
        logging::log!("rendering with hide: {}", hide.get());
        view! {
            <button
                on:click=move |_| {
                    if !hide.get() {
                        set_cards.set(new_hand());
                    }
                    set_hide.set(!hide.get());
                }
            >
                {text}
            </button>
        }
    };

    view! {
        <div  style="float: right">
            <p>
                <a href="https://github.com/ecashin/brpts">source code</a>
            </p>
            <p>
                <a href="https://en.wikipedia.org/wiki/Hand_evaluation">info on hand evaluation</a>
            </p>
        </div>
        {reveal_button}
        {card_display}
        <table class:hidden=move || hide.get()
        >
            {fcp}
            {lsp}
            {ssp}
            {total_points}
        </table>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[cfg(test)]
mod test {
    use crate::rank_repr;

    #[test]
    fn test_ten_repr() {
        assert_eq!(rank_repr(10), "\u{2469}");
    }
}
