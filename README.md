# ggez-dodger

A kind of dodger game, written in Rust, using ggez.

## Gameplay
You start the game with a blank character, who can capture any kind of block.

When you capture a block, you get its color and face, 
and then you can only capture blocks sharing an attribute with you, 
and dodge the others.

## Controls
- Controller:
  - `Left`, `Right` => Go left or right,
  - `Down` => Go down faster,
  - `B` => Jump,
  - `A` => Use shield,
  - `Start` => Pause game,
  - `Back` => (Re)spawn
- Arrow keys:
  - `Left`, `Right` => Go left or right,
  - `Down` => Go down faster,
  - `Up` => Jump,
  - `Right Ctrl` => Use shield,
  - `Space` => Pause game,
  - `Enter` => (Re)spawn
- ZQSD (yes, I'm french, so I use a AZERTY keyboard layout...):
  - `Q`, `D` => Go left or right,
  - `S` => Go down faster,
  - `Z` => Jump,
  - `Left Shift` => Use shield,
  - `Space` => Pause game,
  - `Back` => (Re)spawn
  
  ## Screenshot
  ![screenshot](dodger-screenshot.png?raw=true)
