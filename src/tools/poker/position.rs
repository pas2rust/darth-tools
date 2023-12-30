#[derive(Clone, Debug)]
pub enum Position {
    EarlyPosition1,  // Under The Gun (UTG)
    EarlyPosition2,  // Under The Gun +1 (UTG+1)
    EarlyPosition3,  // Under The Gun +2 (UTG+2)
    MiddlePosition1, // Middle Position (MP)
    MiddlePosition2, // Middle Position (MP1)
    MiddlePosition3, // Middle Position (MP2)
    LatePosition1,   // Late Position (LP)
    Cutoff,          // Cutoff (CO)
    Button,          // Button (BTN)
    SmallBlind,      // Small Blind (SB)
    BigBlind,        // Big Blind (BB)
}
