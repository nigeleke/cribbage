package com.nigeleke.cribbage

//import com.nigeleke.cribbage.actors.UserService.AuthTimestamp
//import com.nigeleke.cribbage.actors.UserService.AuthToken
import org.scalatest.*
import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class UserServiceSpec extends AnyWordSpec with Matchers:

  "A UserService" should {

    "allow new UserAccounts to be created" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      val result = eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      result.reply should be(Some(user.id))
//      result.event should be(UserService.UserRegistered(user))
//      result.state.validUsers should contain(user.id -> user.secret)
//      result.state.loggedInUsers should be(empty)
    }

    "disallow new UserAccount if User already exists" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      val result = eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      result.reply should be(None)
//      result.events should be(empty)
//      result.state.validUsers should contain(user.id -> user.secret)
//      result.state.loggedInUsers should be(empty)
    }

    "return the Secret for an existing User" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      val result = eventSourcedTestKit.runCommand(UserService.GetSecret(user.id, _))
//      result.reply match
//        case Some(password) =>
//          password should be("hashedSecret")
//          result.events should be(empty)
//          result.state.validUsers should contain(user.id -> user.secret)
//          result.state.loggedInUsers should be(empty)
//        case None => fail("Expected Some(secret)")
    }

    "not return the Secret for a non-existing User" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      val result = eventSourcedTestKit.runCommand(UserService.GetSecret(user.id, _))
//      result.reply should be(None)
//      result.events should be(empty)
//      result.state.validUsers.keys should not contain (user.id)
//      result.state.loggedInUsers should be(empty)
    }

    "allow a non logged-in User to Login" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      val result = eventSourcedTestKit.runCommand(UserService.LoginUser(user.id, _))
//      result.reply match
//        case Some(authToken) =>
//          result.event should be(a[UserService.UserLoggedIn])
//          result.state.validUsers should contain(user.id -> user.secret)
//          val timestamp = result.state.loggedInUsers(user.id)
//          timestamp.token should be(authToken)
//        case None => fail("Expected Some(authToken)")
    }

    "allow a logged-in User to Login" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      eventSourcedTestKit.runCommand(UserService.LoginUser(user.id, _))
//      val result = eventSourcedTestKit.runCommand(UserService.LoginUser(user.id, _))
//      result.reply match
//        case Some(authToken) =>
//          result.event should be(a[UserService.UserLoggedIn])
//          result.state.validUsers should contain(user.id -> user.secret)
//          val timestamp = result.state.loggedInUsers(user.id)
//          timestamp.token should be(authToken)
//        case None => fail("Expected Some(authToken)")
//
    }

    "not allow a non-registered User to Login" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      val result = eventSourcedTestKit.runCommand(UserService.LoginUser(user.id, _))
//      result.reply should be(None)
//      result.events should be(empty)
//      result.state.validUsers should be(empty)
//      result.state.loggedInUsers should be(empty)
    }

    "allow a logged-in User to Logout" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      eventSourcedTestKit.runCommand(UserService.LoginUser(user.id, _))
//      val result = eventSourcedTestKit.runCommand(UserService.LogoutUser(user.id, _))
//      result.reply should be(Some(user.id))
//      result.event should be(a[UserService.UserLoggedOut])
//      result.state.validUsers should contain(user.id -> user.secret)
//      result.state.loggedInUsers should be(empty)
    }

    "ignore a non logged-in User's request to Logout" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      val result = eventSourcedTestKit.runCommand(UserService.LogoutUser(user.id, _))
//      result.reply should be(None)
//      result.events should be(empty)
//      result.state.validUsers should contain(user.id -> user.secret)
//      result.state.loggedInUsers should be(empty)
    }

    "return an AuthToken for a logged-in User" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      eventSourcedTestKit.runCommand(UserService.LoginUser(user.id, _))
//      val result = eventSourcedTestKit.runCommand(UserService.GetAuthToken(user.id, _))
//      result.reply match
//        case Some(authToken) =>
//          result.event should be(a[UserService.AuthTokenProvided])
//          result.state.validUsers should contain(user.id -> user.secret)
//          result.state.loggedInUsers(user.id).token should be(authToken)
//        case None => fail("Expected Some(authToken)")
    }

    "not return an AuthToken for a non logged-in User" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      val result = eventSourcedTestKit.runCommand(UserService.GetAuthToken(user.id, _))
//      result.reply should be(None)
//      result.events should be(empty)
//      result.state.validUsers should contain(user.id -> user.secret)
//      result.state.loggedInUsers should be(empty)
    }

    "not return an AuthToken for a logged-out User" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      eventSourcedTestKit.runCommand(UserService.LoginUser(user.id, _))
//      eventSourcedTestKit.runCommand(UserService.LogoutUser(user.id, _))
//      val result = eventSourcedTestKit.runCommand(UserService.GetAuthToken(user.id, _))
//      result.reply should be(None)
//      result.events should be(empty)
//      result.state.validUsers should contain(user.id -> user.secret)
//      result.state.loggedInUsers should be(empty)
    }

    "deregister an existing User" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      val result = eventSourcedTestKit.runCommand(UserService.DeregisterUser(user.id, _))
//      result.reply should be(Some(user.id))
//      result.event should be(UserService.UserDeregistered(user.id))
//      result.state.validUsers.keys should not contain (user.id)
//      result.state.loggedInUsers should be(empty)
    }

    "not deregister a logged-in User" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      eventSourcedTestKit.runCommand(UserService.RegisterUser(user, _))
//      eventSourcedTestKit.runCommand(UserService.LoginUser(user.id, _))
//      val result = eventSourcedTestKit.runCommand(UserService.DeregisterUser(user.id, _))
//      result.reply should be(None)
//      result.events should be(empty)
//      result.state.validUsers.keys should contain(user.id)
//      result.state.loggedInUsers.keys should contain(user.id)
    }

    "ignore a deregistration request for a non-existing User" in {
      fail()
//      val user = UserService.UserCredentials("user", "hashedSecret")
//      val result = eventSourcedTestKit.runCommand(UserService.DeregisterUser(user.id, _))
//      result.reply should be(None)
//      result.events should be(empty)
//      result.state.validUsers.keys should not contain (user.id)
//      result.state.loggedInUsers should be(empty)
    }

  }
