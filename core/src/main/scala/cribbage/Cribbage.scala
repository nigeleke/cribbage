package cribbage

import model.*
import model.Cards.*
import model.Draw.*
import model.Rule.*

import cribbage.scorer.Scorer.*

final case class Cribbage(players: Set[Player], deck: Deck)

object Cribbage:
  def newGame: Cribbage =
    Cribbage(
      Set(Player.newPlayer, Player.newPlayer),
      Cards.fullDeck
    )

  def makeDraw: Cribbage => Draw =
    case Cribbage(players, _) => drawForPlayers(players)

  private def drawForPlayers(players: Set[Player]) =
    val shuffled          = shuffledDeck
    val (remaining, cut1) = shuffled.cut
    val (_, cut2)         = remaining.cut
    val cuts              = Seq(cut1, cut2).sortBy(_.rank)
    val playerByCut       = cuts.zip(players).toMap
    val cutsByPlayer      = for ((k, v) <- playerByCut) yield (v, k)
    val sameRank          = cut1.rank == cut2.rank
    if sameRank
    then Draw.Undecided(cutsByPlayer)
    else Draw.Decided(cutsByPlayer, playerByCut(cuts.head), playerByCut(cuts.last))

  def redraw: Undecided => Draw =
    case Undecided(draws) => drawForPlayers(draws.keySet)

  def dealFirstHand: Decided => Discarding =
    case Decided(_, dealer, pone) =>
      val initialScores = Set(dealer, pone).map((_, Score.zero)).toMap
      newDiscarding(initialScores, dealer, pone)

  private def newDiscarding(scores: Map[Player, Score], dealer: Player, pone: Player): Discarding =
    val (deck, hands) = shuffledDeck.deal(NumberOfPlayersInGame, CardsDealtPerHand)
    val dealtHands    = Set(pone, dealer).zip(hands).toMap
    val crib          = emptyCrib
    Discarding(deck, scores, dealtHands, dealer, pone, crib)

  def discardToCrib(player: Player, discards: Seq[Card]): Discarding => Discarding | Playing |
    Finished =
    case discarding @ Discarding(deck, scores, hands, dealer, pone, crib) =>
      require(Set(dealer, pone).contains(player), NonParticipatingPlayer(player))
      require(hands(player).containsAll(discards), UnheldCards(player, discards))
      require(
        hands(player).removeAll(discards).size >= CardsKeptPerHand,
        TooManyDiscards(player, discards)
      )
      val updatedHand  = hands(player).removeAll(discards)
      val updatedHands = hands.updated(player, updatedHand)
      val updatedCrib  = crib.addAll(discards)
      val (_, cut)     = deck.cut
      if updatedCrib.isFull
      then scoreHisHeels(Playing(scores, updatedHands, dealer, pone, updatedCrib, cut, Plays(pone)))
      else discarding.copy(hands = updatedHands, crib = updatedCrib)

  def play(player: Player, card: Card): Playing => Playing | Discarding | Finished =
    case playing @ Playing(_, hands, dealer, pone, _, _, plays) =>
      def opponent(player: Player): Player = if player == dealer then pone else dealer

      require(Set(dealer, pone).contains(player), NonParticipatingPlayer(player))
      require(plays.nextPlayer == player, NotYourTurn(player))
      require(hands(player).contains(card), UnheldCard(player, card))
      require(plays.runningTotal + card.value <= PlayTarget, PlayTargetExceeded(player, card))

      val updatedHand  = hands(player).remove(card)
      val updatedHands = hands.updated(player, updatedHand)
      val updatedPlays = plays.play(player, card)
      scorePlay(playing.copy(hands = updatedHands, plays = updatedPlays)) match
        case scoredPlay: Playing =>
          if updatedPlays.allPlaysCompleted
          then
            val regatheredHands = updatedPlays.regather
            val resetPlays      = Plays(pone)
            scoreHands(scoredPlay.copy(hands = regatheredHands, plays = resetPlays)) match
              case scoredHands: Playing => newDiscarding(scoredHands.scores, pone, dealer)
              case finished: Finished   => finished
          else scoredPlay.copy(plays = updatedPlays.withNextPlayer(opponent(player)))
        case finished: Finished  => finished

  def pass(player: Player): Playing => Playing | Finished =
    case playing @ Playing(_, hands, dealer, pone, _, _, plays) =>
      def opponent(player: Player): Player = if player == dealer then pone else dealer

      require(Set(dealer, pone).contains(player), NonParticipatingPlayer(player))
      require(plays.nextPlayer == player, NotYourTurn(player))
      require(plays.mustPass(hands(player)), PlayTargetAttainable(player))

      val updatedPlays = plays.pass(player)

      scorePass(playing.copy(plays = updatedPlays)) match
        case scoredPlaying: Playing =>
          if updatedPlays.allPassed
          then scoredPlaying.copy(plays = updatedPlays.restarted)
          else scoredPlaying.copy(plays = updatedPlays.withNextPlayer(opponent(player)))
        case finished: Finished     => finished
