/// A DTO representing a Card. The value is the "cid" (Card identifier), e.g. "AS", "QD".
pub struct CardDTO(String);

impl std::fmt::Display for CardDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
