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
| Some         | Some           | Go     | NotCalled     | Opponent           | No                      |
| Some         | Some           | Go     | Called        | Last Play Opponent | Yes (second go)         |
| Some         | Some           | Go     | PlayContinued | Last Play Opponent | Yes (second go)         |
| Some         | None           | Play   | NotCalled     | Player             | No (player continues)   |
| Some         | None           | Play   | Called        | Player             | No                      |
| Some         | None           | Play   | PlayContinued | Player             | No                      |
| Some         | None           | Go     | NotCalled     | Player             | Yes (opponent no plays) |
| Some         | None           | Go     | Called        | Player             | Yes (second go)         |
| Some         | None           | Go     | PlayContinued | Player             | Yes (second go)         |
| None         | Some           | Play   | NotCalled     | Opponent           | No                      |
| None         | Some           | Play   | Called        | Opponent           | No                      |
| None         | Some           | Play   | PlayContinued | Opponent           | No                      |
| None         | Some           | Go     | NotCalled     | Opponent           | No                      |
| None         | Some           | Go     | Called        | Opponent           | Yes (second go)         |
| None         | Some           | Go     | PlayContinued | Opponent           | Yes (second go)         |
| None         | None           | Play   | NotCalled     | n/a                | Yes (must reset)        |
| None         | None           | Play   | Called        | n/a                | Yes (must reset)        |
| None         | None           | Play   | PlayContinued | n/a                | Yes (must reset)        |
| None         | None           | Go     | NotCalled     | n/a                | Yes (must reset)        |
| None         | None           | Go     | Called        | n/a                | Yes (must reset)        |
| None         | None           | Go     | PlayContinued | n/a                | Yes (must reset)        |
