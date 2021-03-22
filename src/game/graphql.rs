use std::collections::HashMap;
use std::fs;

use diesel::prelude::*;
use itertools::{EitherOrBoth::*, Itertools};
use juniper::{graphql_object, FieldResult};

use crate::db::PooledConnection;
use crate::deck::graphql::CardGroup;
use crate::deck::records::{Card, CardGroupDescription, CardGroupRecord, Deck, NewCardGroupRecord};
use crate::errors::ServiceResult;
use crate::game::record::GameRecord;
use crate::game_rules::GameRules;
use crate::graphql::Context;
use crate::player::Player;
use crate::schema::{card_groups, card_groups_cards, cards, games_users, users};
use crate::user::model::User;

pub struct Game<'a> {
    record: GameRecord,
    communal_card_groups: HashMap<String, CardGroup<'a>>,
    pub player_state: Vec<Player<'a>>,
}

impl<'a> Game<'a> {
    pub fn offset_from_current_player_mut(&'a mut self, offset: i32) -> Option<&'a mut Player<'a>> {
        let player_index =
            (self.record.player_turn_index + offset) % self.player_state.len() as i32;

        self.player_state.get_mut(player_index as usize)
    }

    pub fn offset_from_current_player(&'a self, offset: i32) -> Option<&'a Player<'a>> {
        let player_index =
            (self.record.player_turn_index + offset) % self.player_state.len() as i32;

        self.player_state.get(player_index as usize)
    }

