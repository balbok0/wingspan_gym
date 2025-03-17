use strum::IntoEnumIterator;

use super::BirdCard;
use crate::{action::Action, bird_card::BirdCardColor, bird_card_callback::BirdCardCallback, error::{WingError, WingResult}, food::FoodIndex, habitat::Habitat, nest::NestType, wingspan_env::WingspanEnv};

#[derive(Debug)]
#[derive(Default)]
pub struct ActivateResult {
  pub immediate_actions: Vec<Action>,
  pub end_of_turn_actions: Vec<Action>
}




impl BirdCard {
  pub fn activate(&self, env: &mut WingspanEnv, habitat: &Habitat, bird_idx: usize,) -> WingResult<ActivateResult> {
    match self {
      Self::BlackTern
        | Self::ClarksGrebe
        | Self::ForstersTern => {
        // draw 1 [card]. if you do, discard 1 [card] from your hand at the end of your turn.
        Ok(ActivateResult {
          immediate_actions: vec![Action::GetBirdCard],
          end_of_turn_actions: vec![Action::DiscardBirdCard],
        })
      },
      Self::RedWattlebird => {
        // gain 1 [nectar] from the supply for each bird with a wingspan less than 49cm in your [forest].
        todo!()
      },
      Self::Pukeko => {
        // lay 1 [egg] on an adjacent bird.
        todo!()
      },
      Self::ParrotCrossbill => {
        // remove any 1 [die] from the birdfeeder, then gain 1 [seed] from the supply.
        todo!()
      },
      Self::VerditerFlycatcher => {
        // if you have gained a [invertebrate] from the birdfeeder on this turn, gain 1 [fruit] from the supply.
        todo!()
      },
      Self::GoldenHeadedCisticola => {
        // play another bird in your [grassland]. pay its normal cost with a 1 [egg] discount.
        todo!()
      },
      Self::CommonGoldeneye => {
        // lay 1 [egg] on this bird for each other bird with a [cavity] nest that you have.
        todo!()
      },
      Self::PhilippineEagle => {
        // roll all 5 [die]. you may reroll any number of [die], up to 2 times. if at least 3 [rodent] are showing when you stop, draw 2 bonus cards and keep 1. reset the birdfeeder.
        todo!()
      },
      Self::ShortToedTreecreeper => {
        // discard 1 [egg] from any bird. if you do, play another bird in your [forest]. pay its normal food and egg cost.
        todo!()
      },
      Self::AzureTit => {
        // gain 1 [invertebrate], [seed], or [fruit] from the supply.
        todo!()
      },
      Self::BrewersBlackbird
        | Self::Bushtit
        | Self::CommonGrackle
        | Self::Dickcissel
        | Self::RedWingedBlackbird
        | Self::YellowHeadedBlackbird => {
        // tuck 1 [card] from your hand behind this bird. if you do, you may also lay 1 [egg] on this bird.
        Ok(ActivateResult{
          immediate_actions: vec![Action::DoThen(
            Box::new(Action::TuckBirdCard(*habitat, bird_idx)),
            Box::new(Action::GetEggAtLoc(*habitat, bird_idx, 1))
          )],
          ..Default::default()
        })
      },
      Self::AmericanRedstart
        | Self::WhiteBackedWoodpecker => {
        // gain 1 [die] from the birdfeeder.
        Ok(ActivateResult{
          immediate_actions: vec![Action::GetFood],
          ..Default::default()
        })
      },
      Self::GreatIndianBustard => {
        // score 1 of your bonus cards now by caching 1 [seed] from the supply on this bird for each point. also score it normally at game end.
        todo!()
      },
      Self::CommonChaffinch
        | Self::CommonChiffchaff => {
        // choose 1-5 birds in this habitat. tuck 1 [card] from your hand behind each.
        todo!()
      },
      Self::LaughingKookaburra => {
        // reset the birdfeeder. if you do, gain 1 [invertebrate], [fish], or [rodent], if there is one.
        todo!()
      },
      Self::Dunnock => {
        // choose 1 other player. for each action cube on their [grassland], lay 1 [egg] on this bird.
        todo!()
      },
      Self::FireFrontedSerin => {
        // lay 1 [egg] on this bird for each bird to its left in this row.
        todo!()
      },
      Self::LesserFrigatebird => {
        // all players may discard 1 [egg] from a [wetland] bird. each player that discards an [egg] gains 1 [wild] from the supply.
        todo!()
      },
      Self::CaliforniaQuail
        | Self::MourningDove
        | Self::NorthernBobwhite
        | Self::ScaledQuail => {
        // lay 1 [egg] on this bird.
        env.current_player_mut().get_mat_mut().get_row_mut(habitat).place_egg_at_exact_bird_idx(bird_idx)?;
        Ok(Default::default())
      },
      Self::CommonIora => {
        // lay 1 [egg] on another bird in this column.
        todo!()
      },
      Self::BlackHeadedGull => {
        // steal 1 [wild] from another player's supply and add it to your own supply. they gain 1 [die] from the birdfeeder.
        todo!()
      },
      Self::WhiteThroatedKingfisher => {
        // choose any 1 [die]. roll it up to 3 times. each time, if you roll a [invertebrate], [fish], or [rodent], cache 1 here. if not, stop and return all food cached here this turn.
        todo!()
      },
      Self::BlueRockThrush => {
        // if this bird has no birds to the right of it, you may move it (only the card) to the player mat of the player on your left (you choose its habitat). if you do, draw 3 [card].
        todo!()
      },
      Self::ManedDuck => {
        // tuck up to 3 [card] from your hand behind this bird. if you tuck at least 1 [card], gain 1 [seed] from the supply.
        todo!()
      },
      Self::AustralasianPipit => {
        // tuck 1 [card] from the deck behind each bird in your [grassland], including this one.
        todo!()
      },
      Self::CountRaggisBirdofParadise => {
        // choose 1 other player. you both gain 1 [fruit] from the supply.
        todo!()
      },
      Self::FranklinsGull
        | Self::Killdeer => {
        // discard 1 [egg] to draw 2 [card].
        Ok(ActivateResult {
          immediate_actions: vec![Action::DoThen(Box::new(Action::DiscardEgg), Box::new(Action::MultipleActions(vec![Action::GetBirdCard, Action::GetBirdCard])))],
          ..Default::default()
        })
      },
      Self::ForestOwlet => {
        // choose any 2 [die]. roll them up to 3 times. each time, if you roll at least 1 [invertebrate] or [rodent], cache 1 here. if not, stop and return all food cached here this turn.
        todo!()
      },
      Self::BlackRedstart
        | Self::LesserWhitethroat => {
        // choose a habitat with no [egg]. lay 1 [egg] on each bird in that habitat.
        todo!()
      },
      Self::CommonTeal => {
        // for every 3 [egg] in your [wetland], draw 1 [card] from the deck. you may tuck up to 2 [card] from your hand behind this bird.
        todo!()
      },
      Self::Cockatiel => {
        // discard 1 [seed] to choose a [card] from the tray and tuck it behind this bird.
        todo!()
      },
      Self::MoltonisWarbler
        | Self::WhiteWagtail
        | Self::Yellowhammer => {
        // if you used all 4 types of actions this round, play another bird. pay its normal food and egg cost.
        todo!()
      },
      Self::BrahminyKite => {
        // choose any 3 [die]. roll them up to 3 times. each time, if you roll at least 1 [fish] or [rodent], cache 1 here. if not, stop and return all food cached here this turn.
        todo!()
      },
      Self::CommonGreenMagpie => {
        // gain 1 [invertebrate] or [rodent] from the birdfeeder, if there is one. you may cache it on this bird.
        todo!()
      },
      Self::Ibisbill => {
        // all players draw 1 [card] from the deck and gain 1 [invertebrate] from the supply. you draw 1 additional [card] from the deck.
        todo!()
      },
      Self::LittleBustard
        | Self::SnowyOwl => {
        // draw 1 new bonus card. then gain 1 [card] or lay 1 [egg] on any bird.
        todo!()
      },
      Self::EuropeanTurtleDove => {
        // draw 1 new bonus card. then gain 1 [die] from the birdfeeder, lay 1 [egg] on any bird, or draw 1 [card].
        todo!()
      },
      Self::GreenPheasant => {
        // all players lay 1 [egg].
        todo!()
      },
      Self::Brolga => {
        // choose 1 other player. they lay 1 [egg]; you draw 2 [card].
        todo!()
      },
      Self::Kakapo => {
        // draw 4 bonus cards, keep 1, and discard the other 3.
        todo!()
      },
      Self::Twite => {
        // draw 2 [card] from the deck and add them to your hand. then, tuck any 2 [card] from your hand behind this bird.
        todo!()
      },
      Self::GreyHeadedMannikin => {
        // play a bird. pay its normal food cost, but ignore 1 [egg] in its egg cost. if it has a "when played" or "game end" power, you may use it.
        todo!()
      },
      Self::GreaterAdjutant => {
        // copy one bonus card of the player on your left, as if it were your own (score it based on your own birds).
        todo!()
      },
      Self::RedBackedShrike => {
        // steal 1 [invertebrate] from another player's supply and cache it on this bird. they gain 1 [die] from the birdfeeder.
        todo!()
      },
      Self::WelcomeSwallow => {
        // tuck 1 [card] from the deck behind each bird in this habitat, including this bird.
        todo!()
      },
      Self::Korimako => {
        // discard any number of [rodent] to gain that many [nectar] from the supply.
        todo!()
      },
      Self::MuskDuck => {
        // draw 1 face-up [card] from the tray with a [ground] or [star] nest. you may reset or refill the tray before doing so.
        todo!()
      },
      Self::RedWingedParrot => {
        // give 1 [nectar] from your supply to another player. if you do, lay 2 [egg] on this bird or gain 2 [die] from the birdfeeder.
        todo!()
      },
      Self::CrestedIbis
        | Self::SpoonBilledSandpiper => {
        // draw 2 new bonus cards and keep 1. other players may discard any 2 resources ([wild], [egg], or [card]) to do the same.
        todo!()
      },
      Self::Brant => {
        // draw the 3 face-up [card] in the bird tray.
        for bird_card in env._bird_deck.get_display_cards().clone() {
          env.current_player_mut().add_bird_card(bird_card);
        }
        env._bird_deck.reset_display();
        Ok(Default::default())
      },
      Self::MaskedLapwing => {
        // reset the birdfeeder, then, for each type of food in the birdfeeder, gain 1 of that type.
        todo!()
      },
      Self::EasternRosella => {
        // all players gain 1 [nectar] from the supply. you also gain 1 [seed] from the supply.
        todo!()
      },
      Self::Hawfinch => {
        // reset the birdfeeder. if you do, gain 1 [seed] from the birdfeeder after resetting.
        todo!()
      },
      Self::BlackNeckedStilt
        | Self::CarolinaWren => {
        // draw 2 [card].
        Ok(ActivateResult{
          immediate_actions: vec![Action::GetBirdCard, Action::GetBirdCard],
          ..Default::default()
        })
      },
      Self::CrimsonChat => {
        // discard 1 [wild] to tuck 1 [card] from the deck behind this bird.
        todo!()
      },
      Self::Mallard => {
        // draw 1 [card].
        let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];
        env.current_player_mut().add_bird_card(bird_card);
        Ok(Default::default())
      },
      Self::Silvereye => {
        // all players gain 1 [nectar] from the supply.
        todo!()
      },
      Self::EleonorasFalcon => {
        // roll all dice not in the birdfeeder. if any are [rodent], place 1 [egg] on this card.
        todo!()
      },
      Self::SriLankaFrogmouth => {
        // roll any 1 [die]. if you roll a [invertebrate], cache 1 [invertebrate] from the supply on this bird. all players may discard 1 [card] from their hand to gain 1 [invertebrate] from the supply.
        todo!()
      },
      Self::LittleOwl => {
        // steal 1 [rodent] from another player's supply and cache it on this bird. they gain 1 [die] from the birdfeeder.
        todo!()
      },
      Self::NorthIslandBrownKiwi => {
        // discard a bonus card. if you do, draw 4 bonus cards, keep 2, and discard the other 2.
        todo!()
      },
      Self::TawnyFrogmouth => {
        // reset the birdfeeder. cache 1 [invertebrate] or [rodent] from the birdfeeder (if available) on this bird.
        todo!()
      },
      Self::BlackShoulderedKite => {
        // reset the birdfeeder and gain 1 [rodent], if there is one. you may give it to another player; if you do, lay up to 3 [egg] on this bird.
        todo!()
      },
      Self::AustralianMagpie => {
        // discard 1 [egg] from each bird in this row and column that has an [egg] on it, excluding this bird. for each discarded [egg], cache 2 [seed] from the supply on this bird.
        todo!()
      },
      Self::TheklasLark => {
        // discard 1 [seed] from your supply. if you do, lay 2 [egg] on this bird.
        todo!()
      },
      Self::CommonYellowthroat
        | Self::PiedBilledGrebe
        | Self::RedBreastedMerganser
        | Self::RuddyDuck
        | Self::WoodDuck => {
        // draw 2 [card]. if you do, discard 1 [card] from your hand at the end of your turn.
        Ok(ActivateResult {
          immediate_actions: vec![Action::GetBirdCard, Action::GetBirdCard],
          end_of_turn_actions: vec![Action::DiscardBirdCard],
        })
      },
      Self::DarkEyedJunco
        | Self::PineSiskin
        | Self::VauxsSwift
        | Self::CedarWaxwing
        | Self::PygmyNuthatch
        => {
        // tuck 1 [card] from your hand behind this bird. if you do, gain 1 [FOOD TYPE] from the supply.
        let food_choices: Box<[FoodIndex]> = match self {
          Self::DarkEyedJunco
            | Self::PineSiskin => Box::new([FoodIndex::Seed]),
          Self::VauxsSwift => Box::new([FoodIndex::Invertebrate]),
          Self::CedarWaxwing => Box::new([FoodIndex::Fruit]),
          Self::PygmyNuthatch => Box::new([FoodIndex::Invertebrate, FoodIndex::Seed]),
          _ => return Err(WingError::InvalidBird(format!("Got {self:?} in unexpected arm")))
        };
        Ok(ActivateResult {
          immediate_actions: vec![Action::DoThen(Box::new(Action::TuckBirdCard(*habitat, bird_idx)), Box::new(Action::GetFoodFromSupplyChoice(food_choices)))],
          ..Default::default()
        })
      },
      Self::GouldsFinch => {
        // play a bird. pay its normal food and egg cost. if it has a "when played" or "game end" power, you may use it.
        todo!()
      },
      Self::RufousNightHeron => {
        // look at a [card] from the deck. if it can live in [wetland], tuck it behind this bird. if not, discard it.
        let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];

        let is_valid = bird_card.habitats().contains(&Habitat::Wetland);

        if is_valid {
          env.current_player_mut().get_mat_mut().get_row_mut(habitat).tuck_card(bird_idx);
        }
        Ok(Default::default())
      },
      Self::RedVentedBulbul => {
        // if you have at least 1 [fruit] in your supply, lay 1 [egg] on this bird.
        todo!()
      },
      Self::DesertFinch => {
        // lay 1 [egg] on this bird for each other bird in your [grassland].
        todo!()
      },
      Self::OliveBackedSunbird => {
        // each player may roll any 1 [die] and gain that food from the supply.
        todo!()
      },
      Self::AustralianReedWarbler => {
        // play another bird in your wetland. pay its normal cost with a 1 [egg] discount.
        todo!()
      },
      Self::HermitThrush => {
        // player(s) with the fewest birds in their [forest] gain 1 [die] from birdfeeder.
        let min_birds_num = (0..env.config().num_players)
          .map(|idx| {
            env.get_player(idx).get_mat().get_row(&Habitat::Forest).get_birds().len()
          })
          .min()
          .unwrap();

        let cur_player_idx = env.current_player_idx();

        let mut actions = Vec::new();

        for player_idx in 0..min_birds_num {
          if min_birds_num < env.get_player(player_idx).get_mat().get_row(&Habitat::Wetland).get_birds().len() {
            continue;
          }

          actions.push(Action::GetFood);
          actions.push(Action::ChangePlayer(player_idx));
        }
        actions.insert(0, Action::ChangePlayer(cur_player_idx));

        Ok(ActivateResult {
          immediate_actions: actions,
          ..Default::default()
        })
      },
      Self::BlackDrongo => {
        // discard any number of [card] from the tray then refill it. if at least one of the discarded birds is a [grassland] bird, lay 1 [egg] on this bird.
        todo!()
      },
      Self::RedLeggedPartridge => {
        // lay 1 [egg] on each bird in this column, including this one.
        todo!()
      },
      Self::WhiteBreastedWoodswallow => {
        // lay 1 [egg] on each bird in your [grassland], including this one.
        todo!()
      },
      Self::WedgeTailedEagle => {
        // look at a [card] from the deck. if its wingspan is over 65cm, tuck it behind this bird and cache 1 [rodent] from the supply on this bird. if not, discard it.
        let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];

