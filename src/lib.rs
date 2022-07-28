#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    // TODO: your code goes here.
    left: (Box<dyn Agent>, i32), right: (Box<dyn Agent>, i32),
}

impl Game {

    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        // TODO: your code goes here.
        Game {
            left: (left, 0),
            right: (right, 0)
        }
    }

    pub fn left_score(&self) -> i32 {
        // TODO: your code goes here.
        self.left.1
    }

    pub fn right_score(&self) -> i32 {
        // TODO: your code goes here.
        self.right.1
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        // TODO: your code goes here.

        let left_move = self.left.0.cheat_or_cooperate();
        let right_move = self.right.0.cheat_or_cooperate();

        if left_move && right_move {

            self.left.0.change_state(true);
            self.right.0.change_state(true);

            RoundOutcome::BothCheated
        } else if !left_move && right_move {

            self.right.1 += 3;
            self.left.1 -= 1;

            self.left.0.change_state(true);
            self.right.0.change_state(false);

            RoundOutcome::RightCheated
        } else if left_move && !right_move {

            self.right.1 -= 1;
            self.left.1 += 3;

            self.left.0.change_state(false);
            self.right.0.change_state(true);

            RoundOutcome::LeftCheated
        } else {

            self.left.1 += 2;
            self.right.1 += 2;

            self.left.0.change_state(false);
            self.right.0.change_state(false);

            RoundOutcome::BothCooperated
        }
    }

}

////////////////////////////////////////////////////////////////////////////////

pub trait Agent {
    fn cheat_or_cooperate(&self) -> bool;
    fn change_state(&mut self, _opponent_move: bool);
}

#[derive(Default)]
pub struct CheatingAgent {
    // TODO: your code goes here.
}

// TODO: your code goes here.
impl Agent for CheatingAgent {
    fn cheat_or_cooperate(&self) -> bool {
        true
    }
    fn change_state(&mut self, _opponent_move: bool) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {
    // TODO: your code goes here.
}

// TODO: your code goes here.
impl Agent for CooperatingAgent {

    fn cheat_or_cooperate(&self) -> bool {
        false
    }

    fn change_state(&mut self, _opponent_move: bool) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    // TODO: your code goes here.
    is_opponent_already_cheated: bool,
}

// TODO: your code goes here.
impl Agent for GrudgerAgent {

    fn cheat_or_cooperate(&self) -> bool {
        self.is_opponent_already_cheated
    }

    fn change_state(&mut self, _opponent_move: bool) {
        if !self.is_opponent_already_cheated {
            self.is_opponent_already_cheated = _opponent_move;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CopycatAgent {
    // TODO: your code goes here.
    last_opponent_move: bool,
}

// TODO: your code goes here.
impl Agent for CopycatAgent {
    fn cheat_or_cooperate(&self) -> bool {
        self.last_opponent_move
    }

    fn change_state(&mut self, _opponent_move: bool) {
        self.last_opponent_move = _opponent_move;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct DetectiveAgent {
    // TODO: your code goes here.
    number_of_rounds: u32,
    is_opponent_already_cheated: bool,
    last_opponent_move: bool,
}

// TODO: your code goes here.
impl Agent for DetectiveAgent {

    fn cheat_or_cooperate(&self) -> bool {

        match self.number_of_rounds {
            0 => false,
            1 => true,
            2 => false,
            3 => false,
            _ => {
                if !self.is_opponent_already_cheated {
                    true
                } else {
                    self.last_opponent_move
                }
            }
        }
    }

    fn change_state(&mut self, _opponent_move: bool) {

        self.number_of_rounds += 1;

        self.last_opponent_move = _opponent_move;

        if !self.is_opponent_already_cheated {
            self.is_opponent_already_cheated = _opponent_move;
        }

    }
}
