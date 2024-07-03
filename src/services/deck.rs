use nanoid::nanoid;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use tonic::{Request, Response, Status};

use crate::cards_proto::card::{Rank, Suit};
use crate::cards_proto::deck_service_server::{DeckService, DeckServiceServer};
use crate::cards_proto::game_state::State;
use crate::cards_proto::{
    Card, CreateGameRequest, CreateGameResponse, Deck, GameState, Hand, Player,
};
use crate::AppContext;

pub struct Service {
    ctx: AppContext,
}

#[tonic::async_trait]
impl DeckService for Service {
    async fn create_game(
        &self,
        request: Request<CreateGameRequest>,
    ) -> Result<Response<CreateGameResponse>, Status> {
        let players = request.get_ref().player_ids.clone();
        let players = players
            .into_iter()
            .map(|player_id| Player {
                player_id,
                hand: Some(Hand { cards: vec![] }),
            })
            .collect::<Vec<Player>>();

        let mut deck = Suit::iter()
            .map(|suit| {
                Rank::iter().map(move |rank| Card {
                    suit: suit as i32,
                    rank: rank as i32,
                })
            })
            .flatten()
            .collect::<Vec<Card>>();
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);
        let deck = Deck { cards: deck };

        let game_state = GameState {
            game_id: nanoid!(),
            state: State::WaitingForPlayers as i32,
            deck: Some(deck),
            players,
        };

        // store it in ctx
        self.ctx
            .game_states
            .insert(game_state.game_id.clone(), game_state.clone());

        Ok(Response::new(CreateGameResponse {
            game_state: Some(game_state),
        }))
    }
}

pub fn deck_service(ctx: AppContext) -> DeckServiceServer<Service> {
    DeckServiceServer::new(Service { ctx })
}
