use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ggez::audio::{SoundSource, Source};
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, Image, Mesh, PxScale, Rect, Text};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
use ggez::timer::delta;
use ggez::{Context, GameResult};

use super::Scene;
use crate::door::Door;
use crate::interactable::{Interactable, InteractableType};
use crate::player::Player;
use crate::puzzle::Puzzle;

const DARKNESS_PAN_RATE: f32 = 40f32;
const FLICKER_TIME: [f32; 6] = [1f32, 0.1f32, 0.85f32, 0.07f32, 0.12f32, 0.09f32];
const FLICKER_STATE: [bool; 6] = [true, false, true, false, true, false];
const TEXT_RATE: f32 = 0.3f32;
const TEXT_FAST_RATE: f32 = 0.1f32;
const IN_POD_TEXT_WAIT_TIME: f32 = 1f32;
const GET_OUT_OF_POD_TIME: f32 = 3f32;
const PLAYER_MOVEMENT_SPEED: f32 = 200f32;
const DOOR_EXIT_ENTER_TIME: f32 = 1f32;
const BAD_NEWS_FLICKER_RATE: f32 = 0.08f32;
const BAD_NEWS_OFFSET: f32 = -2800f32;
const BAD_NEWS_GROW_RATE: f32 = 100f32;
const BAD_NEWS_NEW_ROOM_CHANGE: f32 = 150f32;
const SHIP_DRAW_OFFSET: [f32; 2] = [100f32, 380f32];
const SHIP_TRAVEL_TIME: f32 = 14f32;

#[derive(Copy, Clone, PartialEq)]
enum State {
    InPodInDarkness,
    InPodWakeupText,
    GetOutOfPod,
    Investigate,
    EnterDoor(Room),
    ExitDoor,
    InPuzzle(PuzzleID),
    Ending,
}

#[derive(Copy, Clone, PartialEq)]
enum Room {
    StasisPod,
    LeftOfPod,
    MainHallFrontOfPod,
    WindowRightHall,
    LeftHall,
    FarRightHall,
    Computer,
    Final,
}

