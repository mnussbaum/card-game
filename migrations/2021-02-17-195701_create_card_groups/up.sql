CREATE TYPE card_group_enum_layout
    AS ENUM ('pile', 'spread');

CREATE TYPE card_group_enum_visibility
    AS ENUM ('face_down', 'face_up', 'top_face_up_rest_face_down', 'visible_to_owner');

CREATE TABLE card_groups (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  name VARCHAR NOT NULL,
  initial_size INTEGER,
  layout card_group_enum_layout NOT NULL,
  visibility card_group_enum_visibility NOT NULL,
  user_id INTEGER REFERENCES users,
  game_id INTEGER REFERENCES games NOT NULL
);

CREATE INDEX card_groups__game_id ON card_groups(game_id);

CREATE TABLE card_groups_cards (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  card_id INTEGER REFERENCES cards NOT NULL,
  card_group_id INTEGER REFERENCES card_groups NOT NULL
);

CREATE INDEX card_groups_cards__card_group_id_idx ON card_groups_cards(card_group_id);
