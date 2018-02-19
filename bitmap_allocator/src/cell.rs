#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum CellState { 
    Free,
    Boundary,
    Allocated,
}