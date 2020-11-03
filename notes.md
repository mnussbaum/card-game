* Which player is dealer is a piece of state too. Eventually need rules for how
  dealer moves, what it does
* Eventually have a game builder that shows how the pools are layer out on the
  table
* After playing each card eval how it's consequences impact each player
* Consequences will eventually need a priority to resolve conflicts
* Actions are going to need consequences too
  * Should "go to next turn" be on actions or the cards? On cards now since some cards move the turn back, but it's weird
* Some actions are required, some are optional, and some exclude other actions
  * Maybe use labels and selectors to implement exclusion? Or the desciptions
* Not sure about the grammar for actions yet

* Add turn phases
* Moving phases can be the side effect of actions or an action itself in some
  games
* Let player lay out turn and game structure in an explicit config
* Take the consequences off cards and put all the logic into the actions
* Add conditions on action consequences to implement "this card does this"
  logic
* Consequences should be actions
* Conditions will need to have variables for game state exposed, and eventually
  functions. I'm basically implementing ansible :/
* Remove actions returning conditions for future cards
* Instead have actions look at game state like card value to determine if
  they're actually playable
