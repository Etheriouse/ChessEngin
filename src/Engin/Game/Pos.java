package Engin.Game;

import Engin.Engin;

public class Pos implements Comparable<Pos> {

    public String letter;
    public Integer number;

    public int x, y;

    public Pos(int x, int y) {
        this.x = x;
        this.y = y;
        this.letter = ((char) (x + 97)) + "";
        this.number = Engin.height - y;
    }

    public Pos(String s) {
        this.letter = s.substring(0, 1);
        this.number = Integer.parseInt(s.substring(1));

        this.x = letter.charAt(0) - 97;
        this.y = Engin.height - number;
    }

    public boolean isUndefined() {
        return x == -1 || y == -1; 
    }

    @Override
    public String toString() {
        if(x == -1 || y == -1 ) return "undefined";
        return letter + number;
    }

    @Override
    public int hashCode() {
        return 31 * x + y;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj instanceof Pos) {
            Pos other = (Pos) (obj);
            return other.letter.equals(this.letter) && other.number.equals(this.number);
        } else
            return false;
    }

    @Override
    public int compareTo(Pos other) {
        int result = Integer.compare(this.y, other.y);

        if (result == 0) {
            return Integer.compare(this.x, other.x);
        }

        return result;
    }

}
