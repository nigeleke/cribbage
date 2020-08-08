package com.nigeleke.cribbage.html

import org.scalajs.dom.html.Element
import scalatags.JsDom

trait Html {
  def html: JsDom.TypedTag[Element]
}

object Html {

  implicit def toHtmlString(html: Html): String = html.toString

}
