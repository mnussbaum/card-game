CREATE TABLE games_players (
  id SERIAL PRIMARY KEY,
  player_id integer REFERENCES players NOT NULL,
  game_id integer REFERENCES games NOT NULL
);
