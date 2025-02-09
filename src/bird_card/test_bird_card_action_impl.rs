
#[cfg(test)]
mod test {
    use crate::{bird_card::BirdCard, habitat::Habitat, wingspan_env::{WingspanEnv, WingspanEnvConfigBuilder}};

    macro_rules! test_bird_card {
        ($test_name:ident, $bird_name:ident, $habitat:expr) => {
            #[test]
            fn $test_name() {
                let config_builder = WingspanEnvConfigBuilder::default();
                let mut env = WingspanEnv::try_new(config_builder.build().unwrap());

                let bird_card = BirdCard::$bird_name;
                let habitat = $habitat;

                env.current_player_mut().get_mat_mut().get_row_mut(&habitat).play_a_bird(bird_card);

                let _ = bird_card.activate(&mut env, &habitat, 0);
            }
        };
    }

}