enum WalkingState {
    Standing,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum DoorIDs {
    LeftOfPod,
    LeftHall,
}

#[derive(Copy, Clone, PartialEq)]
enum DiscoveryState {
    Normal,
    Discovery,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum PuzzleID {
    FarRightHall,
    Computer,
}

pub struct MainScene {
    font: Font,
    player: Rc<RefCell<Player>>,
    finished: bool,
    current_text: Text,
    final_text: String,
    text_sfx: Source,
    music: Source,
    pod_image: Image,
    pod_flicker_image: Image,
    ground_rect: Rect,
    state: State,
    darkness_image: Image,
    darkness_yoffset: f32,
    timer: f32,
    draw_flicker_pod: bool,
    index: usize,
    room: Room,
    walking_state: WalkingState,
    door_image: Image,
    interactables: Vec<Interactable>,
    interact_text: Text,
    doors: Vec<Door>,
    door_text: Text,
    door_sfx: Source,
    // (is_open, is_unlocked)
    door_states: HashMap<DoorIDs, (bool, bool)>,
    earth_image: Image,
    discovery_state: DiscoveryState,
    discovery_music: Source,
    saw_earth: bool,
    window_image: Image,
    error_sfx: Source,
    puzzle_states: HashMap<PuzzleID, bool>,
    puzzle: Option<Puzzle>,
    success_sfx: Source,
    bg_image: Image,
    end_game: bool,
    bad_news_image: Image,
    bad_news_state: usize,
    bad_news_timer: f32,
    bad_news_xoffset: f32,
    bad_news_music: Source,
    bad_news_started: bool,
    escape_ship_image: Image,
    is_dead: bool,
    ending_music: Source,
    escape_ship_2_image: Image,
}

impl MainScene {
    pub fn new(ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Self {
        let mut music = Source::new(ctx, "/music00.ogg").unwrap();
        music.set_repeat(true);
        let mut current_text = Text::new("");
        current_text.set_font(font, PxScale::from(26f32));
        let mut interact_text = Text::new("[E] or Left Click\nto Interact");
        interact_text.set_font(font, PxScale::from(20f32));
        let mut door_text = Text::new("[W] or Right Click\nto enter door");
        door_text.set_font(font, PxScale::from(20f32));

        let door_states = HashMap::new();

        let mut text_sfx = Source::new(ctx, "/text.ogg").unwrap();
        text_sfx.set_pitch(1.4f32);

        let mut bad_news_music = Source::new(ctx, "/bad_news.ogg").unwrap();
        bad_news_music.set_repeat(true);

        Self {
            font,
            player,
            finished: false,
            current_text,
            final_text: String::new(),
            text_sfx,
            music,
            pod_image: Image::new(ctx, "/stasis_pod.png").unwrap(),
            pod_flicker_image: Image::new(ctx, "/stasis_pod_empty.png").unwrap(),
            ground_rect: Rect::new(0f32, 550f32, 800f32, 50f32),
            state: State::InPodInDarkness,
            darkness_image: Image::new(ctx, "/darkness.png").unwrap(),
            darkness_yoffset: 0f32,
            timer: FLICKER_TIME[0],
            draw_flicker_pod: false,
            index: 0usize,
            room: Room::StasisPod,
            walking_state: WalkingState::Standing,
            door_image: Image::new(ctx, "/door.png").unwrap(),
            interactables: Vec::new(),
            interact_text,
            doors: Vec::new(),
            door_text,
            door_sfx: Source::new(ctx, "/door.ogg").unwrap(),
            door_states,
            earth_image: Image::new(ctx, "/earth.png").unwrap(),
            discovery_state: DiscoveryState::Normal,
            discovery_music: Source::new(ctx, "/music03.ogg").unwrap(),
            saw_earth: false,
            window_image: Image::new(ctx, "/window.png").unwrap(),
            error_sfx: Source::new(ctx, "/error_sfx.ogg").unwrap(),
            puzzle_states: HashMap::new(),
            puzzle: None,
            success_sfx: Source::new(ctx, "/success.ogg").unwrap(),
            bg_image: Image::new(ctx, "/bg.png").unwrap(),
            end_game: false,
            bad_news_image: Image::new(ctx, "/bad_news.png").unwrap(),
            bad_news_state: 0,
            bad_news_timer: 0f32,
            bad_news_xoffset: 0f32,
            bad_news_music,
            bad_news_started: false,
            escape_ship_image: Image::new(ctx, "/escape_ship.png").unwrap(),
            is_dead: false,
            ending_music: Source::new(ctx, "/music02.ogg").unwrap(),
            escape_ship_2_image: Image::new(ctx, "/escape_ship2.png").unwrap(),
        }
    }

    pub fn new_boxed(ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Box<Self> {
        Box::new(Self::new(ctx, font, player))
    }

    fn init_room(&mut self, ctx: &Context) {
        match self.room {
            Room::StasisPod => {
                self.current_text = Text::new("A and D or Left and Right or Left Click to move");
                self.current_text.set_font(self.font, PxScale::from(26f32));
                self.darkness_yoffset = -300f32;
                self.interactables.clear();
                self.doors.clear();
            }
            Room::LeftOfPod => {
                self.current_text = Text::new("");
                self.current_text.set_font(self.font, PxScale::from(26f32));
                self.interactables.clear();
                self.interactables.push(Interactable::new(
                    InteractableType::Door(0),
                    430f32,
                    450f32,
                ));
                self.darkness_yoffset = -300f32;
                self.doors.clear();
                self.doors
                    .push(Door::new(false, 300f32, 600f32 - 160f32 - 50f32, 0));
                if let Some((true, _)) = self.door_states.get(&DoorIDs::LeftOfPod) {
                    self.doors[0].set_open(true);
                }
                if self.state == State::ExitDoor {
                    self.player.borrow_mut().x = 300f32 + (96f32 - 64f32) / 2f32;
                }
            }
            Room::MainHallFrontOfPod => {
                self.doors.clear();
                self.interactables.clear();
                self.doors.push(Door::new(
                    false,
                    400f32 - 96f32 / 2f32,
                    600f32 - 160f32 - 50f32,
                    0,
                ));
                if let Some((true, _)) = self.door_states.get(&DoorIDs::LeftOfPod) {
                    self.doors[0].set_open(true);
                }
                self.interactables.push(Interactable::new(
                    InteractableType::LockedDoor(0, false),
                    330f32,
                    450f32,
                ));
                if let Some((_, true)) = self.door_states.get(&DoorIDs::LeftOfPod) {
                    self.interactables[0].set_unlocked(true);
                }
                if !self.door_states.contains_key(&DoorIDs::LeftOfPod) {
                    self.interactables[0].set_unlocked(true);
                }
                if self.state == State::ExitDoor {
                    self.player.borrow_mut().x = 400f32 - 96f32 / 2f32 + (96f32 - 64f32) / 2f32;
                }
                self.darkness_yoffset = -300f32;
            }
            Room::WindowRightHall => {
                self.doors.clear();
                self.interactables.clear();
                self.darkness_yoffset = -470f32;
                if !self.saw_earth {
                    self.saw_earth = true;
                    self.discovery_state = DiscoveryState::Discovery;
                    self.music.stop(ctx);
                    self.discovery_music.play(ctx).unwrap();
                }
            }
            Room::LeftHall => {
                self.doors.clear();
                self.interactables.clear();
                self.doors
                    .push(Door::new(false, 150f32, 600f32 - 160f32 - 50f32, 0));
                if let Some((true, _)) = self.door_states.get(&DoorIDs::LeftHall) {
                    self.doors[0].set_open(true);
                }
                self.interactables.push(Interactable::new(
                    InteractableType::LockedDoor(0, false),
                    120f32,
                    450f32,
                ));
                if let Some((_, true)) = self.door_states.get(&DoorIDs::LeftHall) {
                    self.interactables[0].set_unlocked(true);
                }
                self.darkness_yoffset = -250f32;
                if self.state == State::ExitDoor {
                    self.player.borrow_mut().x = 150f32 + (96f32 - 64f32) / 2f32;
                }
            }
            Room::FarRightHall => {
                self.doors.clear();
                self.interactables.clear();
                self.interactables.push(Interactable::new(
                    InteractableType::Puzzle(PuzzleID::FarRightHall, false),
                    400f32,
                    500f32,
                ));
                if self.puzzle_states.contains_key(&PuzzleID::FarRightHall) {
                    if let Some(true) = self.puzzle_states.get(&PuzzleID::FarRightHall) {
                        self.interactables[0].set_puzzle_cleared(true);
                    }
                }
                self.darkness_yoffset = -450f32;
            }
            Room::Computer => {
                self.doors.clear();
                self.interactables.clear();
                self.doors
                    .push(Door::new(true, 650f32, 600f32 - 160f32 - 50f32, 0));
                self.interactables.push(Interactable::new(
                    InteractableType::Door(0),
                    780f32,
                    450f32,
                ));
                self.interactables.push(Interactable::new(
                    InteractableType::Puzzle(PuzzleID::Computer, false),
                    300f32,
                    400f32,
                ));
                self.interactables[1].set_radius(200f32);
                if self.puzzle_states.contains_key(&PuzzleID::Computer) {
                    if let Some(true) = self.puzzle_states.get(&PuzzleID::Computer) {
                        self.interactables[1].set_puzzle_cleared(true);
                    }
                }
                self.darkness_yoffset = -530f32;
                if self.state == State::ExitDoor {
                    self.player.borrow_mut().x = 650f32 + (96f32 - 64f32) / 2f32;
                }
            }
            Room::Final => {
                self.doors.clear();
                self.interactables.clear();
                self.darkness_yoffset = -500f32;
                self.interactables.push(Interactable::new(
                    InteractableType::Ship,
                    383f32 + SHIP_DRAW_OFFSET[0],
                    141f32 + SHIP_DRAW_OFFSET[1],
                ));
            }
        }
    }

    fn draw_room_arrows(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut draw_left = false;
        let mut draw_right = false;
        match self.room {
            Room::StasisPod => {
                draw_left = true;
            }
            Room::LeftOfPod => {
                draw_right = true;
            }
            Room::MainHallFrontOfPod => {
                draw_left = true;
                draw_right = true;
            }
            Room::WindowRightHall => {
                draw_left = true;
                draw_right = true;
            }
            Room::LeftHall => {
                draw_right = true;
            }
            Room::FarRightHall => {
                draw_left = true;
                if self.end_game {
                    draw_right = true;
                }
            }
            Room::Computer => (),
            Room::Final => {
                draw_left = true;
            }
        }

        if draw_left {
            let mesh = Mesh::from_triangles(
                ctx,
                &[[32f32, 0f32], [32f32, 32f32], [0f32, 16f32]],
                graphics::Color::WHITE,
            )?;
            graphics::draw(ctx, &mesh, DrawParam::new().dest([32f32, 530f32]))?;
        }
        if draw_right {
            let mesh = Mesh::from_triangles(
                ctx,
                &[[0f32, 0f32], [32f32, 16f32], [0f32, 32f32]],
                graphics::Color::WHITE,
            )?;
            graphics::draw(ctx, &mesh, DrawParam::new().dest([800f32 - 64f32, 530f32]))?;
        }

        Ok(())
    }

    fn check_exit_left(&mut self, ctx: &Context) {
        match self.room {
            Room::StasisPod => {
                self.room = Room::LeftOfPod;
                self.player.borrow_mut().x = 800f32 - 70f32 - 64f32;
                self.init_room(ctx);
            }
            Room::LeftOfPod => (),
            Room::MainHallFrontOfPod => {
                self.room = Room::LeftHall;
                self.player.borrow_mut().x = 800f32 - 70f32 - 64f32;
                self.init_room(ctx);
                if self.end_game {
                    self.bad_news_xoffset += BAD_NEWS_NEW_ROOM_CHANGE;
                }
            }
            Room::WindowRightHall => {
                self.room = Room::MainHallFrontOfPod;
                self.player.borrow_mut().x = 800f32 - 70f32 - 64f32;
                self.init_room(ctx);
                if self.end_game {
                    self.bad_news_xoffset += BAD_NEWS_NEW_ROOM_CHANGE;
                }
            }
            Room::LeftHall => (),
            Room::FarRightHall => {
                self.room = Room::WindowRightHall;
                self.player.borrow_mut().x = 800f32 - 70f32 - 64f32;
                self.init_room(ctx);
                if self.end_game {
                    self.bad_news_xoffset += BAD_NEWS_NEW_ROOM_CHANGE;
                }
            }
            Room::Computer => (),
            Room::Final => {
                self.room = Room::FarRightHall;
                self.player.borrow_mut().x = 800f32 - 70f32 - 64f32;
                self.init_room(ctx);
                if self.end_game {
                    self.bad_news_xoffset += BAD_NEWS_NEW_ROOM_CHANGE;
                }
            }
        }
    }

    fn check_exit_right(&mut self, ctx: &Context) {
        match self.room {
            Room::StasisPod => (),
            Room::LeftOfPod => {
                self.room = Room::StasisPod;
                self.player.borrow_mut().x = 70f32;
                self.init_room(ctx);
            }
            Room::MainHallFrontOfPod => {
                self.room = Room::WindowRightHall;
                self.player.borrow_mut().x = 70f32;
                self.init_room(ctx);
                if self.end_game {
                    self.bad_news_xoffset -= BAD_NEWS_NEW_ROOM_CHANGE;
                }
            }
            Room::WindowRightHall => {
                self.room = Room::FarRightHall;
                self.player.borrow_mut().x = 70f32;
                self.init_room(ctx);
                if self.end_game {
                    self.bad_news_xoffset -= BAD_NEWS_NEW_ROOM_CHANGE;
                }
            }
            Room::LeftHall => {
                self.room = Room::MainHallFrontOfPod;
                self.player.borrow_mut().x = 70f32;
                self.init_room(ctx);
                if self.end_game {
                    self.bad_news_xoffset -= BAD_NEWS_NEW_ROOM_CHANGE;
                }
            }
            Room::FarRightHall => {
                if self.end_game {
                    self.room = Room::Final;
                    self.player.borrow_mut().x = 70f32;
                    self.init_room(ctx);
                    self.bad_news_xoffset -= BAD_NEWS_NEW_ROOM_CHANGE;
                }
            }
            Room::Computer => (),
            Room::Final => (),
        }
    }

    fn check_exit_door(&mut self, door_idx: usize) {
        if self.doors.len() > door_idx {
            match self.room {
                Room::StasisPod => (),
                Room::LeftOfPod => {
                    self.state = State::EnterDoor(Room::MainHallFrontOfPod);
                    self.timer = DOOR_EXIT_ENTER_TIME;
                    self.player.borrow_mut().x =
                        self.doors[door_idx].get_x() + (96f32 - 64f32) / 2f32;
                    self.player.borrow_mut().set_walking(true);
                }
                Room::MainHallFrontOfPod => {
                    self.state = State::EnterDoor(Room::LeftOfPod);
                    self.timer = DOOR_EXIT_ENTER_TIME;
                    self.player.borrow_mut().x =
                        self.doors[door_idx].get_x() + (96f32 - 64f32) / 2f32;
                    self.player.borrow_mut().set_walking(true);
                }
                Room::WindowRightHall => (),
                Room::LeftHall => {
                    self.state = State::EnterDoor(Room::Computer);
                    self.timer = DOOR_EXIT_ENTER_TIME;
                    self.player.borrow_mut().x =
                        self.doors[door_idx].get_x() + (96f32 - 64f32) / 2f32;
                    self.player.borrow_mut().set_walking(true);
                    if self.end_game {
                        self.bad_news_xoffset += BAD_NEWS_NEW_ROOM_CHANGE;
                    }
                }
                Room::FarRightHall => (),
                Room::Computer => {
                    self.state = State::EnterDoor(Room::LeftHall);
                    self.timer = DOOR_EXIT_ENTER_TIME;
                    self.player.borrow_mut().x =
                        self.doors[door_idx].get_x() + (96f32 - 64f32) / 2f32;
                    self.player.borrow_mut().set_walking(true);
                    if self.end_game {
                        self.bad_news_xoffset -= BAD_NEWS_NEW_ROOM_CHANGE;
                    }
                }
                Room::Final => (),
            }
        }
    }

    fn use_interactable(&mut self, itype: InteractableType, ctx: &Context) -> GameResult<()> {
        match itype {
            InteractableType::Door(id) => {
                match self.room {
                    Room::StasisPod => (),
                    Room::LeftOfPod => {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            self.door_states.entry(DoorIDs::LeftOfPod)
                        {
                            e.insert((self.doors[id].toggle_open(), true));
                        } else {
                            self.door_states.get_mut(&DoorIDs::LeftOfPod).unwrap().0 =
                                self.doors[id].toggle_open();
                        }
                    }
                    Room::MainHallFrontOfPod => {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            self.door_states.entry(DoorIDs::LeftOfPod)
                        {
                            e.insert((self.doors[id].toggle_open(), true));
                        } else {
                            self.door_states.get_mut(&DoorIDs::LeftOfPod).unwrap().0 =
                                self.doors[id].toggle_open();
                        }
                    }
                    Room::WindowRightHall => (),
                    Room::LeftHall => (),
                    Room::FarRightHall => (),
                    Room::Computer => {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            self.door_states.entry(DoorIDs::LeftHall)
                        {
                            e.insert((self.doors[id].toggle_open(), true));
                        } else {
                            self.door_states.get_mut(&DoorIDs::LeftHall).unwrap().0 =
                                self.doors[id].toggle_open();
                        }
                    }
                    Room::Final => (),
                }
                self.door_sfx.play(ctx)?;
            }
            InteractableType::LockedDoor(id, unlocked) => match self.room {
                Room::StasisPod | Room::LeftOfPod | Room::WindowRightHall => (),
                Room::MainHallFrontOfPod => {
                    if unlocked {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            self.door_states.entry(DoorIDs::LeftOfPod)
                        {
                            e.insert((self.doors[id].toggle_open(), true));
                        } else {
                            self.door_states.get_mut(&DoorIDs::LeftOfPod).unwrap().0 =
                                self.doors[id].toggle_open();
                        }
                        self.door_sfx.play(ctx)?;
                    } else {
                        self.error_sfx.play(ctx)?;
                    }
                }
                Room::LeftHall => {
                    if unlocked {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            self.door_states.entry(DoorIDs::LeftHall)
                        {
                            e.insert((self.doors[id].toggle_open(), true));
                        } else {
                            self.door_states.get_mut(&DoorIDs::LeftHall).unwrap().0 =
                                self.doors[id].toggle_open();
                        }
                        self.door_sfx.play(ctx)?;
                    } else {
                        self.error_sfx.play(ctx)?;
                    }
                }
                Room::FarRightHall => (),
                Room::Computer => (),
                Room::Final => (),
            },
            InteractableType::Puzzle(id, cleared) => match self.room {
                Room::StasisPod
                | Room::LeftOfPod
                | Room::MainHallFrontOfPod
                | Room::LeftHall
                | Room::WindowRightHall => (),
                Room::FarRightHall => {
                    if !cleared {
                        self.state = State::InPuzzle(id);
                        self.puzzle = Some(Puzzle::new(id, self.font));
                    }
                }
                Room::Computer => {
                    if !cleared {
                        self.state = State::InPuzzle(id);
                        self.puzzle = Some(Puzzle::new(id, self.font));
                    }
                }
                Room::Final => (),
            },
            InteractableType::Ship => {
                self.state = State::Ending;
                self.bad_news_music.stop(ctx);
                self.ending_music.play(ctx)?;
                self.timer = 0f32;
            }
        }
        Ok(())
    }

    fn draw_room(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.room {
            Room::StasisPod => {
                graphics::draw(
                    ctx,
                    &self.pod_flicker_image,
                    DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
                )?;
            }
            Room::LeftOfPod => {}
            Room::MainHallFrontOfPod => {}
            Room::WindowRightHall => {
                graphics::draw(
                    ctx,
                    &self.window_image,
                    DrawParam::new().dest([800f32 / 5f32, 600f32 / 5f32]),
                )?;
            }
            Room::LeftHall => {}
            Room::FarRightHall => {}
            Room::Computer => (),
            Room::Final => {
                graphics::draw(
                    ctx,
                    &self.escape_ship_image,
                    DrawParam::new().dest(SHIP_DRAW_OFFSET),
                )?;
            }
        }
        for door in &self.doors {
            door.draw(ctx, &self.door_image)?;
        }
        for interactable in &self.interactables {
            interactable.draw(ctx)?;
        }
        Ok(())
    }

    fn handle_solved_puzzle(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            State::InPodInDarkness
            | State::InPodWakeupText
            | State::GetOutOfPod
            | State::Investigate
            | State::EnterDoor(_)
            | State::ExitDoor
            | State::Ending => unreachable!("Cannot solve puzzle from invalid state"),
            State::InPuzzle(id) => match id {
                PuzzleID::FarRightHall => {
                    self.puzzle_states.insert(id, true);
                    self.puzzle = None;
                    self.interactables[0].set_puzzle_cleared(true);
                    self.door_states.insert(DoorIDs::LeftHall, (false, true));
                    self.door_states.insert(DoorIDs::LeftOfPod, (false, false));
                }
                PuzzleID::Computer => {
                    self.puzzle_states.insert(id, true);
                    self.puzzle = None;
                    self.interactables[1].set_puzzle_cleared(true);
                    self.end_game = true;
                }
            },
        }
        self.success_sfx.play(ctx)?;
        Ok(())
    }

    fn draw_bad_news(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(
            ctx,
            &self.bad_news_image,
            DrawParam::new()
                .src(Rect::new(
                    0f32,
                    self.bad_news_state as f32 * 600f32 / 1800f32,
                    1f32,
                    1f32 / 3f32,
                ))
                .dest([self.bad_news_xoffset + BAD_NEWS_OFFSET, 0f32]),
        )
    }
}

impl EventHandler for MainScene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = delta(ctx).as_secs_f32();
        if self.is_dead {
            self.bad_news_timer += dt;
            if self.bad_news_timer > BAD_NEWS_FLICKER_RATE {
                self.bad_news_timer -= BAD_NEWS_FLICKER_RATE;
                self.bad_news_state = (self.bad_news_state + 1) % 3;
            }
            return Ok(());
        }
        match &self.state {
            State::InPodInDarkness => {
                let mut player = self.player.borrow_mut();
                player.x = 520f32;
                player.y = 350f32;
                player.rot = 0.78f32;
                self.timer -= dt;
                if self.timer <= 0f32 {
                    self.draw_flicker_pod = FLICKER_STATE[self.index];
                    self.index = (self.index + 1) % 6;
                    self.timer = FLICKER_TIME[self.index];
                }
                if self.darkness_yoffset > -300f32 {
                    self.darkness_yoffset -= dt * DARKNESS_PAN_RATE;
                } else {
                    self.state = State::InPodWakeupText;
                    self.timer = TEXT_RATE;
                    self.final_text = "What.. Where am I?..".chars().rev().collect::<String>();
                }
            }
            State::InPodWakeupText => {
                if !self.final_text.is_empty() {
                    self.timer -= dt;
                    if self.timer <= 0f32 {
                        self.timer = TEXT_RATE;
                        self.current_text.fragments_mut()[0]
                            .text
                            .push(self.final_text.pop().unwrap());
                        self.text_sfx.play(ctx)?;
                        if self.final_text.is_empty() {
                            self.timer = IN_POD_TEXT_WAIT_TIME;
                        }
                    }
                } else {
                    self.timer -= dt;
                    if self.timer <= 0f32 {
                        self.timer = 0f32;
                    }
                }
            }
            State::GetOutOfPod => {
                self.timer -= dt;
                if self.timer > 0f32 {
                    let mut player = self.player.borrow_mut();
                    let lerp = self.timer / GET_OUT_OF_POD_TIME;
                    player.x = (520f32 * lerp) + (500f32 * (1f32 - lerp));
                    player.y = (350f32 * lerp) + (430f32 * (1f32 - lerp));
                    player.rot = (0.78f32 * lerp) + (0f32 * (1f32 - lerp));
                } else if self.timer <= 0f32 {
                    self.state = State::Investigate;
                    self.music.play(ctx)?;
                    self.init_room(ctx);
                }
            }
            State::Investigate => match self.walking_state {
                WalkingState::Standing => {
                    self.player.borrow_mut().set_walking(false);
                }
                WalkingState::Left => {
                    self.player.borrow_mut().x -= dt * PLAYER_MOVEMENT_SPEED;
                    if self.player.borrow().x <= 0f32 {
                        self.player.borrow_mut().x = 0f32;
                        self.walking_state = WalkingState::Standing;
                        self.player.borrow_mut().set_walking(false);
                        self.check_exit_left(ctx);
                    } else {
                        self.player.borrow_mut().set_walking(true);
                        self.player.borrow_mut().set_xflip(true);
                    }
                }
                WalkingState::Right => {
                    self.player.borrow_mut().x += dt * PLAYER_MOVEMENT_SPEED;
                    if self.player.borrow().x + 64f32 >= 800f32 {
                        self.player.borrow_mut().x = 800f32 - 64f32;
                        self.walking_state = WalkingState::Standing;
                        self.player.borrow_mut().set_walking(false);
                        self.check_exit_right(ctx);
                    } else {
                        self.player.borrow_mut().set_walking(true);
                        self.player.borrow_mut().set_xflip(false);
                    }
                }
            },
            State::EnterDoor(room) => {
                self.timer -= dt;
                if self.timer <= 0f32 {
                    match room {
                        Room::StasisPod => unreachable!("Cannot enter stasis room via door"),
                        r => self.room = *r,
                    }
                    self.state = State::ExitDoor;
                    self.timer = DOOR_EXIT_ENTER_TIME;
                    self.init_room(ctx);
                    self.player.borrow_mut().color.a = 0f32;
                } else {
                    self.player.borrow_mut().color.a = self.timer / DOOR_EXIT_ENTER_TIME;
                }
            }
            State::ExitDoor => {
                self.timer -= dt;
                if self.timer <= 0f32 {
                    self.state = State::Investigate;
                    self.player.borrow_mut().set_walking(false);
                    self.player.borrow_mut().color.a = 1f32;
                } else {
                    self.player.borrow_mut().color.a = 1f32 - self.timer / DOOR_EXIT_ENTER_TIME;
                }
            }
            State::InPuzzle(_) => {
                if let Some(puzzle) = &mut self.puzzle {
                    if puzzle.is_solved() {
                        self.handle_solved_puzzle(ctx)?;
                    } else if puzzle.is_abort() {
                        self.puzzle = None;
                        self.state = State::Investigate;
                    } else {
                        puzzle.update(ctx)?;
                    }
                } else {
                    self.state = State::Investigate;
                }
            }
            State::Ending => {
                // TODO update ending
                self.timer += dt;
                if self.timer >= SHIP_TRAVEL_TIME {
                    self.timer = SHIP_TRAVEL_TIME;
                }
                return Ok(());
            }
        }
        self.player.borrow_mut().update(ctx)?;
        if self.discovery_state == DiscoveryState::Discovery && self.discovery_music.stopped() {
            self.discovery_state = DiscoveryState::Normal;
            self.music.play(ctx)?;
        }
        if self.end_game {
            if !self.bad_news_started {
                self.bad_news_started = true;
                self.music.stop(ctx);
                self.bad_news_music.play(ctx)?;
            }
            self.bad_news_xoffset += dt * BAD_NEWS_GROW_RATE;
            if self.bad_news_xoffset >= 2700f32 {
                self.is_dead = true;
            }
            self.bad_news_timer += dt;
            if self.bad_news_timer > BAD_NEWS_FLICKER_RATE {
                self.bad_news_timer -= BAD_NEWS_FLICKER_RATE;
                self.bad_news_state = (self.bad_news_state + 1) % 3;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.state == State::Ending {
            // TODO draw ending
            graphics::draw(ctx, &self.earth_image, DrawParam::new())?;
            let lerp = self.timer / SHIP_TRAVEL_TIME;
            graphics::draw(
                ctx,
                &self.escape_ship_2_image,
                DrawParam::new()
                    .dest([
                        (-5f32 * (1f32 - lerp)) + 500f32 * lerp,
                        (-5f32 * (1f32 - lerp)) + 200f32 * lerp,
                    ])
                    .rotation(-0.2f32 * (1f32 - lerp) + 0.7f32 * lerp)
                    .scale([
                        0.7f32 * (1f32 - lerp) + 0.001f32 * lerp,
                        0.7f32 * (1f32 - lerp) + 0.001f32 * lerp,
                    ]),
            )?;
            return Ok(());
        }

        {
            graphics::draw(ctx, &self.bg_image, DrawParam::new())?;
            let ground_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                self.ground_rect,
                Color::from_rgb(0x49, 0x49, 0x49),
            )?;
            graphics::draw(ctx, &ground_mesh, DrawParam::new())?;
        }

        if self.is_dead {
            graphics::draw(
                ctx,
                &self.darkness_image,
                DrawParam::new().dest([0f32, -80f32]),
            )?;
            return Ok(());
        }

        match self.state {
            State::InPodInDarkness => {
                if self.draw_flicker_pod {
                    graphics::draw(
                        ctx,
                        &self.pod_flicker_image,
                        DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
                    )?;
                } else {
                    graphics::draw(
                        ctx,
                        &self.pod_image,
                        DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
                    )?;
                }
            }
            State::InPodWakeupText | State::GetOutOfPod => {
                graphics::draw(
                    ctx,
                    &self.pod_flicker_image,
                    DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
                )?;
            }
            State::Investigate => {
                self.draw_room(ctx)?;
            }
            State::EnterDoor(_) => {
                self.draw_room(ctx)?;
            }
            State::ExitDoor => {
                self.draw_room(ctx)?;
            }
            State::InPuzzle(_) => (),
            State::Ending => (),
        }

        self.player.borrow_mut().draw(ctx)?;

        graphics::draw(
            ctx,
            &self.darkness_image,
            DrawParam::new().dest([0f32, self.darkness_yoffset]),
        )?;

        if self.end_game {
            self.draw_bad_news(ctx)?;
        }

        match self.state {
            State::InPodInDarkness => (),
            State::InPodWakeupText => {
                graphics::draw(
                    ctx,
                    &self.current_text,
                    DrawParam::new().dest([100f32, 100f32]),
                )?;
            }
            State::GetOutOfPod => (),
            State::Investigate => {
                match self.room {
                    Room::StasisPod => {
                        graphics::draw(
                            ctx,
                            &self.current_text,
                            DrawParam::new()
                                .dest([100f32, 100f32])
                                .color(graphics::Color::WHITE),
                        )?;
                    }
                    Room::LeftOfPod => {}
                    Room::MainHallFrontOfPod => {}
                    Room::WindowRightHall => (),
                    Room::LeftHall => (),
                    Room::FarRightHall => (),
                    Room::Computer => (),
                    Room::Final => (),
                }

                for interactable in &self.interactables {
                    if interactable.is_within_range(
                        self.player.borrow().x + 32f32,
                        self.player.borrow().y + 64f32,
                    ) {
                        let text_offset = (self.interact_text.width(ctx) / 2.0f32).floor();
                        graphics::draw(
                            ctx,
                            &self.interact_text,
                            DrawParam::new().dest([
                                interactable.get_x() - text_offset,
                                interactable.get_y() - 50f32,
                            ]),
                        )?;
                    }
                }
                for door in &self.doors {
                    if door.is_within_range(
                        self.player.borrow().x + 32f32,
                        self.player.borrow().y + 64f32,
                    ) && door.get_open()
                    {
                        let text_offset = (self.door_text.width(ctx) / 2.0f32).floor();
                        graphics::draw(
                            ctx,
                            &self.door_text,
                            DrawParam::new()
                                .dest([door.get_center_x() - text_offset, door.get_y() - 15f32]),
                        )?;
                    }
                }

                self.draw_room_arrows(ctx)?;
            }
            State::EnterDoor(_) | State::ExitDoor => (),
            State::InPuzzle(_) => {
                if let Some(puzzle) = &mut self.puzzle {
                    puzzle.draw(ctx)?;
                }
            }
            State::Ending => (),
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match self.state {
            State::InPodInDarkness => (),
            State::InPodWakeupText => {
                if self.final_text.is_empty() && self.timer <= 0f32 {
                    self.state = State::GetOutOfPod;
                    self.timer = GET_OUT_OF_POD_TIME;
                } else {
                    self.timer = 0f32;
                }
            }
            State::GetOutOfPod => (),
            State::Investigate => {
                if button == MouseButton::Left {
                    let mut itype: Option<InteractableType> = None;
                    for interactable in &self.interactables {
                        if interactable.is_within_range(
                            self.player.borrow().x + 32f32,
                            self.player.borrow().y + 64f32,
                        ) && interactable.is_within_range(x, y)
                        {
                            itype = Some(interactable.get_type());
                            break;
                        }
                    }
                    if let Some(it) = itype {
                        self.use_interactable(it, ctx).unwrap();
                    } else if self.player.borrow().x > x {
                        self.walking_state = WalkingState::Left;
                    } else if self.player.borrow().x + 64f32 < x {
                        self.walking_state = WalkingState::Right;
                    }
                } else if button == MouseButton::Right {
                    let mut door_idx: Option<usize> = None;
                    for door in &self.doors {
                        if door.get_open()
                            && door.is_within_range(
                                self.player.borrow().x + 32f32,
                                self.player.borrow().y + 64f32,
                            )
                            && door.is_within_range(x, y)
                        {
                            door_idx = Some(door.get_id());
                            break;
                        }
                    }
                    if let Some(idx) = door_idx {
                        self.check_exit_door(idx);
                    }
                }
            }
            State::EnterDoor(_) | State::ExitDoor => (),
            State::InPuzzle(_) => {
                if let Some(puzzle) = &mut self.puzzle {
                    if button == MouseButton::Left {
                        puzzle.handle_click(ctx, x, y);
                    }
                }
            }
            State::Ending => (),
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        match self.state {
            State::InPodInDarkness => (),
            State::InPodWakeupText => (),
            State::GetOutOfPod => (),
            State::Investigate => {
                if button == MouseButton::Left {
                    self.walking_state = WalkingState::Standing;
                }
            }
            State::EnterDoor(_) | State::ExitDoor => (),
            State::InPuzzle(_) => {}
            State::Ending => (),
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match self.state {
            State::InPodInDarkness => (),
            State::InPodWakeupText => {
                if self.final_text.is_empty() && self.timer <= 0f32 {
                    self.state = State::GetOutOfPod;
                    self.timer = GET_OUT_OF_POD_TIME;
                    self.current_text.fragments_mut()[0].text.clear();
                } else {
                    self.timer = 0f32;
                }
            }
            State::GetOutOfPod => (),
            State::Investigate => {
                if keycode == KeyCode::A || keycode == KeyCode::Left {
                    if self.player.borrow().x > 0f32 {
                        self.walking_state = WalkingState::Left;
                    }
                } else if keycode == KeyCode::D || keycode == KeyCode::Right {
                    if self.player.borrow().x + 64f32 < 800f32 {
                        self.walking_state = WalkingState::Right;
                    }
                } else if keycode == KeyCode::E {
                    let mut itype: Option<InteractableType> = None;
                    for interactable in &self.interactables {
                        if interactable.is_within_range(
                            self.player.borrow().x + 32f32,
                            self.player.borrow().y + 64f32,
                        ) {
                            itype = Some(interactable.get_type());
                            break;
                        }
                    }
                    if let Some(it) = itype {
                        self.use_interactable(it, ctx).unwrap();
                    }
                } else if keycode == KeyCode::W {
                    let mut door_idx: Option<usize> = None;
                    for door in &self.doors {
                        if door.get_open()
                            && door.is_within_range(
                                self.player.borrow().x + 32f32,
                                self.player.borrow().y + 64f32,
                            )
                        {
                            door_idx = Some(door.get_id());
                            break;
                        }
                    }
                    if let Some(idx) = door_idx {
                        self.check_exit_door(idx);
                    }
                }
            }
            State::EnterDoor(_) | State::ExitDoor => (),
            State::InPuzzle(_) => {
                if let Some(puzzle) = &mut self.puzzle {
                    puzzle.handle_key(ctx, keycode);
                }
            }
            State::Ending => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match self.state {
            State::InPodInDarkness => (),
            State::InPodWakeupText => (),
            State::GetOutOfPod => (),
            State::Investigate => {
                if keycode == KeyCode::A
                    || keycode == KeyCode::Left
                    || keycode == KeyCode::D
                    || keycode == KeyCode::Right
                {
                    self.walking_state = WalkingState::Standing;
                }
            }
            State::EnterDoor(_) | State::ExitDoor => (),
            State::InPuzzle(_) => {}
            State::Ending => (),
        }
    }
}

impl Scene for MainScene {
    fn finished(&self) -> bool {
        self.finished
    }
}
