# Game of Trust

The simulator of the game called [The Evolution of Trust](https://ncase.me/trust/).

Basically, there are 5 types of agents

- `CheatingAgent` - always cheats.
- `CooperatingAgent` - always cooperates.
- `GrudgerAgent` - always cooperates until first betrayal, then always cheats.
- `CopycatAgent` - cooperates first, then repeats the last turn of opponent.
- `DetectiveAgent` - begins with sequence "cooperate", "cheat", "cooperate", "cooperate". If opponent never cheated, then always cheats. Otherwise, plays as copycat agent.
