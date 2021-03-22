CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  player_turn_index INTEGER NOT NULL,
  turn_count INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
