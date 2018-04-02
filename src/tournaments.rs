/*
use std::rc::Rc;




struct Team{
    lead: u64,
    players: Vec<u64>,
    name: String,
    status: TeamStatus,
}

enum TeamStatus{
    None,
    Lose(Rc<CellPair>),
    Wait(Rc<CellPair>),
    Won,
    Play(Rc<CellPair>),
    Disqualified(Rc<CellPair>),
}

struct Tourney{
    teams: Vec<Rc<Team>>,
    grid: Grid,
}

struct Grid{
    pairs: Vec<CellPair>,
}

struct CellPair{
    id: u16,
    cell1: CellStatus,
    cell2: CellStatus,
    next: Option<Rc<CellPair>>,
    prev: Option<Rc<CellPair>>,
}

enum CellStatus{
    Won(Rc<Team>),
    Lose(Rc<Team>),
    Play(Rc<Team>),
    Empty,
    Disqualified(Rc<Team>),
}

impl Tourney{
    fn team_add(&mut self, team: Team){
        &self.teams.push(Rc::from(team));
    }

    fn team_find(&self, player: u64) -> Option<Rc<Team>>{
        for team in &self.teams{
            for id in team.players{
                if id == player{
                    return Some(team.clone());
                }
            }
        }
        return None;
    }


}


fn tourney(){

}
*/