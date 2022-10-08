package com.nigeleke.cribbage.model

import com.nigeleke.cribbage.util.*
import Cards.*
import GameState.*
import scala.util.Random

/** A Cribbage game in a provided [GameState].
  * @param state
  *   The current state for this game.
  */
case class Game(state: GameState)

object Game:
  /** Create a new game.
    * @return
    *   The game in the [GameState.Starting], with no players.
    */
  def create = Game(Starting(Set.empty))

extension (game: Game)

  /** @return
    *   The current set of players in a game,
    */
  def players: Set[Player] =
    game.state match
      case Starting(players)                 => players
      case Discarding(_, scores, _, _, _, _) => scores.keySet
      case Playing(scores, _, _, _, _, _, _) => scores.keySet
      case Scoring(scores, _, _, _, _, _)    => scores.keySet
      case Finished(scores)                  => scores.keySet

  /** Get the opponent for the given player.
    * @param player
    *   The player.
    * @return
    *   The player's opponent.
    * @throws RuntimeException
    *   If the key player isn't in the game, or no opponent exists.
    */
  def opponent(player: Player): Player =
    require(game.players.contains(player))
    require(game.players.size == playersInGame)
    val others = players.filterNot(_ == player)
    others.head

  /** Get the Score for the given player,
    * @param player
    *   The player whose Score is required.
    * @return
    *   The player's current score, or zero if the player isn't in the game.
    */
  def scores(player: Player): Score =
    game.state match
      case Starting(_)                       => Score.zero
      case Discarding(_, scores, _, _, _, _) => scores.get(player).getOrElse(Score.zero)
      case Playing(scores, _, _, _, _, _, _) => scores.get(player).getOrElse(Score.zero)
      case Scoring(scores, _, _, _, _, _)    => scores.get(player).getOrElse(Score.zero)
      case Finished(scores)                  => scores.get(player).getOrElse(Score.zero)

  private def withState(state: GameState) = Right(game.copy(state = state))
  private def withError(message: String)  = Left(message)

  /** Add player to the current game.
    * @param player
    *   The player to be added.
    * @return
    *   A validated game with the player added or an error when the player already exists in the
    *   game or no more players are required.
    */
  // format: off
  def addPlayer(player: Player): Either[String, Game] = game.state match
    case Starting(players) if players.contains(player)     => withError(s"Player $player already added")
    case Starting(players) if players.size == playersInGame => withError(s"Cannot add player $player")
    case s @ Starting(players)                             => withState(s.copy(players = players + player))
    case _                                                 => withError(s"Cannot add player $player")
  // format: on

  /** Start the game.
    * @return
    *   A validated game, with dealer selected and ready to discard, or an error if the game cam't
    *   be started, i.e. if not enough players, or already started.
    */
  def start: Either[String, Game] = game.state match
    case Starting(players) if players.size == playersInGame =>
      val scores = players.map((_, Score.zero)).toMap
      val cut    = Random.shuffle(players)
      withState(newDiscarding(scores, cut.head, cut.last))
    case _                                                  => withError("Cannot start game")

  private def newDiscarding(scores: Map[Player, Score], dealer: Player, pone: Player): Discarding =
    val (deck, hands) = shuffledDeck.deal(playersInGame, cardsPerHand)
    val dealtHands    = players.zip(hands).toMap
    val crib          = emptyCrib
    Discarding(deck, scores, dealtHands, dealer, pone, crib)

  /** Disard a player's card(s) into the Crib.
    * @param player
    *   The player whose cards are being discarded.
    * @param discards
    *   The discards.
    * @return
    *   A validated game with the discards removed from the player's hand and added to the crib, or
    *   an error if an invalid player, or they don't hold the cards, or they're discarding too many
    *   cards, or the game is not in a state for discarding.
    */
  def discardCribCards(player: Player, discards: Seq[Card]): Either[String, Game] =
    game.state match
      case d @ Discarding(_, _, hands, _, _, crib) =>
        lazy val playerHand = hands(player)
        val canDiscard      =
          players.contains(player) &&
            playerHand.size - discards.size >= 4 &&
            discards.forall(playerHand.contains)
        if (canDiscard)
          val updatedHand  = playerHand.removeAll(discards)
          val updatedHands = hands.updated(player, updatedHand)
          val updatedCrib  = crib.addAll(discards)
          withState(d.copy(hands = updatedHands, crib = updatedCrib))
        else withError("Cannot discard cards")
      case _                                       => withError("Cannot discard cards")

  /** Start the Play phase, defining the next player to play. Two points are scored for the dealer
    * if the cut returned a Jack. A Finished game is returned if that resulted in a win.
    * @param nextPlayer
    *   The next player to play.
    * @return
    *   A validated game, with the deck cut for the dealer, or an error if the player isn't valid,
    *   or not enough cards are in the crib.
    */
  def startPlay(nextPlayer: Player): Either[String, Game] =
    game.state match
      case Discarding(deck, scores, hands, dealer, pone, crib) =>
        val canStartPlay = players.contains(nextPlayer) && crib.isFull
        if (canStartPlay)
          val (_, cut)      = deck.cut
          val updatedScore  = scores(dealer).add(Scorer.forCut(cut).total)
          val updatedScores = scores.updated(dealer, updatedScore)
          val plays         = Plays(nextPlayer)
          if updatedScore.points >= winningPoints
          then withState(Finished(updatedScores))
          else withState(Playing(updatedScores, hands, dealer, pone, crib, cut, plays))
        else withError("Cannot start play")
      case _                                                   => withError("Cannot start play")

  /** Allow the player to lay their next card in the Play phase. A Finished game is returned if the
    * play scored and resulted in a win.
    * @param player
    *   The player laying a card.
    * @param card
    *   Their card to be laid.
    * @return
    *   A validated game, with the played card laid and scored, or an error if it's not the player's
    *   turn to play, they don't hold the card or the running total is more than 31.
    */
  def playCard(player: Player, card: Card): Either[String, Game] =
    game.state match
      case p @ Playing(scores, hands, _, _, _, _, plays) =>
        val currentTotal = plays.runningTotal
        val canPlay      =
          plays.nextPlayer == player &&
            hands(player).contains(card) &&
            hands(player).mustPlay(currentTotal)
        if (canPlay)
          val updatedHand   = hands(player).remove(card)
          val updatedHands  = hands.updated(player, updatedHand)
          val currentPlay   = plays.inPlay :+ Plays.Laid(player, card)
          val nextPlayer    = game.opponent(player)
          val updatedPlays  = plays.copy(nextPlayer = nextPlayer, inPlay = currentPlay)
          val updatedScore  = scores(player).add(Scorer.forPlay(currentPlay).total)
          val updatedScores = scores.updated(player, updatedScore)
          if updatedScore.points >= winningPoints
          then withState(Finished(updatedScores))
          else withState(p.copy(scores = updatedScores, hands = updatedHands, plays = updatedPlays))
        else withError("Cannot play card")
      case _                                             => withError("Cannot play card")

  /** Allow a player to pass their next lay in the Play phase. The game plays are reset if both
    * players have passed. A Finished game is returned if end of play scoring results in a win.
    * @param player
    *   The player to pass.
    * @return
    *   A validated game, with scores updated if both players have passed, or an error if it's not
    *   the player next turn to go, or if they have a card that they can lay.
    */
  def pass(player: Player): Either[String, Game] =
    game.state match
      case p @ Playing(scores, hands, _, _, _, _, plays) =>
        val currentTotal = plays.runningTotal
        val canPass      = plays.nextPlayer == player &&
          hands(player).mustPass(currentTotal)
        if (canPass)
          val currentPlay   = plays.inPlay :+ Plays.Pass(player)
          val nextPlayer    = game.opponent(player)
          val passedPlayers = plays.passedPlayers + player
          val updatedPlays0 = plays.copy(nextPlayer = nextPlayer, inPlay = currentPlay)
          val resetPlays    = Plays(nextPlayer, Seq.empty, plays.played ++ currentPlay)
          val updatedScore  = scores(player).add(Scorer.forEndPlay(updatedPlays0).total)
          val updatedScores = scores.updated(player, updatedScore)
          val updatedPlays1 = if updatedPlays0.passCount == 2 then resetPlays else updatedPlays0
          if updatedScore.points >= winningPoints
          then withState(Finished(updatedScores))
          else withState(p.copy(scores = updatedScores, plays = updatedPlays1))
        else withError("Cannot pass")
      case _                                             => withError("Cannot pass")

  /** Regather laid cards into the Player's Hands if all plays have been completed.
    * @return
    *   A validated game ready for scoring, or an error if there are still cards to be played.
    */
  def regatherPlays: Either[String, Game] =
    game.state match
      case p @ Playing(scores, hands, dealer, pone, crib, cut, plays) =>
        val canRegather  = hands.forall(_._2.isEmpty)
        val updatedHands = (plays.played ++ plays.inPlay)
          .collect { case p: Plays.Laid => p }
          .groupBy(_.player)
          .view
          .mapValues(laids => handOf(laids.map(_.card)))
          .toMap
        if canRegather
        then withState(Scoring(scores, updatedHands, dealer, pone, crib, cut))
        else withError("Cannot regather")
      case _                                                          => withError("Cannot regather")

  /** Score the pone's hand. Discard their cards once scored.
    * @returns
    *   A validated game with the pone's hand scored, or an error if the game isn't in a state for
    *   scoring.
    */
  def scorePoneHand: Either[String, Game] =
    game.state match
      case s @ Scoring(scores, hands, _, pone, _, cut) =>
        val updatedScore  = scores(pone).add(Scorer.forCards(hands(pone), cut).total)
        val updatedScores = scores.updated(pone, updatedScore)
        if updatedScore.points >= winningPoints
        then withState(Finished(updatedScores))
        else withState(s.copy(scores = updatedScores))
      case _                                           => withError("Cannot score pone hand")

  /** Score the dealer's hand. Discard their cards once scored.
    * @returns
    *   A validated game with the dealer's hand scored, or an error if the game isn't in a state for
    *   scoring.
    */
  def scoreDealerHand: Either[String, Game] =
    game.state match
      case s @ Scoring(scores, hands, dealer, _, _, cut) =>
        val updatedScore  = scores(dealer).add(Scorer.forCards(hands(dealer), cut).total)
        val updatedScores = scores.updated(dealer, updatedScore)
        if updatedScore.points >= winningPoints
        then withState(Finished(updatedScores))
        else withState(s.copy(scores = updatedScores))
      case _                                             => withError("Cannot score dealer hand")

  /** Score the crib. Discard the crib cards once scored.
    * @returns
    *   A validated game with the crib scored, or an error if the game isn't in a state for scoring.
    */
  def scoreCrib: Either[String, Game] =
    game.state match
      case s @ Scoring(scores, _, dealer, _, crib, cut) =>
        val updatedScore  = scores(dealer).add(Scorer.forCards(crib, cut).total)
        val updatedScores = scores.updated(dealer, updatedScore)
        if updatedScore.points >= winningPoints
        then withState(Finished(updatedScores))
        else withState(s.copy(scores = updatedScores))
      case _                                            => withError("Cannot score crib")

  /** Swap the dealer if all scores completed.
    * @return
    *   The game with the dealer swapped to the other player.
    */
  def swapDealer: Either[String, Game] =
    game.state match
      case Scoring(scores, _, dealer, pone, _, _) => withState(newDiscarding(scores, pone, dealer))
      case _                                      => withError("Cannot swap dealer")

  /** Return the winner.
    * @returns
    *   The winner if the game is finished, otherwise None.
    */
  def winner: Option[Player] =
    game.state match
      case Finished(scores) => scores.toList.sortBy(_._2.points).lastOption.map(_._1)
      case _                => None

  /** Return the loser.
    * @returns
    *   The loser if the game is finished, otherwise None.
    */
  def loser: Option[Player] = winner.map(opponent)
