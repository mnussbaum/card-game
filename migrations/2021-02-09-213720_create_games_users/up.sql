CREATE TABLE games_users (
  id SERIAL PRIMARY KEY,
  user_id integer REFERENCES users NOT NULL,
  game_id integer REFERENCES games NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
