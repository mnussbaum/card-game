CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  player_turn_index integer NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
