package com.nigeleke.cribbage.suit;

public enum Face {
    Ace(1, 1),
    Two(2, 2),
    Three(3, 3),
    Four(4, 4),
    Five(5, 5),
    Six(6, 6),
    Seven(7, 7),
    Eight(8, 8),
    Nine(9, 9),
    Ten(10, 10),
    Jack(10, 11),
    Queen(10, 12),
    King(10, 13);

    public final int value;
    public final int rank;

    private Face(int value, int rank) {
        this.value = value;
        this.rank = rank;
    }
}