    pub fn communal_card_groups(&self) -> &HashMap<String, CardGroup<'a>> {
        &self.communal_card_groups
    }

    pub fn create_card_group_from_description(
        &mut self,
        description: &CardGroupDescription,
        connection: &PooledConnection,
    ) -> ServiceResult<CardGroupRecord> {
        Ok(diesel::insert_into(card_groups::table)
            .values(&NewCardGroupRecord::new_from_description(
                description,
                self.record.id,
            ))
            .returning(card_groups::all_columns)
            .get_result::<CardGroupRecord>(connection)?)
    }

    pub fn deal(&mut self, connection: &PooledConnection) -> ServiceResult<()> {
        let yams = fs::read_to_string("poo_head_rules.yaml")
            .expect("Something went wrong reading the file");
        let game_rules: GameRules = serde_yaml::from_str(&yams).unwrap();
        let mut deck = Deck::new_from_description(game_rules.deck, connection)?;

        for player_card_group_description in game_rules.player_hand.iter() {
            let mut players_card_groups_full = false;
            let mut player_card_groups: Vec<&mut CardGroup> = self
                .player_state
                .iter_mut()
                .map(|player| {
                    player.create_card_group_from_description(
                        player_card_group_description,
                        connection,
                    )
                })
                .fold_ok(Vec::new(), |mut acc, player| {
                    acc.push(player);
                    acc
                })?;

            for card_group_index in (0..player_card_groups.len()).cycle() {
                if card_group_index == 0 && players_card_groups_full {
                    break;
                } else {
                    players_card_groups_full = true
                }

                let card_group = player_card_groups
                    .get_mut(card_group_index)
                    .expect("Missing card_group fetched by index during dealing");
                let card_group_full =
                    card_group.deal_card_from_deck_if_not_full(&mut deck, connection)?;

                // If there aren't enough cards to finish dealing I abruptly
                // return here. Is this behavior correct? Should it be configurable?
                // if deck.cards.len() == 0 {
                //     return Ok(());
                // }

                if !card_group_full {
                    players_card_groups_full = false;
                }
            }
        }

        for communal_card_group_description in game_rules.communal_cards.iter() {
            let card_group_record = self
                .create_card_group_from_description(communal_card_group_description, connection)?;
            self.communal_card_groups
                .entry(card_group_record.name.clone())
                .or_insert(CardGroup::new(card_group_record, vec![]));
        }

        for (_, communal_card_group) in self.communal_card_groups.iter_mut() {
            loop {
                let card_group_full =
                    communal_card_group.deal_card_from_deck_if_not_full(&mut deck, connection)?;

                // If there aren't enough cards to finish dealing I abruptly
                // return here. Is this behavior correct? Should it be configurable?

                if card_group_full || deck.cards.len() == 0 {
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn new(record: GameRecord, connection: &PooledConnection) -> ServiceResult<Game<'a>> {
        let mut users = users::table
            .inner_join(games_users::table)
            .filter(games_users::game_id.eq(record.id))
            .order_by(users::id)
            .select(users::all_columns)
            .get_results::<User>(connection)?;

        let player_card_groups = CardGroupRecord::belonging_to(&users)
            .order_by(card_groups::user_id.desc())
            .load::<CardGroupRecord>(connection)?;

        let card_group_ids = &player_card_groups
            .iter()
            .map(|c| c.id)
            .collect::<Vec<i32>>();

        use diesel::pg::expression::dsl::any;
        let card_group_ids_and_cards = card_groups::table
            .left_outer_join(card_groups_cards::table.left_outer_join(cards::table))
            .filter(card_groups_cards::card_group_id.eq(any(card_group_ids)))
            .select((card_groups::id, cards::all_columns.nullable()))
            .order_by(card_groups::id)
            .load::<(i32, Option<Card>)>(connection)?;

        let mut cards_by_card_group: HashMap<i32, Vec<Card>> = card_group_ids_and_cards
            .into_iter()
            .group_by(|(card_group_id, _)| *card_group_id)
            .into_iter()
            .fold(
                HashMap::new(),
                |mut acc, (card_group_id, card_group_cards_iter)| {
                    let card_group_cards =
                        card_group_cards_iter.filter_map(|(_, card)| card).collect();
                    acc.entry(card_group_id).or_insert(card_group_cards);
                    acc
                },
            );

        let mut card_groups_by_user = player_card_groups.grouped_by(&users);
        card_groups_by_user.reverse();

        let mut user_card_groups_cards: HashMap<
            User,
            HashMap<String, (CardGroupRecord, Vec<Card>)>,
        > = HashMap::new();
        for users_card_groups in card_groups_by_user.into_iter() {
            let user = users.pop().expect("Missing expected user");
            let user_card_groups = user_card_groups_cards.entry(user).or_insert(HashMap::new());

            for card_group in users_card_groups.into_iter() {
                let cards = cards_by_card_group
                    .remove(card_group.id())
                    .unwrap_or(Vec::new());
                user_card_groups.insert(card_group.name.clone(), (card_group, cards));
            }
        }

        let communal_card_group_records = CardGroupRecord::belonging_to(&record)
            .filter(card_groups::user_id.is_null())
            .order_by(card_groups::id.desc())
            .load::<CardGroupRecord>(connection)?;
        let mut communal_card_groups_by_id =
            communal_card_group_records
                .into_iter()
                .fold(HashMap::new(), |mut acc, c| {
                    acc.entry(c.id).or_insert(c);
                    acc
                });

        let card_group_ids: Vec<&i32> = communal_card_groups_by_id.keys().collect();
        let communal_card_group_ids_and_cards = card_groups::table
            .left_outer_join(card_groups_cards::table.left_outer_join(cards::table))
            .filter(card_groups::id.eq(any(card_group_ids)))
            .select((card_groups::id, cards::all_columns.nullable()))
            .order_by(card_groups::id)
            .load::<(i32, Option<Card>)>(connection)?;

        let communal_card_groups: HashMap<String, CardGroup> = communal_card_group_ids_and_cards
            .into_iter()
            .group_by(|(card_group_id, _)| *card_group_id)
            .into_iter()
            .fold(
                HashMap::new(),
                |mut acc, (card_group_id, card_group_cards_iter)| {
                    let card_group_cards =
                        card_group_cards_iter.filter_map(|(_, card)| card).collect();
                    let card_group_record = communal_card_groups_by_id
                        .remove(&card_group_id)
                        .expect("Missing card group indexed by ID");

                    acc.entry(card_group_record.name.to_owned())
                        .or_insert(CardGroup::new(card_group_record, card_group_cards));

                    acc
                },
            );

        let player_state = Game::build_player_state(&record, user_card_groups_cards);

        Ok(Game {
            record,
            communal_card_groups: communal_card_groups,
            player_state,
        })
    }

    pub fn build_player_state(
        game_record: &GameRecord,
        user_card_groups_cards: HashMap<User, HashMap<String, (CardGroupRecord, Vec<Card>)>>,
    ) -> Vec<Player<'a>> {
        user_card_groups_cards
            .into_iter()
            .map(|(user, card_group_details)| {
                let card_groups = card_group_details.into_iter().fold(
                    HashMap::new(),
                    |mut acc, (card_group_name, (card_group_record, cards))| {
                        acc.entry(card_group_name)
                            .or_insert(CardGroup::new(card_group_record, cards));

                        acc
                    },
                );

                Player::new(game_record.id, user, card_groups)
            })
            .collect()
    }

    pub fn turn_count(&self) -> i32 {
        self.record.turn_count
    }

    pub fn player_turn_index(&self) -> i32 {
        self.record.player_turn_index
    }
}

#[graphql_object(context = Context<'a>)]
#[graphql(description = "A game")]
impl<'a> Game<'a> {
    fn id(&self) -> i32 {
        self.record.id
    }

    fn communal_card_groups(&self) -> Vec<&CardGroup> {
        self.communal_card_groups.values().collect()
    }

    fn players(&self) -> Vec<&Player> {
        self.player_state.iter().collect()
    }

    fn player_turn_index(&self) -> i32 {
        self.player_turn_index()
    }

    fn turn_count(&self) -> i32 {
        self.turn_count()
    }
}
