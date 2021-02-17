CREATE TABLE card_groups (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),

  -- The owner of a card group is either a user or a game in the case of communal cards
  owner_type VARCHAR NOT NULL,
  owner_id INTEGER NOT NULL
);

CREATE INDEX card_groups__owner_type__owner_id_idx ON card_groups(owner_type, owner_id);

CREATE TABLE card_groups_cards (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  card_id INTEGER REFERENCES cards NOT NULL,
  card_group_id INTEGER REFERENCES card_groups NOT NULL
);

CREATE INDEX card_groups_cards__card_group_id_idx ON card_groups_cards(card_group_id);
