import wingspan_gym
from wingspan_gym._internal import StepResult
import random

import wingspan_gym.game


class DummyHeuristic:
    def __init__(self, env: wingspan_gym.game.WingspanEnv):
        self.env = env
        self.tried_to_play_a_card = False
        self.played_a_card = False

    def get_next_action(self):
        action_type = self.env.next_action()
        if not self.played_a_card and str(action_type) == "ChooseAction":
            if not self.tried_to_play_a_card:
                self.tried_to_play_a_card = True
                return 0
            else:
                # Tried to play a card and failed
                # Get food then
                return 1

        # No longer choose action, so this works I think
        if self.tried_to_play_a_card:
            self.played_a_card = True

        self.tried_to_play_a_card = False

        return random.randint(0, self.env.action_space_size())


env = wingspan_gym.game.WingspanEnv()

env.reset()
player = [DummyHeuristic(env) for _ in range(2)]
# observation, info = env.reset()

episode_over = False
max_steps = 10000
step_idx = 0
while not episode_over and step_idx < max_steps:
    print("=========")
    step_idx += 1
    # action = env.action_space.sample()  # agent policy that uses the observation and info
    print(env._debug_print_state())

    action = player[env.cur_player()].get_next_action()
    print(f"Chose action: {action}")
    print(f"Step: {step_idx}/{max_steps}")

    if env.step(action) == StepResult.Terminated:
        break

env.close()
