use crate::solves::Solve;

pub enum ProgramSection {
    MainMenu,
    Solve((u32, u32)),
    RunningSolve(Solve),
    About,
    Quit,
}
