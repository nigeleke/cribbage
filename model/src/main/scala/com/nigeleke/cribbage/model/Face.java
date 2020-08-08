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

package com.nigeleke.cribbage.model;

public enum Face {
    Ace(1, 1, "A"),
    Two(2, 2, "2"),
    Three(3, 3, "3"),
    Four(4, 4, "4"),
    Five(5, 5, "5"),
    Six(6, 6, "6"),
    Seven(7, 7, "7"),
    Eight(8, 8, "8"),
    Nine(9, 9, "9"),
    Ten(10, 10, "T"),
    Jack(10, 11, "J"),
    Queen(10, 12, "Q"),
    King(10, 13, "K");

    public final int value;
    public final int rank;
    public final String smallFace;

    private Face(int value, int rank, String smallFace) {
        this.value = value;
        this.rank = rank;
        this.smallFace = smallFace;
    }
}
