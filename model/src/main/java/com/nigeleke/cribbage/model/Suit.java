package com.nigeleke.cribbage.model;

public enum Suit {
    Clubs("\u2663"),
    Diamonds("\u2666"),
    Hearts("\u2665"),
    Spades("\u2660");

    public final String smallSuit;

    private Suit(String smallSuit) {
        this.smallSuit = smallSuit;
    }

}
