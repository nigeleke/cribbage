package com.nigeleke.cribbage

import cats.data.NonEmptyList

import org.scalatest.*
import org.scalatest.matchers.should.Matchers.*

def failWith(errors: NonEmptyList[String]) = fail(errors.toList.mkString(", "))

def passIfMatched(errors: NonEmptyList[String], list: List[String]) = errors.toList.mkString(", ") should be(list.mkString(", "))
