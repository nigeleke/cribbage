package cribbage
package styling

import model.*
import model.Card.Face.*
import model.Card.Suit.*
import model.Cards.*

object Styling:

  private def styledFace: Card.Face => String =
    case Ace   => "A"
    case Two   => "2"
    case Three => "3"
    case Four  => "4"
    case Five  => "5"
    case Six   => "6"
    case Seven => "7"
    case Eight => "8"
    case Nine  => "9"
    case Ten   => "T"
    case Jack  => "J"
    case Queen => "Q"
    case King  => "K"

  private def styledSuit: Card.Suit => String =
    case Hearts   => "\u2665"
    case Clubs    => "\u2663"
    case Diamonds => "\u2666"
    case Spades   => "\u2660"

  private def colourOf: Card.Suit => String =
    case Hearts | Diamonds => s"${Ansi.escape}[31m"
    case Clubs | Spades    => s"${Ansi.escape}[30m"

  extension (card: Card)
    def styled: String =
      val colour = colourOf(card.suit)
      val face   = styledFace(card.face)
      val suit   = styledSuit(card.suit)
      s"$colour$face$suit${Ansi.reset}"

  /** ANSI escape codes for pretty printing to console. */
  object Ansi:
    val escape = "\u001b"
    val reset  = s"$escape[0m"

  extension (cards: Crib | Deck | Hand)
    def styled: String = cards.toSeq.map(_.styled).mkString("[", " ", "]")

  extension (player: Player) def styled: String = player.id.toString.drop(30)

  extension (score: Score) def styled: String = s"${score.back}\u2192 ${score.front}"

  extension (play: Plays.Play)
    def styled: String =
      play match
        case Plays.Laid(player, card) => s"${player.styled} => ${card.styled}"
        case Plays.Pass(player)       => s"${player.styled} => pass"

  extension (plays: Plays)
    def styled: String =
      val inPlayStyled = plays.inPlay.map(_.styled).mkString("[", ", ", "]")
      val playedStyled = plays.played.map(_.styled).mkString("[", ", ", "]")
      s"nextToPlay: ${plays.nextPlayer.styled} inPlay: $inPlayStyled played: $playedStyled"

  extension (discarding: Discarding)
    def styled: String =
      val scoresStyled =
        discarding.scores.map((k, v) => s"${k.styled} => ${v.styled}").mkString(" / ")
      val handsStyled  =
        discarding.hands.map((k, v) => s"${k.styled} => ${v.styled}").mkString(" / ")
      s"""Discarding
         |  deck:   ${discarding.deck.styled}
         |  scores: $scoresStyled
         |  hands:  $handsStyled
         |  dealer: ${discarding.dealer.styled}
         |  pone:   ${discarding.pone.styled}
         |  crib:   ${discarding.crib.styled}""".stripMargin

  extension (playing: Playing)
    def styled: String =
      val scoresStyled = playing.scores.map((k, v) => s"${k.styled} => ${v.styled}").mkString(" / ")
      val handsStyled  = playing.hands.map((k, v) => s"${k.styled} => ${v.styled}").mkString(" / ")
      s"""Playing
         |  scores: $scoresStyled
         |  hands:  $handsStyled
         |  dealer: ${playing.dealer.styled}
         |  pone:   ${playing.pone.styled}
         |  crib:   ${playing.crib.styled}
         |  cut:    ${playing.cut.styled}
         |  plays:  ${playing.plays.styled}""".stripMargin

  extension (finished: Finished)
    def styled: String =
      val scoresStyled =
        finished.scores.map((k, v) => s"${k.styled} => ${v.styled}").mkString(" / ")
      s"""Finished
         |  scores: $scoresStyled""".stripMargin
