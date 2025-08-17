use crate::{Cut, Player};

/// The cuts of the current players. This includes a placeholder cut
/// if the player hasn't made the cut action.
pub type Cuts = [Cut; 2];
