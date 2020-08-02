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
