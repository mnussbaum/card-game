## How to encode the rules of any card game?

1. Dealing
2. Who can play right now?
 1. In turn vs out of turn
 2. Multiple actions a person can take
8. Passing cards to other players
  1. How many
  2. To whom?
  3. Can recipient reject them?
9. How does any card impact play?
  1. Changes cards that can be played on top of it
  2. Change direction of play
  3. Reset turn
  4. End turn
  5. Cause player to gain points
  6. Cause player to lose points
3. Tracking kitty, multiple kitties
4. Tracking player bankroll, points
5. Tracking cards that are in the common area
6. What actions reset play eg new hand or clear a pile
  1. In turn reaches a certain person
  2. Certain cards are played
7. Ending the game
  1. Certain cards are played
  2. Game might end for certain players first, all players done
    1. Is the winner the first out or the last?
    2. Conditions to end game for a player
      1. Out of cards
      2. Out of points

* Poll each player to see what actions they can do at the start of every round
* Whether or not it's a players turn should be a piece of state that just opens
  up more actions
* Need to encode every possible action as data so that users can choose the
  combos
* Which player is dealer is a piece of state too. Eventually need rules for how
  dealer moves, what it does
* Players should have multiple pools of cards to represent the different piles
* Different card pools are either not visible to any, visible just to player or
  visible to all, specific card visibility like just top card. Need to define
  rules for moving cards between pools, how the play interacts with each pool
* Eventually have a game builder that shows how the pools are layer out on the
  table
* After playing each card eval how it's consequences impact each player
* Consequences should be expressed as noun, verb, subject expressions, eg "next
  player", "picks up", "to pool xyz", "from communal pool ABC" 
* Need way to say "if turn == 0 then player has these actions"
* Need way to say "if turn > 0 and player on turn then player has these default actions"
* Need way to transition card groups to be playable
* Consequences will eventually need a priority to resolve conflicts
