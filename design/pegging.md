| Go Status     | Action | Total (1) | Next Player | Next Go Status | Comment                                 | Reset? | 
|---------------|--------|-----------|-------------|----------------|-----------------------------------------|--------|
| NotCalled     | Play   |    31     | Opponent    |   - (3)        | Two points + standard pegging to player | Yes    | 
| NotCalled     | Play   |  < 31     | Opponent    | NotCalled      | Standard pegging to player              | No     | 
| NotCalled     | Go     |  - (2)    | Opponent    | Called         | Opponent continues play                 | No     | 
| Called        | Play   |    31     | Opponent    |   - (3)        | Two points + standard pegging to player | Yes    | 
| Called        | Play   |  < 31     | Player      | PlayContinued  | Standard pegging to player              | No     | 
| Called        | Go     |  - (2)    | Opponent    |   - (3)        | One point to opponent                   | Yes    | 
| PlayContinued | Play   |    31     | Opponent    |   - (3)        | Two points + standard pegging to player | Yes    | 
| PlayContinued | Play   |  < 31     | Player      | PlayContinued  | Standard pegging to player              | No     | 
| PlayContinued | Go     |  - (2)    | Opponent    |   - (3)        | One point to player                     | Yes    | 

(1) Including played card
(2) Will always be <31
(3) Reset

| Player Plays | Opponent Plays | Action | Go Status     | Next Player        | New Play (Reason)?      |
| ------------ | -------------- | ------ | ------------- | ------------------ | ----------------------- |
| Some         | Some           | Play   | NotCalled     | Opponent           | No                      |
| Some         | Some           | Play   | Called        | Player             | No                      |
| Some         | Some           | Play   | PlayContinued | Player             | No                      |
| Some         | Some           | Pass   | NotCalled     | Opponent           | No                      |
| Some         | Some           | Pass   | Called        | Last Play Opponent | Yes (second pass)       |
| Some         | Some           | Pass   | PlayContinued | Last Play Opponent | Yes (second pass)       |
| Some         | None           | Play   | NotCalled     | Player             | No (player continues)   |
| Some         | None           | Play   | Called        | Player             | No                      |
| Some         | None           | Play   | PlayContinued | Player             | No                      |
| Some         | None           | Pass   | NotCalled     | Player             | Yes (opponent no plays) |
| Some         | None           | Pass   | Called        | Player             | Yes (second pass)       |
| Some         | None           | Pass   | PlayContinued | Player             | Yes (second pass)       |
| None         | Some           | Play   | NotCalled     | Opponent           | No                      |
| None         | Some           | Play   | Called        | Opponent           | No                      |
| None         | Some           | Play   | PlayContinued | Opponent           | No                      |
| None         | Some           | Pass   | NotCalled     | Opponent           | No                      |
| None         | Some           | Pass   | Called        | Opponent           | Yes (second pass)       |
| None         | Some           | Pass   | PlayContinued | Opponent           | Yes (second pass)       |
| None         | None           | Play   | NotCalled     | n/a                | Yes (must reset)        |
| None         | None           | Play   | Called        | n/a                | Yes (must reset)        |
| None         | None           | Play   | PlayContinued | n/a                | Yes (must reset)        |
| None         | None           | Pass   | NotCalled     | n/a                | Yes (must reset)        |
| None         | None           | Pass   | Called        | n/a                | Yes (must reset)        |
| None         | None           | Pass   | PlayContinued | n/a                | Yes (must reset)        |
