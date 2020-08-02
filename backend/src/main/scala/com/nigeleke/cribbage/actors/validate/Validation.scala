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