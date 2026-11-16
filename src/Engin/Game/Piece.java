package Engin.Game;

public enum Piece {
    
    None(null),

    wRook(Type.Rook),
    wKnight(Type.Knight),
    wBishop(Type.Bishop),
    wKing(Type.King),
    wQueen(Type.Queen),
    wPawn(Type.Pawn),

    bRook(Type.Rook),
    bKnight(Type.Knight),
    bBishop(Type.Bishop),
    bKing(Type.King),
    bQueen(Type.Queen),
    bPawn(Type.Pawn);

    private final Type t;

    Piece(Type t) {
        this.t = t;
    }

    enum Type {
        Rook,
        Knight,
        Bishop,
        King,
        Queen,
        Pawn
    }


    public boolean sameColor(Piece p) {
        return (this.ordinal() < 7 && p.ordinal() < 7) || (this.ordinal() >= 7 && p.ordinal() >= 7) && (this.ordinal() != 0 || p.ordinal() != 0);
    }

    public boolean isType(Type t) {
        return this.t != null && t.ordinal() == this.t.ordinal();
    }

    public boolean getColor() {
        return this.ordinal() < 7 && this.ordinal() > 0;
    }

    @Override
    public String toString() {
        switch (this.ordinal()) {
            case 1:
                return "a";
            case 2:
                return "z";
            case 3:
                return "e";
            case 4:
                return "r";
            case 5:
                return "t";
            case 6:
                return "y";

            case 7:
                return "q";
            case 8:
                return "s";
            case 9:
                return "d";
            case 10:
                return "f";
            case 11:
                return "g";
            case 12:
                return "h";
            case 0:
            default:
                return " ";
        }
    }
}
