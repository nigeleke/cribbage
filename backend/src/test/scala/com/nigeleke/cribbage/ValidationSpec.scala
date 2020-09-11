package com.nigeleke.cribbage

import com.nigeleke.cribbage.entity.validate.Validation
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class ValidationSpec extends AnyWordSpec with Matchers {

  case object AlwaysValid extends Validation {
    override def validate: Option[String] = None
  }

  case object AlwaysInvalid extends Validation {
    override def validate: Option[String] = Some("Invalid")
  }

  "A Validation" should {

    import Validation._

    "return None if no error" in {
      validate(AlwaysValid) should be(None)
    }

    "return Some(error) if error" in {
      validate(AlwaysInvalid) should be(Some("Invalid"))
    }

    "return None if no errors over many Validations" in {
      validate(AlwaysValid and AlwaysValid) should be(None)
    }

    "return Some(errors) if many errors over many Validations" in {
      validate(AlwaysInvalid and AlwaysInvalid) should be(Some("Invalid\nInvalid"))
    }

    "return Some(errors) if mix of errors over many Validations" in {
      validate(AlwaysValid and AlwaysInvalid and AlwaysValid and AlwaysInvalid) should be(Some("Invalid\nInvalid"))
      validate(AlwaysInvalid and AlwaysValid and AlwaysInvalid and AlwaysValid) should be(Some("Invalid\nInvalid"))
    }

  }

}
