package cribbage.api
package repository

import model.*

trait Repository:
  def invitations: List[Invitation]
  def add(invite: Invitation): Either[String, Invitation]
