/*
 * Copyright (C) 2020  Nigel Eke
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

package com.nigeleke.cribbage.actors.validate

trait Validation {
  def validate: Option[String]
}

object Validation {

  case class Many(validations: Seq[Validation]) extends Validation {
    def validate: Option[String] = {
      val allValidated = validations.flatMap(_.validate).mkString("\n")
      if (allValidated.isEmpty) None else Some(allValidated)
    }
  }

  def validate(validation: Validation): Option[String] = validation.validate

  implicit class ValidationOps(validation: Validation) {
    def and(other: Validation) = Many(Seq(validation, other))
  }

}