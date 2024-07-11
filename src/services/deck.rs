use nanoid::nanoid;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use tonic::{Request, Response, Status};

use crate::cards_proto::card::{Rank, Suit};
use crate::cards_proto::deck_service_server::{DeckService, DeckServiceServer};
use crate::cards_proto::game_state::State;
use crate::cards_proto::{
    Card, CreateGameRequest, CreateGameResponse, Deck, GameState, GetGameRequest, GetGameResponse,
    GetGamesRequest, GetGamesResponse, Hand, Player,
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

    async fn get_game(
        &self,
        request: Request<GetGameRequest>,
    ) -> Result<Response<GetGameResponse>, Status> {
        let game_id = request.get_ref().game_id.clone();
        let game_state = self
            .ctx
            .game_states
            .get(&game_id)
            .ok_or_else(|| Status::not_found("Game not found"))?;

        Ok(Response::new(GetGameResponse {
            game_state: Some(game_state.clone()),
        }))
    }

    async fn get_games(
        &self,
        _request: Request<GetGamesRequest>,
    ) -> Result<Response<GetGamesResponse>, Status> {
        let game_ids = self
            .ctx
            .game_states
            .iter()
            .map(|entry| entry.key().clone())
            .collect::<Vec<String>>();

        Ok(Response::new(GetGamesResponse { game_ids }))
    }
}

pub fn deck_service(ctx: AppContext) -> DeckServiceServer<Service> {
    DeckServiceServer::new(Service { ctx })
}
