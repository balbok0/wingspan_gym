import wingspan_gym
import gymnasium

import wingspan_gym.game

env = wingspan_gym.game.WingspanEnv()

env.reset()
# observation, info = env.reset()

episode_over = False
max_steps = 100
step_idx = 0
while not episode_over and step_idx < max_steps:
    step_idx += 1
    action = env.action_space.sample()  # agent policy that uses the observation and info
    # print(action)
    print(env._debug_print_state())

    observation, reward, terminated, truncated, info = env.step(action)

    episode_over = terminated or truncated

env.close()