        let is_valid = match bird_card.wingspan() {
          Some(x) => x > 65,
          None => true
        };

        if is_valid {
          let row = env.current_player_mut().get_mat_mut().get_row_mut(habitat);
          row.tuck_card(bird_idx);
          row.cache_food(bird_idx, FoodIndex::Rodent);
        }
        Ok(Default::default())
      },
      Self::GreatSpottedWoodpecker => {
        // gain 1 [invertebrate] or [seed] from the birdfeeder, if there is one.
        todo!()
      },
      Self::RufousBandedHoneyeater => {
        // discard 1 [invertebrate]. if you do, gain 1 [nectar] from the supply.
        todo!()
      },
      Self::CommonLittleBittern => {
        // gain 1 face-up [card] that can live in [grassland].
        todo!()
      },
      Self::DesertWheatear => {
        // for each bird in your [grassland] with an [egg] on it, roll any 1 [die]. choose 1 type of food you rolled, and gain 1 of that food from the supply.
        todo!()
      },
      Self::StubbleQuail => {
        // discard up to 6 [wild]. lay 1 [egg] on this bird for each discarded food.
        todo!()
      },
      Self::MuteSwan => {
        // choose 1-3 birds in your [wetland]. tuck 1 [card] from your hand behind each. if you tuck at least 1 card, draw 1 [card].
        todo!()
      },
      Self::AustralasianShoveler => {
        // choose 1 other player. you both draw 1 [card] from the deck.
        todo!()
      },
      Self::AustralianIbis => {
        // shuffle the discard pile, then draw 2 [card] from it. choose 1 and tuck it behind this bird or add it to your hand. discard the other.
        todo!()
      },
      Self::PacificBlackDuck => {
        // for every 2 [egg] in your [wetland], lay 1 [egg] on this bird.
        todo!()
      },
      Self::LittleGrebe => {
        // for each bird in this column with an [egg] on it, draw 1 [card]. keep 1 and discard the rest.
        todo!()
      },
      Self::PurpleHeron => {
        // choose any 2 [die]. roll them up to 3 times. each time, if you roll at least 1 [invertebrate] or [fish], cache 1 here. if not, stop and return all food cached here this turn.
        todo!()
      },
      Self::BlythsHornbill => {
        // discard all [egg] from 1 of your birds with a [cavity] nest. tuck twice that many [card] from the deck behind this bird.
        todo!()
      },
      Self::RosyStarling => {
        // tuck up to 3 [card] from your hand behind this bird. if you tuck at least 1 [card], gain 1 [invertebrate] from the supply.
        todo!()
      },
      Self::GreenPygmyGoose
        | Self::PinkEaredDuck => {
        // draw 2 [card] from the deck. keep 1 and give the other to another player.
        todo!()
      },
      Self::BonellisEagle
        | Self::EasternImperialEagle
        | Self::EurasianSparrowhawk
        | Self::NorthernGoshawk => {
        // for each [rodent] in this bird's cost, you may pay 1 [card] from your hand instead. if you do, tuck the paid [card] behind this card.
        todo!()
      },
      Self::Bluethroat
        | Self::CommonNightingale => {
        // choose a food type. all players gain 1 of that food from the supply.
        todo!()
      },
      Self::BlackNapedOriole => {
        // if you used all 4 actions this round, gain 1 [wild] from the supply, lay 1 [egg] on any bird, and draw 1 [card] from the deck.
        todo!()
      },
      Self::RoseRingedParakeet => {
        // copy a "when played" (white) ability on 1 of your neighbors' birds.
        todo!()
      },
      Self::Bullfinch => {
        // reset the birdfeeder. if you do, gain 1 [seed] or 1 [fruit] from the birdfeeder after resetting.
        todo!()
      },
      Self::RegentBowerbird => {
        // choose 1 other player. you both gain 1 [invertebrate] from the supply.
        todo!()
      },
      Self::HoodedMerganser => {
        // repeat 1 [predator] power in this habitat.
        let num_choices  = env.current_player().get_mat().get_row(habitat).get_birds()
          .iter()
          .filter(|bc| bc.is_predator())
          .count();

        if num_choices == 0 {
          return Ok(Default::default());
        }

        Ok(ActivateResult {
          immediate_actions: vec![
            Action::ChooseThenAction(num_choices as u8, *self, *habitat, bird_idx)
          ],
          ..Default::default()
        })
      },
      Self::CommonBuzzard
        | Self::EurasianHobby
        | Self::MontagusHarrier
        | Self::RedKite => {
        // instead of paying any costs, you may play this bird on top of another bird on your player mat. discard any eggs and food from that bird. it becomes a tucked card.
        todo!()
      },
      Self::BrownShrike => {
        // all players may cache 1 [invertebrate] from their personal supply on a bird in their [grassland].
        todo!()
      },
      Self::BairdsSparrow
        | Self::CassinsSparrow
        | Self::ChippingSparrow
        | Self::GrasshopperSparrow => {
        // lay 1 [egg] on any bird.
        Ok(ActivateResult {
          immediate_actions: vec![Action::GetEgg],
          ..Default::default()
        })
      },
      Self::DownyWoodpecker
        | Self::RedEyedVireo
        | Self::RubyCrownedKinglet
        | Self::TuftedTitmouse
        | Self::EasternBluebird
        | Self::MountainBluebird
        | Self::SavannahSparrow
        | Self::GreatBlueHeron
        | Self::GreatEgret
        | Self::HouseWren
        => {
        // play an additional bird in your [x]. pay its normal cost.
        let habitat = match self {
          Self::DownyWoodpecker
            | Self::RedEyedVireo
            | Self::RubyCrownedKinglet
            | Self::TuftedTitmouse => Habitat::Forest,
          Self::EasternBluebird
            | Self::MountainBluebird
            | Self::SavannahSparrow => Habitat::Grassland,
          Self::GreatBlueHeron
            | Self::GreatEgret => Habitat::Wetland,
          Self::HouseWren => *habitat,
          _ => panic!("Got bird {self:?} in action that is not related.")
        };
        Ok(ActivateResult {
          immediate_actions: vec![Action::PlayBirdHabitat(habitat)],
          ..Default::default()
        })
      },
      Self::Wrybill => {
        // look through all discarded bonus cards. keep 1 of them.
        todo!()
      },
      Self::EuropeanHoneyBuzzard => {
        // reset the birdfeeder. if you do, gain all [invertebrate] in the birdfeeder after resetting.
        todo!()
      },
      Self::Kereru => {
        // if the player to your left has a [nectar] in their personal supply, gain 1 [nectar] from the general supply.
        todo!()
      },
      Self::PlumbeousRedstart => {
        // draw 1 [card] from the deck and add it to your hand. all other players draw 1 [card] from the deck and add it to their hand if the bird has a [invertebrate] or [seed] in its food cost.
        todo!()
      },
      Self::Galah => {
        // choose 1 other player. they reset the birdfeeder and gain a [seed], if there is one. you tuck 2 [card] from the deck behind this bird.
        todo!()
      },
      Self::PlainsWanderer => {
        // draw 1 bonus card for each bird in your [grassland]. keep 1 and discard the rest.
        todo!()
      },
      Self::YellowBittern => {
        // draw the [card] in the middle slot of the bird tray.
        todo!()
      },
      Self::AshThroatedFlycatcher
        | Self::SaysPhoebe
        | Self::RedBackedFairywren
        | Self::IncaDove
        | Self::Malleefowl
        | Self::OrangeFootedScrubfowl
        | Self::LittlePiedCormorant
        | Self::Bobolink
        => {
        // lay 1 [egg] on each of your birds with a x nest.

        let goal_nest_type = match self {
          Self::AshThroatedFlycatcher => NestType::Cavity,
          Self::SaysPhoebe => NestType::Bowl,
          Self::RedBackedFairywren => NestType::Wild,
          Self::IncaDove | Self::LittlePiedCormorant => NestType::Platform,
          Self::Malleefowl | Self::OrangeFootedScrubfowl | Self::Bobolink => NestType::Ground,
          _ => panic!("Encountered {self:?} in activate branch which it does not belong to (lay 1 egg on each of your birds with a [x] nest).")
        };

        let idxs = env.current_player().get_mat().get_birds_with_nest_type(goal_nest_type);
        for (row_idx, bird_idx) in idxs {
          let _ = env.current_player_mut().get_mat_mut().get_row_mut(&row_idx).place_egg_at_exact_bird_idx(bird_idx);
        }

        Ok(Default::default())
      },
      Self::WhiteHeadedDuck => {
        // draw 3 new bonus cards and keep 1.
        env.draw_bonus_cards(3);

        Ok(ActivateResult {
          immediate_actions: vec![Action::DiscardBonusCard, Action::DiscardBonusCard],
          ..Default::default()
        })
      },
      Self::CarolinaChickadee
        | Self::JuniperTitmouse
        | Self::MountainChickadee
        | Self::RedBreastedNuthatch
        | Self::WhiteBreastedNuthatch => {
        // cache 1 [seed] from the supply on this bird.

        env.current_player_mut().get_mat_mut().get_row_mut(habitat).cache_food(bird_idx, FoodIndex::Seed);
        Ok(Default::default())
      },
      Self::SulphurCrestedCockatoo => {
        // tuck 1 [card] from your hand behind this bird. if you do, all players gain 1 [nectar] from the supply.
        todo!()
      },
      Self::AmericanAvocet
        | Self::BeltedKingfisher
        | Self::BronzedCowbird
        | Self::BrownHeadedCowbird
        | Self::YellowBilledCuckoo
        | Self::SpangledDrongo
        | Self::LoggerheadShrike
        | Self::CommonCuckoo
        | Self::BlackVulture
        | Self::BlackBilledMagpie
        | Self::TurkeyVulture
        | Self::EuropeanGoldfinch
        | Self::PheasantCoucal
        | Self::HorsfieldsBronzeCuckoo
        | Self::VioletCuckoo
        | Self::SnowBunting
        | Self::BarrowsGoldeneye
        | Self::EurasianGoldenOriole
        | Self::AsianKoel
        | Self::EurasianTreeSparrow
        | Self::EasternKingbird
        | Self::HornedLark
        | Self::SacredKingfisher
        | Self::AustralianOwletNightjar
        => {
        // when another player takes X do Y
        env.push_callback(BirdCardCallback {
            card: *self,
            habitat: *habitat,
            card_idx: bird_idx,
            card_player_idx: env.current_player_idx(),
        });
        Ok(Default::default())
      },
      Self::ScalyBreastedMunia => {
        // gain 1 [seed] from the supply or tuck 1 [card] from the deck behind this bird.
        todo!()
      },
      Self::AnnasHummingbird
        | Self::RubyThroatedHummingbird => {
        // each player gains 1 [die] from the birdfeeder, starting with the player of your choice.
        Ok(ActivateResult {
          immediate_actions: vec![
            Action::ChooseThenAction(env.config().num_players as u8, *self, *habitat, bird_idx)
          ],
          ..Default::default()
        })
      },
      Self::AmericanCoot
        | Self::AmericanRobin
        | Self::BarnSwallow
        | Self::HouseFinch
        | Self::PurpleMartin
        | Self::RingBilledGull
        | Self::TreeSwallow
        | Self::VioletGreenSwallow
        | Self::YellowRumpedWarbler => {
        // tuck 1 [card] from your hand behind this bird. if you do, draw 1 [card].
        Ok(ActivateResult {
          immediate_actions: vec![Action::DoThen(
            Box::new(Action::TuckBirdCard(*habitat, bird_idx)),
            Box::new(Action::GetBirdCard),
          )],
          ..Default::default()
        })
      },
      Self::AustralianZebraFinch => {
        // if the player to your right has a [seed] in their personal supply, tuck a [card] from the deck behind this bird.
        let player_to_right = env.get_player(env.current_player_idx() - 1);
        if player_to_right.get_foods()[FoodIndex::Seed as usize] > 0 {
          env.current_player_mut().get_mat_mut().get_row_mut(habitat).tuck_card(bird_idx);
        }

        Ok(Default::default())
      },
      Self::IndianPeafowl => {
        // all players draw 2 [card] from the deck. you draw 1 additional [card].
        let cur_player_idx = env.current_player_idx();
        for player_idx in 0..env.config().num_players {
          env.set_current_player(player_idx);
          let mut cards = env._bird_deck.draw_cards_from_deck(2);
          env.current_player_mut().append_bird_cards(&mut cards);
        }

        env.set_current_player(cur_player_idx);
        let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];
        env.current_player_mut().add_bird_card(bird_card);

        Ok(Default::default())
      },
      Self::RockPigeon => {
        // all players lay 1 [egg]. you lay 1 additional [egg].
        todo!()
      },
      Self::CrestedLark => {
        // discard 1 [seed]. if you do, lay 1 [egg] on this bird.
        Ok(ActivateResult {
          immediate_actions: vec![Action::DoThen(
            Box::new(Action::DiscardFoodChoice(Box::new([(FoodIndex::Seed, 1)]))),
            Box::new(Action::GetEggAtLoc(*habitat, bird_idx, 1)),
          )],
          ..Default::default()
        })
      },
      Self::BrownFalcon => {
        // look at a [card] from the deck. if its food cost includes an [invertebrate] or a [rodent], tuck it behind this bird. if not, discard it.
        let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];

        let food_cost = bird_card.cost().0;
        let is_valid = food_cost[FoodIndex::Rodent as usize].is_some() || food_cost[FoodIndex::Invertebrate as usize].is_some();

        if is_valid {
          env.current_player_mut().get_mat_mut().get_row_mut(habitat).tuck_card(bird_idx);
        }
        Ok(Default::default())
      },
      Self::WhiteCrestedLaughingthrush => {
        // tuck 1 [card] from your hand behind this bird. if you do, gain 1 [invertebrate], [seed], or [fruit] from the birdfeeder.
        todo!()
      },
      Self::NoisyMiner => {
        // tuck 1 [card] from your hand behind this bird. if you do, lay up to 2 [egg] on this bird. all other players may lay 1 [egg].
        todo!()
      },
      Self::EurasianCoot => {
        // tuck up to 3 [card] from your hand behind this bird.
        todo!()
      },
      Self::GreatCormorant => {
        // you may move 1 [fish] from this bird to your supply. then, roll any 2 [die]. if any are [fish], cache 1 [fish] on this bird from the supply.
        todo!()
      },
      Self::NewHollandHoneyeater => {
        // gain 1 [nectar] from the birdfeeder, if there is one.
        todo!()
      },
      Self::AustralianShelduck => {
        // draw 1 face-up [card] from the tray with a [cavity] or [star] nest. you may reset or refill the tray before doing so.
        todo!()
      },
      Self::SmallMinivet => {
        // play 1 additional bird in your [forest]. you may ignore 1 [invertebrate] or 1 [egg] in its cost.
        todo!()
      },
      Self::AtlanticPuffin
        | Self::BellsVireo
        | Self::CaliforniaCondor
        | Self::CassinsFinch
        | Self::CeruleanWarbler
        | Self::ChestnutCollaredLongspur
        | Self::GreaterPrairieChicken
        | Self::KingRail
        | Self::PaintedBunting
        | Self::RedCockadedWoodpecker
        | Self::RoseateSpoonbill
        | Self::SpottedOwl
        | Self::SpraguesPipit
        | Self::WhoopingCrane
        | Self::WoodStork => {
        // draw 2 new bonus cards and keep 1.
        // TODO: Discard should be only of these two cards
        env.draw_bonus_cards(2);
        Ok(ActivateResult {
          immediate_actions: vec![Action::DiscardBonusCard],
          ..Default::default()
        })
      },
      Self::AustralianRaven => {
        // cache up to 5 [wild] from your supply on this bird.
        todo!()
      },
      Self::StorkBilledKingfisher => {
        // choose any 1 [die]. roll it once for each of your [wetland] birds. if you roll at least 1 [fish], gain 1 from the supply. you may cache it on this bird.
        todo!()
      },
      Self::BlackBelliedWhistlingDuck
        | Self::CanadaGoose
        | Self::SandhillCrane
        | Self::AmericanWhitePelican
        | Self::DoubleCrestedCormorant
        => {
        // discard 1 [seed] to tuck 2 [card] from the deck behind this bird.

        let choices = match self {
          Self::BlackBelliedWhistlingDuck
            | Self::CanadaGoose
            | Self::SandhillCrane => Box::new([(FoodIndex::Seed, 1)]),
          Self::AmericanWhitePelican
            | Self::DoubleCrestedCormorant => Box::new([(FoodIndex::Fish, 1)]),
          _ => return Err(WingError::InvalidBird(format!("Got {self:?} in unexpected arm")))
        };

        Ok(ActivateResult{
          immediate_actions: vec![Action::DoThen(
            Box::new(Action::DiscardFoodChoice(choices)),
            Box::new(Action::MultipleActions(vec![Action::TuckBirdCardFromDeck(*habitat, bird_idx), Action::TuckBirdCardFromDeck(*habitat, bird_idx)])),
          )],
          ..Default::default()
        })
      },
      Self::BrownPelican => {
        // gain 3 [fish] from the supply.
        env.current_player_mut().add_food(FoodIndex::Fish, 3);
        Ok(Default::default())
      },
      Self::GreenBeeEater => {
        // if there is at least 1 bird on the tray that has [invertebrate] in its food cost, tuck 1 of them behind this bird.
        todo!()
      },
      Self::EuropeanRobin => {
        // from the supply, gain 1 food of a type you already gained this turn.
        todo!()
      },
      Self::CommonKingfisher => {
        // steal 1 [fish] from another player's supply and cache it on this bird. they gain 1 [die] from the birdfeeder.
        todo!()
      },
      Self::MandarinDuck => {
        // draw 5 [card] from the deck. add 1 to your hand, tuck 1 behind this bird, give 1 to another player, and discard the rest.
        todo!()
      },
      Self::CoppersmithBarbet => {
        // gain 1 [invertebrate] or [fruit] from the birdfeeder, if there is one.
        todo!()
      },
      Self::EurasianHoopoe => {
        // steal 1 [invertebrate] from each of your neighbors. each neighbor from whom a [invertebrate] was stolen may gain 1 [wild] from the supply.
        todo!()
      },
      Self::GracefulPrinia => {
        // discard 1 [egg]. if you do, gain 1 [invertebrate] from the supply.
        todo!()
      },
      Self::WhiteBelliedSeaEagle => {
        // reset the birdfeeder. gain 1 [fish] or 1 [rodent] from the birdfeeder, if there is one, and cache it on this bird.
        todo!()
      },
      Self::PeacefulDove => {
        // discard any number of [seed]. lay 1 [egg] on this bird for each discarded [seed].
        todo!()
      },
      Self::GreatTit => {
        // reset the birdfeeder. if you do, gain 1 [die] from the birdfeeder after resetting.
        todo!()
      },
      Self::EurasianMagpie => {
        // choose 1 other player. for each action cube on their [grassland], cache 1 [wild] from the supply on any of your birds.
        todo!()
      },
      Self::MajorMitchellsCockatoo => {
        // tuck 1 [card] from your hand behind this bird. if you do, all players gain 1 [seed] from the supply.
        todo!()
      },
      Self::CommonSandpiper => {
        // draw 1 [card] for each bird in your [wetland] with an [egg] on it. keep 1 and discard the rest.
        todo!()
      },
      Self::WillowTit => {
        // cache 1 [invertebrate], [seed], or [fruit] from the birdfeeder on this bird, if there is one.
        todo!()
      },
      Self::LazuliBunting | Self::WesternMeadowlark | Self::PileatedWoodpecker => {
        // all players lay 1 [egg] on any 1 [NEST TYPE] bird. you may lay 1 [egg] on 1 additional [bowl] bird.

        let nest_type = match self {
          Self::LazuliBunting => NestType::Bowl,
          Self::WesternMeadowlark => NestType::Ground,
          Self::PileatedWoodpecker => NestType::Cavity,
          _ => return Err(WingError::InvalidBird(format!("Got {self:?} in unexpected arm")))
        };

        let cur_player_idx = env.current_player_idx();
        let mut actions = Vec::new();
        for player_idx in 0..env.config().num_players {
          let mat = env.get_player(player_idx).get_mat();
          let mut playable_birds = vec![];
          for habitat in [Habitat::Forest, Habitat::Grassland, Habitat::Wetland] {
            let row = mat.get_row(&habitat);
            for (bird_idx, bird_card) in row.get_birds().iter().enumerate() {
              if bird_card.nest_type() == &nest_type && row.can_place_egg(bird_idx) {
                playable_birds.push((habitat, bird_idx));
              }
            }
          }

          // If there are birds that satisfy condition, add actions for that
          if !playable_birds.is_empty() {
            if player_idx == cur_player_idx {
              actions.push(Action::GetEggChoice(playable_birds.clone().into_boxed_slice()));
              actions.push(Action::GetEggChoice(playable_birds.into_boxed_slice()));
            } else {
              actions.push(Action::GetEggChoice(playable_birds.into_boxed_slice()));
            }
            actions.push(Action::ChangePlayer(player_idx));
          }
        }

        // Switch back to the player
        actions.insert(0, Action::ChangePlayer(cur_player_idx));
        Ok(ActivateResult {
          immediate_actions: actions,
          ..Default::default()
        })
      },
      Self::CommonStarling
        | Self::EurasianCollaredDove => {
        // discard up to 5 [wild] from your supply. for each, tuck 1 [card] from the deck behind this bird.
        todo!()
      },
      Self::LittlePenguin => {
        // draw and discard 5 [card] from the deck. for each [fish] in their food costs, cache 1 [fish] from the supply on this bird.
        todo!()
      },
      Self::PesquetsParrot => {
        // if the player to your right has a [nectar] in their personal supply, gain 1 [nectar] from the general supply.
        todo!()
      },
      Self::HoodedCrow => {
        // choose 1 other player. for each action cube on their [grassland], tuck 1 [card] from your hand behind this bird, then draw an equal number of [card].
        todo!()
      },
      Self::AmericanBittern
        | Self::CommonLoon => {
        // player(s) with the fewest birds in their [wetland] draw 1 [card].
        let min_birds_num = (0..env.config().num_players)
          .map(|idx| {
            env.get_player(idx).get_mat().get_row(&Habitat::Wetland).get_birds().len()
          })
          .min()
          .unwrap();

        let cur_player_idx = env.current_player_idx();

        for player_idx in 0..min_birds_num {
          if min_birds_num < env.get_player(player_idx).get_mat().get_row(&Habitat::Wetland).get_birds().len() {
            continue;
          }

          env.set_current_player(player_idx);
          let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];
          env.current_player_mut().add_bird_card(bird_card);
        }

        env.set_current_player(cur_player_idx);
        Ok(Default::default())
      },
      Self::WhiteFacedHeron => {
        // reset the birdfeeder and gain all [fish], if there are any. you may cache any or all of them on this bird.
        todo!()
      },
      Self::GreyShrikethrush => {
        // reset the birdfeeder and gain all [rodent], if there are any. you may cache any or all of them on this bird.
        todo!()
      },
      Self::GreatCrestedGrebe
        | Self::WilsonsStormPetrel => {
        // draw 1 [card] for each empty card slot in this row. at the end of your turn, keep 1 and discard the rest.
        todo!()
      },
      Self::GreaterRoadrunner => {
        // look at a [card] from the deck. if less than 50cm, tuck it behind this bird. if not, discard it.
        let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];

        let is_valid = match bird_card.wingspan() {
          Some(x) => x < 50,
          None => true
        };

        if is_valid {
          env.current_player_mut().get_mat_mut().get_row_mut(habitat).tuck_card(bird_idx);
        }
        Ok(Default::default())
      },
      Self::Kea => {
        // draw 1 bonus card. you may discard any number of [wild] to draw that many additional bonus cards. keep 1 of the cards you drew and discard the rest.
        todo!()
      },
      Self::CoalTit
        | Self::EurasianNuthatch => {
        // gain 1 [seed] from the supply and cache it on this card. at any time, you may spend [seed] cached on this card.
        todo!()
      },
      Self::BlackStork => {
        // lay 1 [egg] on each of the birds immediately to the left and right of this bird.
        todo!()
      },
      Self::PrincessStephaniesAstrapia => {
        // choose 1 other player. you both lay 1 [egg].
        todo!()
      },
      Self::WhiteThroatedSwift => {
        // tuck 1 [card] from your hand behind this bird. if you do, lay 1 [egg] on any bird.
        Ok(ActivateResult {
          immediate_actions: vec![Action::DoThen(Box::new(Action::TuckBirdCard(*habitat, bird_idx)), Box::new(Action::GetEgg))],
          ..Default::default()
        })
      },
      Self::AcornWoodpecker
        | Self::BlueJay
        | Self::ClarksNutcracker
        | Self::RedBelliedWoodpecker
        | Self::RedHeadedWoodpecker
        | Self::StellersJay => {
        // gain 1 [seed] from the birdfeeder, if available. you may cache it on this bird.
        if env._bird_feeder.count(FoodIndex::Seed) > 0 {
        };
        if env._bird_feeder.take_specific_food(FoodIndex::Seed).is_err() {
          // There is no food in bird feeder
          return Ok(Default::default());
        }
        // Add it to player stash
        env.current_player_mut().add_food(FoodIndex::Seed, 1);

        Ok(ActivateResult {
          immediate_actions: vec![Action::DoThen(
            Box::new(Action::DiscardFoodChoice(Box::new([(FoodIndex::Seed, 1)]))),
            Box::new(Action::CacheFoodChoice(Box::new([(FoodIndex::Seed, 1)]), *habitat, bird_idx))
          )],
          ..Default::default()
        })
      },
      Self::CommonSwift => {
        // discard up to 5 [invertebrate] from your supply. for each, tuck 1 [card] from the deck behind this bird.
        todo!()
      },
      Self::BayaWeaver => {
        // tuck up to 3 [card] from your hand behind this bird. if you tuck at least 1 [card], lay 1 [egg] on this bird.
        todo!()
      },
      Self::EasternWhipbird => {
        // choose 1 other player. you both gain 1 [seed] from the supply.
        todo!()
      },
      Self::Ruff => {
        // tuck up to 3 [card] from your hand behind this bird. draw 1 [card] for each card you tucked.
        todo!()
      },
      Self::ManyColoredFruitDove => {
        // all players gain 1 [fruit] from the supply. you gain 1 additional [fruit] from the supply.
        todo!()
      },
      Self::MagpieLark => {
        // discard 2 [egg] from your [forest]. if you do, play 1 bird in your [grassland] at its normal food cost (ignore its egg cost). if it has a "when played" or "game end" power, you may use it.
        todo!()
      },
      Self::EuropeanRoller => {
        // place this bird sideways, so that it covers 2 [grassland] spaces. pay the lower egg cost.
        todo!()
      },
      Self::CrestedPigeon => {
        // cache up to 8 [seed] from your supply on this bird.
        todo!()
      },
      Self::GoldenEagle
        | Self::GreatHornedOwl
        | Self::PeregrineFalcon => {
        // look at a [card] from the deck. if less than 100cm, tuck it behind this bird. if not, discard it.
        let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];

        let is_valid = match bird_card.wingspan() {
          Some(x) => x < 100,
          None => true
        };

        if is_valid {
          env.current_player_mut().get_mat_mut().get_row_mut(habitat).tuck_card(bird_idx);
        }
        Ok(Default::default())
      },
      Self::SquaccoHeron => {
        // gain 1 face-up [card] that can live in [wetland].
        todo!()
      },
      Self::RoseBreastedGrosbeak => {
        // gain 1 [seed] or [fruit] from the birdfeeder, if available.
        let has_seed = env._bird_feeder.contains(FoodIndex::Seed) > 0;
        let has_fruit = env._bird_feeder.contains(FoodIndex::Fruit) > 0;

        if has_seed && has_fruit {
          return Ok(ActivateResult {
            immediate_actions: vec![Action::GetFoodFromSupplyChoice(Box::new([FoodIndex::Seed, FoodIndex::Fruit]))],
            ..Default::default()
          });
        }
        if has_seed {
          env.current_player_mut().add_food(FoodIndex::Seed, 1);
          env._bird_feeder.take_specific_food(FoodIndex::Seed).unwrap();
        } else if has_fruit {
          env.current_player_mut().add_food(FoodIndex::Fruit, 1);
          env._bird_feeder.take_specific_food(FoodIndex::Fruit).unwrap();
        }

        Ok(ActivateResult::default())
      },
      Self::RoyalSpoonbill => {
        // draw 1 face-up [card] from the tray with a [platform] or [star] nest. you may reset or refill the tray before doing so.
        todo!()
      },
      Self::GrayWagtail => {
        // if you used all 4 types of action this round, gain 2 [wild] from the supply.
        todo!()
      },
      Self::KelpGull => {
        // discard any number of [wild] to draw that many [card].
        todo!()
      },
      Self::HouseCrow => {
        // you may cache 1 [wild] from your supply on each bird in this row.
        todo!()
      },
      Self::RedAvadavat => {
        // give 1 [card] from your hand to another player. if you do, draw 2 [card].
        todo!()
      },
      Self::RedNeckedAvocet => {
        // if the player to your left or right has an [invertebrate] in their personal supply, gain 1 [invertebrate] from the general supply.
        todo!()
      },
      Self::OrientalMagpieRobin => {
        // for every 3 [egg] in your [grassland], gain 1 [invertebrate] or [seed] from the supply. you may cache up to 2 of them on this bird.
        todo!()
      },
      Self::LittleRingedPlover => {
        // discard 1 [card] from your hand. if you do, lay 1 [egg] on this bird.
        todo!()
      },
      Self::CommonMyna => {
        // copy the "when activated" (brown) power of a bird in the [grassland] of the player on your left.
        todo!()
      },
      Self::AmericanGoldfinch => {
        // gain 3 [seed] from the supply.
        env.current_player_mut().add_food(FoodIndex::Seed, 3);
        Ok(Default::default())
      },
      Self::AsianEmeraldDove => {
        // lay 2 [egg] on each other bird in this column.
        for (col_idx, birds) in env.current_player().get_mat().get_columns().iter().enumerate() {
          let row_idx = birds.iter().position(|x| x == &Some(self));

          if let Some(row_idx) = row_idx {
            // Found a column with this bird

            // Grab row idxs not equal to this one
            for cur_row_idx in [0, 1, 2] {
              if row_idx == cur_row_idx {
                continue;
              }

              let _ = env.current_player_mut().get_mat_mut().get_row_mut(&cur_row_idx.into()).place_egg_at_exact_column(col_idx);
            }
            break;
          }
        }

        Ok(Default::default())
      },
      Self::Tui => {
        // copy a brown power on one bird in the [forest] of the player to your left.
        todo!()
      },
      Self::Emu => {
        // gain all [seed] that are in the birdfeeder. keep half (rounded up), then choose how to distribute the remainder among the other player(s).
        todo!()
      },
      Self::GreyWarbler => {
        // play another bird in your [forest]. pay its normal cost with a 1 [egg] discount.
        todo!()
      },
      Self::ZebraDove => {
        // all players may discard 1 [seed] from their supply to lay 1 [egg].
        todo!()
      },
      Self::AbbottsBooby => {
        // draw 3 bonus cards, then discard 2. you may discard bonus cards you did not draw this turn.
        env.draw_bonus_cards(3);
        Ok(ActivateResult {
          immediate_actions: vec![Action::DiscardBonusCard, Action::DiscardBonusCard],
          ..Default::default()
        })
      },
      Self::EurasianJay => {
        // steal 1 [seed] from another player's supply and cache it on this bird. they gain 1 [die] from the birdfeeder.
        todo!()
      },
      Self::CarrionCrow
        | Self::GriffonVulture => {
        // choose any 1 player (including yourself). cache 1 [rodent] from the supply on this bird for each [predator] that player has.
        todo!()
      },
      Self::Budgerigar => {
        // tuck the smallest bird in the tray behind this bird.
        todo!()
      },
      Self::RedCappedRobin => {
        // if the player to your left has an [invertebrate] in their personal supply, gain 1 [invertebrate] from the general supply.
        todo!()
      },
      Self::Grandala => {
        // if you laid an [egg] on this bird this turn, tuck 1 [card] from the deck behind this bird.
        todo!()
      },
      Self::SpotlessCrake => {
        // lay 1 [egg] on each bird in your [wetland], including this one.
        todo!()
      },
      Self::Goldcrest => {
        // discard 1 [card] from your hand. if you do, play another bird in your [forest]. pay its normal food and egg cost.
        todo!()
      },
      Self::Anhinga
        | Self::BlackSkimmer
        | Self::CommonMerganser
        | Self::SnowyEgret
        | Self::WhiteFacedIbis
        | Self::Willet
        | Self::AmericanKestrel
        | Self::BarnOwl
        | Self::BroadWingedHawk
        | Self::BurrowingOwl
        | Self::EasternScreechOwl
        | Self::FerruginousHawk
        | Self::MississippiKite => {
        // roll all dice not in birdfeeder. if any are [fish], cache 1 [fish] from the supply on this bird.

        let food_idx = match self {
          Self::Anhinga
            | Self::BlackSkimmer
            | Self::CommonMerganser
            | Self::SnowyEgret
            | Self::WhiteFacedIbis
            | Self::Willet => FoodIndex::Fish,
          Self::AmericanKestrel
            | Self::BarnOwl
            | Self::BroadWingedHawk
            | Self::BurrowingOwl
            | Self::EasternScreechOwl
            | Self::FerruginousHawk
            | Self::MississippiKite => FoodIndex::Rodent,
          _ => panic!("Got bird {self:?} in activation case which it not belongs to")
        };

        let dice_idxs = food_idx.dice_sides();

        let dice_out_of_birdfeeder = env._bird_feeder.roll_all_dice_not_in_birdfeeder(&mut env.rng);
        let num_dice_matching = dice_out_of_birdfeeder.iter().filter(|dice_idx| dice_idxs.contains(dice_idx)).count();

        if num_dice_matching > 0 {
          env.current_player_mut().get_mat_mut().get_row_mut(habitat).cache_food(bird_idx, food_idx);
        }
        Ok(Default::default())
      },
      Self::OrientalBayOwl => {
        // activate the "when activated" (brown) powers of all of your other [predator].
        todo!()
      },
      Self::Rook => {
        // cache 1 [wild] from your supply on this bird or tuck 1 [card] from your hand behind this bird. if you do either, tuck 1 [card] from the deck behind this bird.
        todo!()
      },
      Self::AmericanOystercatcher => {
        // draw [card] equal to the number of players + 1. starting with you and proceeding clockwise, each player selects 1 of those cards and places it in their hand. you keep the extra card.
        Ok(
          ActivateResult {
            immediate_actions: vec![
              Action::ChangePlayer(env.current_player_idx()),
              Action::GetCardFromSetAndChangePlayer(env._bird_deck.draw_cards_from_deck(env.config().num_players + 1))
            ],
            ..Default::default()
          }
        )
      },
      Self::Osprey
        | Self::BaltimoreOriole
        | Self::BlackChinnedHummingbird
        | Self::RedCrossbill
        | Self::EasternPhoebe
        | Self::ScissorTailedFlycatcher
        => {
        // all players gain 1 [x] from the supply.

        let food_type = match self {
          Self::Osprey => FoodIndex::Fish,
          Self::BaltimoreOriole | Self::BlackChinnedHummingbird => FoodIndex::Fruit,
          Self::RedCrossbill => FoodIndex::Seed,
          Self::EasternPhoebe | Self::ScissorTailedFlycatcher => FoodIndex::Invertebrate,
          _ => panic!("Got bird {self:?} in activate where it does not belong.")
        };
        let cur_player_idx = env.current_player_idx();
        for player_idx in 0..env.config().num_players {
          env.set_current_player(player_idx);
          env.current_player_mut().add_food(food_type, 1);
        }

        env.set_current_player(cur_player_idx);
        Ok(Default::default())
      },
      Self::LewinsHoneyeater => {
        // choose 1 other player. you both gain 1 [nectar] from the supply.
        todo!()
      },
      Self::HimalayanMonal => {
        // all players gain 1 [seed] from the supply. you also lay 1 [egg].
        todo!()
      },
      Self::GrayCatbird
        | Self::NorthernMockingbird => {
        // repeat a brown power on another bird in this habitat.
        let num_choices  = env.current_player().get_mat().get_row(habitat).get_birds()
          .iter()
          .filter(|bc| bc.color() == &BirdCardColor::Brown)
          .count();

        if num_choices == 0 {
          return Ok(Default::default());
        }

        Ok(ActivateResult {
          immediate_actions: vec![
            Action::ChooseThenAction(num_choices as u8, *self, *habitat, bird_idx)
          ],
          ..Default::default()
        })
      },
      Self::Mistletoebird => {
        // gain 1 [fruit] from the supply, or discard 1 [fruit] to gain 1 [nectar] from the supply.
        todo!()
      },
      Self::LargeBilledCrow => {
        // cache 1 [wild] from your supply on any bird. if you do, you may tuck 1 [card] from your hand behind this bird.
        todo!()
      },
      Self::Brambling => {
        // draw 2 [card] from the deck and add them to your hand. then, tuck up to 2 [card] from your hand behind this bird.
        todo!()
      },
      Self::GreyButcherbird => {
        // look at a [card] from the deck. if its wingspan is less than 40cm, tuck it behind this bird and cache 1 [rodent] from the supply on this bird. if not, discard it.
        let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];

        let is_valid = match bird_card.wingspan() {
          Some(x) => x < 40,
          None => true
        };

        if is_valid {
          let row = env.current_player_mut().get_mat_mut().get_row_mut(habitat);
          row.tuck_card(bird_idx);
          row.cache_food(bird_idx, FoodIndex::Rodent);
        }
        Ok(Default::default())
      },
      Self::CommonBlackbird
        | Self::LongTailedTit => {
        // place this bird sideways, so that it covers 2 [forest] spaces. pay the lower egg cost.
        todo!()
      },
      Self::SplendidFairywren => {
        // lay 1 [egg] on each of your birds with a wingspan less than 30cm, including this one.
        todo!()
      },
      Self::SouthernCassowary => {
        // discard a bird from your [forest] and put this bird in its place (do not pay an egg cost). if you do, lay 4 [egg] on this bird and gain 2 [fruit] from the supply.
        todo!()
      },
      Self::AmericanCrow
        | Self::BlackCrownedNightHeron
        | Self::FishCrow => {
        // discard 1 [egg] from any of your other birds to gain 1 [wild] from the supply.
        Ok(ActivateResult {
          immediate_actions: vec![Action::DoThen(Box::new(Action::DiscardEgg), Box::new(Action::GetFood))],
          ..Default::default()
        })
      },
      Self::BlackNoddy => {
        // reset the birdfeeder and gain all [fish], if there are any. you may discard any of these [fish] to tuck that many [card] from the deck behind this bird instead.
        todo!()
      },
      Self::LittleEgret => {
        // draw 1 [card] from the deck and add it to your hand. all other players draw 1 [card] from the deck and keep it if it can live in [wetland].
        todo!()
      },
      Self::EurasianNutcracker => {
        // choose 1-5 birds in your [forest]. cache 1 [seed] from your supply on each.
        todo!()
      },
      Self::RedCrownedCrane => {
        // score 1 of your bonus cards now by caching 1 [wild] from the supply on this bird for each point. discard that bonus card and draw 1 new one.
        todo!()
      },
      Self::CettisWarbler
        | Self::EurasianGreenWoodpecker
        | Self::GreylagGoose => {
        // this bird counts double toward the end-of-round goal, if it qualifies for the goal.
        todo!()
      },
      Self::SpottedDove => {
        // if this bird has no birds to the right of it, you may move it (only the card) to the player mat of the player on your right (you choose its habitat). if you do, draw 3 [card].
        todo!()
      },
      Self::WhiteBrowedTitWarbler => {
        // for each bird in your [forest] with an egg on it, roll any 1 [die]. choose 1 type of food you rolled and gain 1 of that food from the supply.
        todo!()
      },
      Self::BlackThroatedDiver
        | Self::WhiteStork
        | Self::WhiteThroatedDipper => {
        // discard all remaining face-up [card] and refill the tray. if you do, draw 1 of the new face-up [card].
        todo!()
      },
      Self::RedWattledLapwing => {
        // discard any number of [card] from the tray, then refill it. if at least 1 of the discarded birds is a [predator], lay 1 [egg] on this bird.
        todo!()
      },
      Self::AmericanWoodcock
        | Self::BlueWingedWarbler
        | Self::HoodedWarbler
        | Self::ProthonotaryWarbler
        | Self::TrumpeterSwan
        | Self::WildTurkey => {
        // None
        Ok(Default::default())
      },
      Self::NorthernGannet => {
        // roll all dice not in birdfeeder. if any are a [fish], gain that many [fish] from the supply and cache them on this bird.
        let food_idx = FoodIndex::Fish;
        let dice_idxs = food_idx.dice_sides();

        let dice_out_of_birdfeeder = env._bird_feeder.roll_all_dice_not_in_birdfeeder(&mut env.rng);
        let num_dice_matching = dice_out_of_birdfeeder.iter().filter(|dice_idx| dice_idxs.contains(dice_idx)).count();

        for _ in 0..num_dice_matching {
          env.current_player_mut().get_mat_mut().get_row_mut(habitat).cache_food(bird_idx, food_idx);
        }

        Ok(Default::default())
      },
      Self::AudouinsGull => {
        // draw 2 [card] from the deck. tuck 1 behind this bird and keep the other.
        todo!()
      },
      Self::RufousOwl => {
        // draw 1 face-up [card] from the tray with a wingspan less than 75cm and tuck it behind this bird.
        todo!()
      },
      Self::SatyrTragopan => {
        // give 1 [card] from your hand to another player. if you do, lay 2 [egg] on this bird.
        todo!()
      },
      Self::GreyTeal => {
        // look at 3 [card] from the deck. keep 1 [wetland] bird, if there is one. you may add it to your hand or tuck it behind this bird. discard the other cards.
        todo!()
      },
      Self::Canvasback
        | Self::NorthernShoveler
        | Self::PurpleGallinule
        | Self::SpottedSandpiper
        | Self::WilsonsSnipe => {
        // all players draw 1 [card] from the deck.
        let cur_player_idx = env.current_player_idx();
        for player_idx in 0..env.config().num_players {
          env.set_current_player(player_idx);
          let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];
          env.current_player_mut().add_bird_card(bird_card);
        }

        env.set_current_player(cur_player_idx);
        Ok(Default::default())
      },
      Self::GreenHeron => {
        // trade 1 [wild] for any other type from the supply.
        Ok(ActivateResult {
          immediate_actions: vec![Action::DoThen(
              Box::new(Action::DiscardFood),
              Box::new(Action::GetFoodFromSupplyChoice(FoodIndex::iter().collect::<Box<[FoodIndex]>>()))
            )],
          ..Default::default()
        })
      },
      Self::RhinocerosAuklet => {
        // roll any 2 [die]. if you roll at least 1 [fish], cache 1 [fish] from the supply on this bird. all players may discard 1 [card] from their hand to gain 1 [fish] from the supply.
        todo!()
      },
      Self::BewicksWren
        | Self::BlueGrosbeak
        | Self::ChimneySwift
        | Self::CommonNighthawk
        | Self::LincolnsSparrow
        | Self::SongSparrow
        | Self::WhiteCrownedSparrow
        | Self::YellowBreastedChat => {
        // if this bird is to the right of all other birds in its habitat, move it to another habitat.
        if env.current_player().get_mat().get_row(habitat).get_birds().len() == bird_idx + 1 {
          let other_habitats: Vec<_> = self.habitats().iter().filter(|card_hab| card_hab != &habitat).cloned().collect();
          Ok(ActivateResult {
            immediate_actions: vec![Action::MoveBird(*self, other_habitats)],
            ..Default::default()
          })
        } else {
          Ok(Default::default())
        }
      },
      Self::TrumpeterFinch => {
        // play 1 additional bird in your [grassland]. you may ignore 1 [seed] or 1 [egg] in its cost.
        todo!()
      },
      Self::IndigoBunting
        | Self::WesternTanager => {
        // gain 1 [invertebrate] or [fruit] from the birdfeeder, if available.
        let num_inverterbrate = env._bird_feeder.contains(FoodIndex::Invertebrate);
        let num_fruit = env._bird_feeder.contains(FoodIndex::Fruit);

        if num_fruit == 0 || num_inverterbrate == 0 {
          return Ok(Default::default());
        }

        let mut v = vec![];
        if num_fruit > 0 {
          v.push(FoodIndex::Fruit);
        }
        if num_inverterbrate > 0 {
          v.push(FoodIndex::Invertebrate);
        }

        Ok(ActivateResult{
          immediate_actions: vec![Action::GetFoodFromSupplyChoice(v.into_boxed_slice())],
          ..Default::default()
        })
      },
      Self::BeardedReedling => {
        // for each other bird in this column with an egg on it, lay 1 [egg] on this bird.
        todo!()
      },
      Self::CorsicanNuthatch => {
        // draw 1 new bonus card. then gain 1 [die] from the birdfeeder.
        env.draw_bonus_cards(1);
        Ok(ActivateResult {
          immediate_actions: vec![Action::GetFood],
          ..Default::default()
        })
      },
      Self::EurasianTreecreeper => {
        // for every 3 [egg] in your [forest], gain 1 [invertebrate] or [seed] from the supply. you may cache up to 2 of them on this bird.
        todo!()
      },
      Self::GreatCrestedFlycatcher => {
        // gain 1 [invertebrate] from the birdfeeder, if available.
        if let Ok(()) = env._bird_feeder.take_specific_food(FoodIndex::Invertebrate) {
          env.current_player_mut().add_food(FoodIndex::Invertebrate, 1);
        }
        Ok(Default::default())
      },
      Self::WillieWagtail => {
        // draw 1 face-up [card] from the tray with a [bowl] or [star] nest. you may reset or refill the tray before doing so.
        todo!()
      },
      Self::BlackTailedGodwit
        | Self::RedKnot => {
        // draw 1 new bonus card. then draw 3 [card] and keep 1 of them.
        todo!()
      },
      Self::ChihuahuanRaven
        | Self::CommonRaven => {
        // discard 1 [egg] from any of your other birds to gain 2 [wild] from the supply.
        let mut choices = vec![];
        let mat = env.current_player().get_mat();
        for iter_habitat in [Habitat::Forest, Habitat::Grassland, Habitat::Wetland] {
          let row = mat.get_row(&iter_habitat);
          for (iter_bird_idx, _bird_card) in row.get_birds().iter().enumerate() {
            if (iter_habitat != *habitat || iter_bird_idx != bird_idx) && row.can_discard_egg(bird_idx)  {
              choices.push((iter_habitat, iter_bird_idx));
            }
          }
        }
        Ok(ActivateResult{
          immediate_actions: vec![Action::DoThen(
            Box::new(Action::DiscardEggChoice(choices.into_boxed_slice())),
            Box::new(Action::MultipleActions(vec![Action::GetFood, Action::GetFood])),
          )],
          ..Default::default()
        })
      },
      Self::EurasianEagleOwl
        | Self::EurasianMarshHarrier => {
        // up to 3 times, draw 1 [card] from the deck. when you stop, if the birds' total wingspan is less than 110 cm, tuck them behind this bird. if not, discard them.
        todo!()
      },
      Self::IndianVulture => {
        // copy one bonus card of the player on your right, as if it were your own (score it based on your own birds).
        todo!()
      },
      Self::NorthernCardinal
        | Self::BlueGrayGnatcatcher
        | Self::PaintedWhitestart
        | Self::YellowBelliedSapsucker
        | Self::SpottedTowhee
        => {
        // gain 1 [x] from the supply.
        let food_type = match self {
          Self::NorthernCardinal => FoodIndex::Fruit,
          Self::BlueGrayGnatcatcher | Self::PaintedWhitestart | Self::YellowBelliedSapsucker => FoodIndex::Invertebrate,
          Self::SpottedTowhee => FoodIndex::Seed,
          _ => panic!("Got bird {self:?} in action that is not related.")
        };
        env.current_player_mut().add_food(food_type, 1);
        Ok(Default::default())
      },
      Self::SuperbLyrebird => {
        // copy a brown power on one bird in the [forest] of the player to your right.
        todo!()
      },
      Self::BarredOwl
        | Self::CoopersHawk
        | Self::NorthernHarrier
        | Self::RedShoulderedHawk
        | Self::RedTailedHawk
        | Self::SwainsonsHawk => {
        // look at a [card] from the deck. if less than 75cm, tuck it behind this bird. if not, discard it.
        let bird_card = env._bird_deck.draw_cards_from_deck(1)[0];

        let is_valid = match bird_card.wingspan() {
          Some(x) => x < 75,
          None => true
        };

        if is_valid {
          env.current_player_mut().get_mat_mut().get_row_mut(habitat).tuck_card(bird_idx);
        }
        Ok(Default::default())
      },
      Self::RedJunglefowl => {
        // count the [egg] on all of your birds. if the total is fewer than 6 [egg], lay 1 [egg] on this bird.
        todo!()
      },
      Self::SriLankaBlueMagpie => {
        // you may cache 1 [wild] from your supply on each of your other birds.
        todo!()
      },
      Self::CommonTailorbird => {
        // find a contiguous group of birds in your preserve that all have the same nest type. lay 1 [egg] on each of them. [star] nests count as any nest type.
        todo!()
      },
      Self::EurasianKestrel => {
        // roll any 3 [die]. if you roll at least 1 [rodent], cache 1 [rodent] on this bird.
        todo!()
      },
      Self::HorsfieldsBushlark => {
        // discard 1 [seed]. if you do, lay up to 2 [egg] on this bird.
        todo!()
      },
      Self::SarusCrane => {
        // each player may discard 1 [egg] to draw 1 [card] from the deck.
        let mut actions: Vec<_> = (0..env.config().num_players)
          .flat_map(|player_idx| {
            [
              Action::DoThen(Box::new(Action::DiscardEgg), Box::new(Action::GetBirdCardFromDeck)),
              Action::ChangePlayer(player_idx),
            ]
          }).collect();

        actions.insert(0, Action::ChangePlayer(env.current_player_idx()));
        Ok(ActivateResult{
          immediate_actions: actions,
          ..Default::default()
        })
      },
      Self::GreatHornbill => {
        // all players may tuck a [card] from their hand under a bird in their [forest] and/or cache 1 [fruit] from their supply on a bird in their [forest].
        todo!()
      },
      Self::RuddyShelduck => {
        // draw 5 [card] from the deck. add 1 to your hand, tuck 1 behind this bird, and discard the rest.
        todo!()
      },
      Self::HouseSparrow => {
        // discard up to 5 [seed] from your supply. for each, tuck 1 [card] from the deck behind this bird.
        todo!()
      },
      Self::GreyHeron => {
        // place this bird sideways, so that it covers 2 [wetland] spaces. pay the lower egg cost.
        todo!()
      },
      Self::BlackSwan => {
        // lay 1 [egg] on each of your birds with a wingspan over 100cm, including this one.
        todo!()
      },
      Self::RainbowLorikeet => {
        // discard 1 [nectar] to the "spent nectar" space for your [forest]. if you do, gain 2 [die] from the birdfeeder.
        todo!()
      },
      Self::SouthIslandRobin => {
        // if the player to your right has an [invertebrate] in their supply, cache 1 [invertebrate] from the general supply on this bird.
        todo!()
      },
      Self::SavisWarbler => {
        // draw 2 [card]. all other players draw 1 [card] from the deck.
        todo!()
      },

      Self::BlackWoodpecker
        | Self::NorthernFlicker
        | Self::BaldEagle => {
        // gain all FOOD TYPE that are in the birdfeeder.
        let food_type = match self {
          Self::BlackWoodpecker | Self::NorthernFlicker => FoodIndex::Invertebrate,
          Self::BaldEagle => FoodIndex::Fish,
          _ => return Err(WingError::InvalidBird(format!("Got {self:?} in unexpected arm")))
        };

        let num_food = env._bird_feeder.contains(food_type);
        env.current_player_mut().add_food(food_type, num_food as u8);

        for _ in 0..num_food {
          env._bird_feeder.take_specific_food(food_type).unwrap();
        }

        Ok(Default::default())
      },
      Self::GreaterFlamingo => {
        // choose 1 other player. for each action cube on their [wetland], tuck 1 [card] from your hand behind this bird, then draw an equal number of [card].
        todo!()
      },
      Self::Smew => {
        // draw 4 [card]. tuck 2 [card] behind this bird and add the other 2 [card] to your hand.
        todo!()
      },
      Self::EuropeanBeeEater => {
        // reset the birdfeeder. if you do, gain 1 [invertebrate] from the birdfeeder after resetting.
        todo!()
      },
      Self::CommonMoorhen => {
        // discard 1 [wild] from your supply. if you do, play another bird in your [wetland]. pay its normal food and egg cost.
        todo!()
      },
      Self::GoldenPheasant => {
        // all players lay 2 [egg]. you lay 2 additional [egg].
        todo!()
      },
    }
  }

  /// Performs a condition check for "Once between turns, when X happens do Y" (aka Pink birds).
  ///
  /// If bird_player_idx is current players index it always returns Ok(false).
  /// Does both the check (X) and performs action if (Y) happens.
  /// If bird is has such a condition (it is a pink bird) it will return Ok(true), if X succeeds and Ok(false) if it fails.
  /// If bird is not a pink bird, returns a Err(InvalidBird)
  pub fn conditional_callback(
    &self,
    env: &mut WingspanEnv,
    action_type_taken: &Action,
    action_taken: u8,
    bird_habitat: &Habitat,
    bird_idx: usize,
    bird_player_idx: usize,
  ) -> WingResult<bool> {
    // Pink b
    if bird_player_idx == env.current_player_idx() {
      return Ok(false);
    }


    match self {
      Self::SacredKingfisher | Self::AustralianOwletNightjar | Self::EurasianTreeSparrow | Self::EurasianGoldenOriole => {
        // when another player takes the "gain food" action, gain 1 [FOOD types] from the birdfeeder, if there is one, at the end of their turn.
        if *action_type_taken == Action::ChooseAction && action_taken == 1 {
          let food_choice: Box<[FoodIndex]> = match self {
            Self::AustralianOwletNightjar => Box::new([FoodIndex::Invertebrate]),
            Self::SacredKingfisher => Box::new([FoodIndex::Invertebrate, FoodIndex::Fish, FoodIndex::Rodent]),
            Self::EurasianTreeSparrow => Box::new([FoodIndex::Seed]),
            Self::EurasianGoldenOriole => Box::new([FoodIndex::Invertebrate, FoodIndex::Fruit]),
            _ => return Err(WingError::InvalidBird(format!("Bird {self:?} was called in conditional callback path, but it doesn't invoke such."))),
          };

          env.prepend_actions(&mut [
            Action::ChangePlayer(env.current_player_idx()),
            Action::GetFoodFromSupplyChoice(food_choice),
            Action::ChangePlayer(bird_player_idx),
          ]);

          Ok(true)
        } else {
          Ok(false)
        }
      },
      Self::BronzedCowbird
        | Self::BrownHeadedCowbird
        | Self::YellowBilledCuckoo
        | Self::BarrowsGoldeneye
        | Self::AmericanAvocet => {
        // when another player takes the "lay eggs" action, lay 1 [egg] on a bird with a [NEST TYPE] nest.
        if *action_type_taken == Action::ChooseAction && action_taken == 1 {
          // Text includes "on another bird"
          let remove_self = matches!(self, Self::BarrowsGoldeneye);

          let nest_type = match self {
            Self::BronzedCowbird
              | Self::BrownHeadedCowbird
              | Self::YellowBilledCuckoo => NestType::Bowl,
            Self::AmericanAvocet => NestType::Ground,
            Self::BarrowsGoldeneye => NestType::Cavity,
            _ => return Err(WingError::InvalidBird(format!("Bird {self:?} was called in conditional callback path, but it doesn't invoke such."))),
          };

          let bird_player = env.get_player(bird_player_idx);
          let mut choices = bird_player.get_mat().get_birds_with_nest_type(nest_type);

          if remove_self {
            if let Some(idx) = choices.iter().position(|val | *val == (*bird_habitat, bird_idx)) {
              choices.swap_remove(idx);
            }
          }

          env.append_actions(&mut vec![
            Action::ChangePlayer(env.current_player_idx()),
            Action::GetEggChoice(choices.into_boxed_slice()),
            Action::ChangePlayer(bird_player_idx),
          ]);
          Ok(true)
        } else {
          Ok(false)
        }
      },
      Self::HornedLark => {
        // when another player plays a bird in their [grassland], tuck 1 [card] from your hand behind this bird.
        if last_bird_played_in_habitat(env, action_type_taken, action_taken, &Habitat::Grassland) {
          env.append_actions(&mut vec![
            Action::ChangePlayer(env.current_player_idx()),
            Action::TuckBirdCard(*bird_habitat, bird_idx),
            Action::ChangePlayer(bird_player_idx),
          ]);

          Ok(true)
        } else {
          Ok(false)
        }
      },
      Self::EasternKingbird => {
        // when another player plays a bird in their [forest], gain 1 [invertebrate] from the supply.
        if last_bird_played_in_habitat(env, action_type_taken, action_taken, &Habitat::Forest) {
          env.append_actions(&mut vec![
            Action::ChangePlayer(env.current_player_idx()),
            Action::GetFoodFromSupplyChoice(Box::new([FoodIndex::Invertebrate])),
            Action::ChangePlayer(bird_player_idx),
          ]);

          Ok(true)
        } else {
          Ok(false)
        }
      },
      Self::AsianKoel => {
        // when another player takes the "lay eggs" action, this bird lays 1 [egg] on another bird with a [platform] nest. you may go 3 over its egg limit while using this power.
        todo!()
      },
      Self::SnowBunting => {
        // when another player tucks a [card] for any reason, tuck 1 [card] from your hand behind this bird, then draw 1 [card] at the end of their turn.
        todo!()
      },
      Self::BeltedKingfisher => {
        // when another player plays a bird in their [wetland], gain 1 [fish] from the supply.
        if last_bird_played_in_habitat(env, action_type_taken, action_taken, &Habitat::Wetland) {
          env.append_actions(&mut vec![
            Action::ChangePlayer(env.current_player_idx()),
            Action::GetFoodFromSupplyChoice(Box::new([FoodIndex::Fish])),
            Action::ChangePlayer(bird_player_idx),
          ]);

          Ok(true)
        } else {
          Ok(false)
        }
      },
      Self::SpangledDrongo => {
        // when another player gains [nectar], gain 1 [nectar] from the supply.
        todo!()
      },
      Self::LoggerheadShrike => {
        // when another player takes the "gain food" action, if they gain any number of [rodent], cache 1 [rodent] from the supply on this bird.
        todo!()
      },
      Self::CommonCuckoo => {
        // when another player takes the "lay eggs" action, this bird lays 1 [egg] on another bird with a [bowl] or [ground] nest.
        todo!()
      },
      Self::BlackVulture
        | Self::BlackBilledMagpie
        | Self::TurkeyVulture => {
        // when another player's [predator] succeeds, gain 1 [die] from the birdfeeder.
        todo!()
      },
      Self::EuropeanGoldfinch => {
        // when another player tucks a [card] for any reason, tuck 1 [card] from the deck behind this bird.
        todo!()
      },
      Self::PheasantCoucal => {
        // when another player takes the "lay eggs" action, lay 1 [egg] on this bird.
        if *action_type_taken == Action::ChooseAction && action_taken == 1 {
          env.append_actions(&mut vec![
            Action::ChangePlayer(env.current_player_idx()),
            Action::GetEggAtLoc(*bird_habitat, bird_idx, 1),
            Action::ChangePlayer(bird_player_idx),
          ]);
          Ok(true)
        } else {
          Ok(false)
        }
      },
      Self::HorsfieldsBronzeCuckoo => {
        // when another player takes the "lay eggs" action, lay 1 [egg] on a bird with a wingspan less than 30cm.
        todo!()
      },
      Self::VioletCuckoo => {
        // when another player takes the "lay eggs" action, lay 1 [egg] on another bird with wingspan less than 30 cm. you may go 2 over its egg limit while using this power.
        todo!()
      },
      _ => Err(WingError::InvalidBird(format!("Bird {self:?} was called in callback, but it doesn't invoke such."))),
    }
  }

  pub fn after_choice_callback(&self, choice_idx: u8, env: &mut WingspanEnv, habitat: &Habitat, _bird_idx: usize,) -> WingResult<ActivateResult> {
    match self {
      Self::AnnasHummingbird
        | Self::RubyThroatedHummingbird => {
        // each player gains 1 [die] from the birdfeeder, starting with the player of your choice.
        let choice_idx = choice_idx as usize;
        if choice_idx >= env.config().num_players {
          return Err(WingError::InvalidAction);
        }

        let mut actions = Vec::new();
        // Loop from choice idx to end and then from start to choice idx
        for player_idx in choice_idx..env.config().num_players {
          actions.push(Action::GetFood);
          actions.push(Action::ChangePlayer(player_idx));
        }
        for player_idx in 0..choice_idx {
          actions.push(Action::GetFood);
          actions.push(Action::ChangePlayer(player_idx));
        }

        actions.insert(0, Action::ChangePlayer(env.current_player_idx()));

        Ok(ActivateResult {
          immediate_actions: actions,
          ..Default::default()
        })
      },
      Self::GrayCatbird
        | Self::NorthernMockingbird => {
        // repeat a brown power on another bird in this habitat.
        let choice_idx = choice_idx as usize;

        let bird_choices: Vec<_> = env.current_player().get_mat().get_row(habitat).get_birds()
          .iter()
          .enumerate()
          .filter(|(_bird_idx, bc)| bc.color() == &BirdCardColor::Brown)
          .collect();

        if choice_idx >= bird_choices.len() {
          return Err(WingError::InvalidAction);
        }

        let (choice_bird_idx, choice_bird_card) = bird_choices[choice_idx];
        choice_bird_card.clone().activate(env, habitat, choice_bird_idx)
      },
      Self::HoodedMerganser => {
        // repeat 1 [predator] power in this habitat.
        let choice_idx = choice_idx as usize;

        let bird_choices: Vec<_> = env.current_player().get_mat().get_row(habitat).get_birds()
          .iter()
          .enumerate()
          .filter(|(_bird_idx, bc)| bc.is_predator())
          .collect();

        if choice_idx >= bird_choices.len() {
          return Err(WingError::InvalidAction);
        }

        let (choice_bird_idx, choice_bird_card) = bird_choices[choice_idx];
        choice_bird_card.clone().activate(env, habitat, choice_bird_idx)
      },
      _ => Err(WingError::InvalidBird(format!("Bird {self:?} was called in callback, but it doesn't invoke such."))),
    }
  }
}


