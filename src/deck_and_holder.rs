use crate::{
    bird_card::BirdCard,
    error::{WingError, WingResult},
};

#[derive(Default, Debug, Clone)]
pub(crate) struct DeckAndHolder {
    _bird_deck: Vec<BirdCard>,
    _face_up_display: Vec<BirdCard>,
}

impl DeckAndHolder {
    pub fn new(deck: Vec<BirdCard>) -> Self {
        Self {
            _bird_deck: deck,
            _face_up_display: vec![],
        }
    }

    pub fn reset_display(&mut self) {
        self._face_up_display.clear();
        self._face_up_display = self.draw_cards_from_deck(3);
    }

    pub fn refill_display(&mut self) {
        if self._face_up_display.len() < 3 {
            let mut new_cards = self.draw_cards_from_deck(3 - self._face_up_display.len());
            self._face_up_display.append(&mut new_cards);
        }
    }

    pub fn get_display_cards(&self) -> &Vec<BirdCard> {
        &self._face_up_display
    }

    pub fn draw_cards_from_deck(&mut self, num_cards: usize) -> Vec<BirdCard> {
        self._bird_deck.split_off(self._bird_deck.len() - num_cards)
    }

    pub fn draw_card(&mut self, source_idx: u8) -> WingResult<BirdCard> {
        let source_idx = source_idx as usize;

        let result = match source_idx {
            0 => self._bird_deck.pop(),
            1..=3 => {
                let display_idx = source_idx - 1;
                if display_idx >= self._face_up_display.len() {
                    Some(self._face_up_display.remove(display_idx))
                } else {
                    None
                }
            }
            _ => return Err(WingError::InvalidAction),
        };

        result.ok_or(WingError::InvalidAction)
    }

    pub fn num_actions(&self) -> usize {
        1 + self._face_up_display.len()
    }
}
