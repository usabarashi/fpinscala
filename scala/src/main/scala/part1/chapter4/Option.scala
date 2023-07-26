
package part1.chapter4

import scala.{Option as _, Some as _, None as _}

enum Option[+A]:
    case None
    case Some(get: A)

    def map[B](f: A => B): Option[B] =
        this match
            case None => None
            case Some(value) => Some(f(value))

    def flatMap[B](f: A => Option[B]): Option[B] =
        map(f).getOrElse(None)

    def getOrElse[B >: A](default: => B): B =
        this match
            case None => default
            case Some(value) => value

    def orElse[B >: A](ob: => Option[B]): Option[B] =
        map(Some(_)).getOrElse(ob)

    def filter(f: A => Boolean): Option[A] =
        flatMap(value => if f(value) then Some(value) else None)