fn last_bird_played_in_habitat(
    env: &mut WingspanEnv,
    action_type_taken: &Action,
    action_taken: u8,
    bird_habitat: &Habitat,
) -> bool {
    *action_type_taken == Action::PlayBirdHabitat(*bird_habitat)
    || (
      *action_type_taken == Action::PlayBird
      && env.current_player().get_playable_card_hab_combos()
        .get(action_taken as usize)
        .map(|x| x.1 == *bird_habitat)
        .unwrap_or(false)
    )
}


#[cfg(test)]
mod tests {
    use std::panic;

    use crate::{bird_card::get_deck, expansion::Expansion, wingspan_env::{WingspanEnv, WingspanEnvConfigBuilder}};

  #[test]
  fn check_card_activation() {
    let deck = get_deck(&[Expansion::Core]);

    let mut good = 0;
    let mut bad = 0;

    for bird_card in deck {
      // Create a fresh env and play a card
      let mut env = WingspanEnv::try_new(WingspanEnvConfigBuilder::default().build().unwrap());

      let habitat = bird_card.habitats()[0];
      env.current_player_mut().get_mat_mut().put_bird_card(bird_card, &habitat).unwrap();

      match panic::catch_unwind(move || { bird_card.activate(&mut env, &habitat, 0) }) {
        Ok(_) => good += 1,
        Err(_) => bad += 1,
      };
    }

    assert_eq!(bad, 0, "Got {} bad cards (should be 0). Got {} good cards.", bad, good);
  }
}
