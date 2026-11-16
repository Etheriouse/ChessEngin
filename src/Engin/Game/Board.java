package Engin.Game;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;
import java.util.function.Function;

import Engin.Engin;

public class Board {

    public Piece cells[][];

    public Map<Pos, Integer> controlByWhite, controlByBlack;
    public Set<Piece> eatedWhite, eatedBlack;
    public boolean youPlayWhite;
    public boolean wleftRookMoved, wrightRookMoved, wKingMoved;
    public boolean bleftRookMoved, brightRookMoved, bKingMoved;
    public Pos wking = new Pos(-1, -1), bking = new Pos(-1, -1);

    public static Board empty(boolean play) {
        return new Board(play, 1);
    }

    public static Board classic(boolean play) {
        return new Board(play, 0);
    }

    public Board(boolean play, int mode) {
        // init the board
        this.youPlayWhite = play;
        this.cells = new Piece[Engin.height][Engin.width];
        this.controlByBlack = new HashMap<>();
        this.controlByWhite = new HashMap<>();
        this.eatedBlack = new TreeSet<>();
        this.eatedWhite = new TreeSet<>();
        if (mode == 0) {

            for (int i = 0; i < cells.length; i++) {
                for (int j = 0; j < cells[0].length; j++) {
                    if (i == 1) {
                        if (play)
                            cells[i][j] = Piece.bPawn;
                        else
                            // cells[i][j] = Piece.None;
                            cells[i][j] = Piece.wPawn;
                    } else if (i == Engin.height - 2) {
                        if (!play)
                            cells[i][j] = Piece.bPawn;
                        else
                            cells[i][j] = Piece.wPawn;

                    } else
                        cells[i][j] = Piece.None;
                }
            }

            if (play) {
                cells[0][0] = Piece.bRook;
                cells[0][Engin.width - 1] = Piece.bRook;
                cells[Engin.height - 1][0] = Piece.wRook;
                cells[Engin.height - 1][Engin.width - 1] = Piece.wRook;

                //
                cells[0][(int) ((Engin.width / 2) * 0.25)] = Piece.bKnight;
                cells[0][Engin.width - 1 - (int) ((Engin.width / 2) * 0.25)] = Piece.bKnight;
                cells[Engin.height - 1][(int) ((Engin.width / 2) * 0.25)] = Piece.wKnight;
                cells[Engin.height - 1][Engin.width - 1 - (int) ((Engin.width / 2) * 0.25)] = Piece.wKnight;

                cells[0][(int) ((Engin.width / 2) * 0.5)] = Piece.bBishop;
                cells[0][Engin.width - 1 - (int) ((Engin.width / 2) * 0.5)] = Piece.bBishop;
                cells[Engin.height - 1][(int) ((Engin.width / 2) * 0.5)] = Piece.wBishop;
                cells[Engin.height - 1][Engin.width - 1 - (int) ((Engin.width / 2) * 0.5)] = Piece.wBishop;

                cells[0][(Engin.height / 2)] = Piece.bKing;
                bking = new Pos((Engin.height / 2), 0);
                cells[0][(Engin.height / 2) - 1] = Piece.bQueen;
                cells[Engin.height - 1][(Engin.height / 2)] = Piece.wKing;
                wking = new Pos((Engin.height / 2), Engin.height - 1);
                cells[Engin.height - 1][(Engin.height / 2) - 1] = Piece.wQueen;
            } else {
                cells[0][0] = Piece.wRook;
                cells[0][Engin.width - 1] = Piece.wRook;
                cells[Engin.height - 1][0] = Piece.bRook;
                cells[Engin.height - 1][Engin.width - 1] = Piece.bRook;

                //
                cells[0][(int) ((Engin.width / 2) * 0.25)] = Piece.wKnight;
                cells[0][Engin.width - 1 - (int) ((Engin.width / 2) * 0.25)] = Piece.wKnight;
                cells[Engin.height - 1][(int) ((Engin.width / 2) * 0.25)] = Piece.bKnight;
                cells[Engin.height - 1][Engin.width - 1 - (int) ((Engin.width / 2) * 0.25)] = Piece.bKnight;

                cells[0][(int) ((Engin.width / 2) * 0.5)] = Piece.wBishop;
                cells[0][Engin.width - 1 - (int) ((Engin.width / 2) * 0.5)] = Piece.wBishop;
                cells[Engin.height - 1][(int) ((Engin.width / 2) * 0.5)] = Piece.bBishop;
                cells[Engin.height - 1][Engin.width - 1 - (int) ((Engin.width / 2) * 0.5)] = Piece.bBishop;

                cells[0][(Engin.height / 2)] = Piece.wKing;
                wking = new Pos((Engin.height / 2), 0);
                cells[0][(Engin.height / 2) - 1] = Piece.wQueen;
                cells[Engin.height - 1][(Engin.height / 2)] = Piece.bKing;
                bking = new Pos((Engin.height / 2), Engin.height - 1);
                cells[Engin.height - 1][(Engin.height / 2) - 1] = Piece.bQueen;

            }
        } else {
            for (int i = 0; i < cells.length; i++) {
                for (int j = 0; j < cells[0].length; j++) {
                    cells[i][j] = Piece.None;
                }
            }
        }
        updateControle();
    }

    public void updateControle() {
        controlByBlack.clear();
        controlByWhite.clear();
        for (int i = 0; i < Engin.height; i++) {
            for (int j = 0; j < Engin.width; j++) {
                boolean color = cells[i][j].getColor();
                addControle(color, new Pos(j, i));
            }
        }
        System.out.println("b: " + controlByBlack);
        System.out.println("w: " + controlByWhite);
        // parcour toute les casse de l'echequier pour
    }

    public void placePiece(Pos pos, Piece p) {
        if (p.isType(Piece.Type.King)) {
            if (p.getColor())
                wking = pos;
            else
                bking = pos;
        }
        cells[pos.y][pos.x] = p;
    }

    public List<Pos> getMove(Pos pos) {
        ArrayList<Pos> l = new ArrayList<>();

        Piece p = this.cells[pos.y][pos.x];
        switch (p) {
            case Piece.wRook:
            case Piece.bRook: {
                int dir[][] = {
                        { 1, 0 },
                        { -1, 0 },
                        { 0, 1 },
                        { 0, -1 },
                };
                movingLine(l, p.getColor(), dir, pos);
                break;
            }

            case Piece.wBishop:
            case Piece.bBishop: {
                int dir[][] = {
                        { 1, 1 },
                        { 1, -1 },
                        { -1, 1 },
                        { -1, -1 },
                };
                movingLine(l, p.getColor(), dir, pos);
                break;
            }

            case Piece.wQueen:
            case Piece.bQueen: {
                int dir[][] = {
                        { 1, 0 },
                        { -1, 0 },
                        { 0, 1 },
                        { 0, -1 },
                        { 1, 1 },
                        { 1, -1 },
                        { -1, 1 },
                        { -1, -1 },
                };
                movingLine(l, p.getColor(), dir, pos);
                break;
            }

            case Piece.wKnight:
            case Piece.bKnight: {
                int dir[][] = {
                        { 2, 1 },
                        { 2, -1 },
                        { 1, -2 },
                        { -1, -2 },
                        { 1, 2 },
                        { -1, 2 },
                        { -2, 1 },
                        { -2, -1 },
                };
                movingPatern(l, p.getColor(), dir, pos);
                break;
            }

            case Piece.bPawn:
            case Piece.wPawn: {
                pawnMovement(l, pos, p);
                break;
            }

            case Piece.wKing: {
                if (!kingIsInCheck(true)) {
                    if (!wKingMoved) {
                        if (!wleftRookMoved) {
                            boolean clean = true;
                            for (int i = wking.x; i > 0; i--) {
                                clean &= cells[wking.y][i] == Piece.None
                                        && !controlByBlack.containsKey(new Pos(i, wking.y));
                            }
                            if (clean)
                                l.add(new Pos(2, wking.y));
                        }
                        if (!wrightRookMoved) {
                            boolean clean = true;
                            for (int i = wking.x; i < Engin.width - 2; i++) {
                                clean &= cells[wking.y][i] == Piece.None
                                        && !controlByBlack.containsKey(new Pos(i, wking.y));
                            }
                            if (clean)
                                l.add(new Pos(Engin.width - 2, wking.y));
                        }
                    }
                }
            }
            case Piece.bKing: {
                if (!kingIsInCheck(false)) {
                    if (!bKingMoved) {
                        if (!bleftRookMoved) {
                            boolean clean = true;
                            for (int i = bking.x - 1; i > 0; i--) {
                                clean &= cells[bking.y][i] == Piece.None
                                        && !controlByWhite.containsKey(new Pos(i, bking.y));
                            }
                            if (clean)
                                l.add(new Pos(2, bking.y));
                        }
                        if (!brightRookMoved) {
                            boolean clean = true;
                            for (int i = bking.x + 1; i < Engin.width - 2; i++) {
                                clean &= cells[bking.y][i] == Piece.None
                                        && !controlByWhite.containsKey(new Pos(i, bking.y));
                            }
                            if (clean)
                                l.add(new Pos(Engin.width - 2, bking.y));
                        }
                    }
                }
                for (int i = pos.y - 1; i < pos.y + 2; i++) {
                    for (int j = pos.x - 1; j < pos.x + 2; j++) {
                        if (j >= 0 && j < Engin.width && i >= 0 && i < Engin.height) {
                            Pos tmp = new Pos(j, i);
                            // System.out.println(tmp);
                            if (cells[i][j] != Piece.None && cells[i][j].sameColor(p))
                                continue;
                            if (p.getColor()) {
                                if (controlByBlack.containsKey(tmp))
                                    continue;
                            } else {
                                if (controlByWhite.containsKey(tmp))
                                    continue;
                            }
                            l.add(tmp);
                        }
                    }
                }
                break;
            }

            default:
                break;
        }

        // System.out.println("l: " + Arrays.toString(l.toArray()));
        protectionKingFilter(pos, l);

        return l;
    }

    public void protectionKingFilter(Pos p, List<Pos> l) {
        // // modifier la fonction pour que si le roi ce bouge lui meme sa prend pas
        // // posking mais acutal

        Pos oldKing = p;
        Piece old;
        Piece head = cells[p.y][p.x];

        List<Pos> toErase = new ArrayList<>();
        for (Pos to : l) {
            old = cells[to.y][to.x];
            cells[to.y][to.x] = head;
            cells[p.y][p.x] = Piece.None;
            if (head.isType(Piece.Type.King)) {
                if (head.getColor()) {
                    oldKing = wking;
                    wking = to;
                } else {
                    oldKing = bking;
                    bking = to;
                }
            }
            int dirBishop[][] = { { 1, 1 }, { -1, 1 }, { -1, -1 }, { 1, -1 } };
            int dirRook[][] = { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } };
            if (isKingAttack(head.getColor(), dirRook, true)
                    || isKingAttack(head.getColor(), dirBishop, false))
                toErase.add(to);

            if (head.isType(Piece.Type.King)) {
                if (head.getColor())
                    wking = oldKing;
                else
                    bking = oldKing;
            }
            cells[to.y][to.x] = old;
            cells[p.y][p.x] = head;
        }
        for (Pos tmp : toErase) {
            l.remove(tmp);
        }

        // Piece oldTo;
        // Piece actual = cells[p.y][p.x];
        // List<Pos> toRemove = new ArrayList<>();
        // for (Pos possibleTo : l) {
        // // placer temporairemt a possitble to
        // oldTo = cells[possibleTo.y][possibleTo.x];
        // cells[possibleTo.y][possibleTo.x] = actual;
        // cells[p.y][p.x] = Piece.None;
        // // partir du roi en queen et knight et checker si y'a une white

        // int dirRook[][] = { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } };
        // int dirBishop[][] = { { 1, 1 }, { -1, 1 }, { -1, -1 }, { 1, -1 } };
        // // int patKnight[][] = { { 2, 1 }, { 2, -1 }, { -2, 1 }, { -2, -1 }, { 1, 2
        // }, {
        // // -1, 2 }, { 1, -2 },
        // // { -1, -2 } };

        // if (isKingAttack(actual.getColor(), dirRook, true, p)
        // || isKingAttack(actual.getColor(), dirBishop, false, p))
        // toRemove.add(possibleTo);

        // // System.out.println("toremove: " + Arrays.toString(toRemove.toArray()));

        // // if (targetLine(actual.getColor(), dirRook, actual.getColor() ? wking :
        // bking,
        // // true)
        // // || targetLine(actual.getColor(), dirBishop, actual.getColor() ? wking :
        // // bking, false)) {
        // // toRemove.add(possibleTo);
        // // }

        // // remplacement
        // cells[p.y][p.x] = actual;
        // cells[possibleTo.y][possibleTo.x] = oldTo;
        // }
        // for (Pos tmp : toRemove) {
        // l.remove(tmp);
        // }
    }

    public boolean kingIsInCheck(boolean color) {
        return color ? controlByBlack.containsKey(wking) : controlByWhite.containsKey(bking);
    }

    public boolean move(Pos from, Pos to) {
        List<Pos> legal = getMove(from);
        if (legal.contains(to)) {
            removeControle(cells[from.y][from.x].getColor(), from);
            eat(from, to);
            addControle(cells[to.y][to.x].getColor(), to);

            return true;
        } else
            return false;
    }

    private boolean eat(Pos from, Pos to) {
        Piece pfrom = cells[from.y][from.x];

        if (pfrom.isType(Piece.Type.Rook)) {
            if (pfrom.getColor()) {
                if (from.x == 0)
                    wleftRookMoved = true;
                if (from.y == Engin.width)
                    wrightRookMoved = true;
            } else {
                if (from.x == 0)
                    bleftRookMoved = true;
                if (from.y == Engin.width)
                    brightRookMoved = true;
            }
        } else if (pfrom.isType(Piece.Type.King)) { // if first move go to +2 => rock
            if (pfrom.getColor()) {
                wKingMoved = true;
            } else {
                bKingMoved = true;
            }
            if (to.x == 2) {
                cells[to.y][3] = cells[to.y][0];
                cells[to.y][0] = Piece.None;
            }
            if (to.x == Engin.width - 2) {
                cells[to.y][Engin.width - 3] = cells[to.y][Engin.width - 1];
                cells[to.y][Engin.width - 1] = Piece.None;
            }

        }

        Piece pto = cells[to.y][to.x];
        if (pto != Piece.None) {
            removeControle(pto.getColor(), to);
            if (pfrom.getColor())
                eatedBlack.add(pto);
            else
                eatedWhite.add(pto);
        }
        cells[to.y][to.x] = pfrom;
        cells[from.y][from.x] = Piece.None;
        return true;
    }

    public void addControle(boolean colorPlayed, Pos to) {
        Map<Pos, Integer> map = colorPlayed ? controlByWhite : controlByBlack;
        for (Pos p : getMove(to))
            map.merge(p, 1, Integer::sum);
    }

    public void removeControle(boolean colorPlayed, Pos from) {
        Map<Pos, Integer> map = colorPlayed ? controlByWhite : controlByBlack;
        for (Pos p : getMove(from)) {
            map.computeIfPresent(p, (_, count) -> {
                return count <= 1 ? null : count - 1; // si count est non null alors -1 sinon null
            });
        }
    }

    private void pawnMovement(List<Pos> l, Pos pos, Piece p) {
        // if you play white, white go to -y
        // if you play black, black go to -y
        boolean whitePiece = p == Piece.wPawn;
        int movePawnWhite = youPlayWhite ? -1 : 1;

        if (whitePiece) {
            if (pos.y == 1)
                l.add(new Pos(pos.x, pos.y + (2 * movePawnWhite)));
            l.add(new Pos(pos.x, pos.y + movePawnWhite));
        } else {
            if (pos.y == Engin.height - 2)
                l.add(new Pos(pos.x, pos.y + (2 * -movePawnWhite)));
            l.add(new Pos(pos.x, pos.y + -movePawnWhite));
        }

        if (pos.y > 0 && pos.y < Engin.height - 1) {
            Piece tmp = Piece.None;
            Pos ptm = null;
            if (pos.x < Engin.width - 1) {
                ptm = new Pos(pos.x + 1, pos.y + movePawnWhite);
                // System.out.println(ptm);
                tmp = cells[ptm.y][ptm.x];
            }
            if (pos.x > 0) {
                ptm = new Pos(pos.x - 1, pos.y + movePawnWhite);
                tmp = cells[ptm.y][ptm.x];
            }
            if (tmp != Piece.None && !tmp.sameColor(p))
                l.add(ptm);

        }
    }

    public boolean simulate(Pos from, Pos to, Function<Void, Boolean> doSomething) {

        Pos oldKing = from;
        Piece old = cells[to.y][to.x];
        Piece head = cells[from.y][from.x];

        boolean result;

        removeControle(cells[from.y][from.x].getColor(), from);
        // eat(from, to); equivalent de eat sans toucher au set de piece eated
        if (!old.isNone())
            removeControle(old.getColor(), to);
        cells[to.y][to.x] = head;
        cells[from.y][from.x] = Piece.None;
        if (head.isType(Piece.Type.King)) {
            if (head.getColor()) {
                oldKing = wking;
                wking = to;
            } else {
                oldKing = bking;
                bking = to;
            }
        }
        addControle(head.getColor(), to);

        result = doSomething.apply(null);

        removeControle(head.getColor(), to);
        if (head.isType(Piece.Type.King)) {
            if (head.getColor())
                wking = oldKing;
            else
                bking = oldKing;
        }
        cells[to.y][to.x] = old;
        cells[from.y][from.x] = head;
        if (!old.isNone())
            addControle(old.getColor(), to);
        addControle(head.getColor(), from);

        return result;
    }

    // cherche si le roi de couleur color est atker
    // parcour toute les direction possible en vers lui du coup

    public boolean mate(boolean kingColor) {
        // try all move from kingcolor piece, if in all case chek, so mate

        for (int i = 0; i < Engin.height; i++) {
            for (int j = 0; j < Engin.width; j++) {
                Piece look = cells[i][j];
                if (!look.isNone() && look.sameColor(kingColor)) { // if look not "none"
                    Pos from = new Pos(j, i);
                    for (Pos to : getMove(from)) {
                        if (!simulate(from, to, (Void) -> {
                            return kingIsInCheck(kingColor);
                        })) {
                            // System.out.println("solution found: " + from + "->" + to);
                            return false;
                        }
                    }
                }
            }
        }

        return true;
    }

    private boolean isKingAttack(boolean kingColor, int dir[][], boolean rook) {

        Pos posKing;
        if (kingColor)
            posKing = wking;
        else
            posKing = bking;

        if (posKing.isUndefined())
            return false;

        for (int d[] : dir) {
            int x = posKing.x + d[0], y = posKing.y + d[1];
            while (x >= 0 && x < Engin.width && y >= 0 && y < Engin.height) {
                if (cells[y][x] != Piece.None) {
                    Piece look = cells[y][x];
                    if (rook) {
                        if (look.getColor() ^ kingColor)
                            return look.isType(Piece.Type.Rook) || look.isType(Piece.Type.Queen);
                        else
                            break;
                    } else {
                        if (look.getColor() ^ kingColor)
                            return look.isType(Piece.Type.Bishop) || look.isType(Piece.Type.Queen);
                        else
                            break;
                    }
                }

                x += d[0];
                y += d[1];
            }
        }
        return false;
    }

    private boolean targetLine(boolean colorMoving, int dir[][], Pos p, boolean rook) {
        return false;
    }

    private void movingLine(List<Pos> l, boolean colorMoving, int dir[][], Pos p) {
        for (int i = 0; i < dir.length; i++) {
            int x = p.x, y = p.y;
            boolean stop = false;
            while (x >= 0 && x < Engin.width && y >= 0 && y < Engin.height && !stop) {
                if (!(p.x == x && p.y == y)) {
                    Piece look = cells[y][x];
                    if (look != Piece.None) {
                        if (look.sameColor(cells[p.y][p.x]))
                            break;
                        else
                            stop = true;
                    }
                    l.add(new Pos(x, y));
                }
                x += dir[i][0];
                y += dir[i][1];
            }
        }
    }

    private void movingPatern(List<Pos> l, boolean colorMoving, int patern[][], Pos p) {
        for (int i = 0; i < patern.length; i++) {
            int x = p.x + patern[i][0], y = p.y + patern[i][1];
            if (!(x >= 0 && x < Engin.width && y >= 0 && y < Engin.height))
                continue;

            Piece look = cells[y][x];
            if (look != Piece.None) {
                if (look.sameColor(cells[p.y][p.x]))
                    continue;

            }
            l.add(new Pos(x, y));
        }
    }

    @Override
    public String toString() {
        String res = "   ";
        for (int i = 0; i < cells.length; i++)
            res += ((char) (i + 97)) + " ";
        res += "\n  ";
        for (int i = 0; i < cells.length; i++)
            res += "--";
        res += "-\n";
        for (int i = 0; i < cells.length; i++) {
            res += cells.length - i + " |";
            for (int j = 0; j < cells[i].length; j++) {
                res += cells[i][j] + "|";
            }
            res += "\n";
        }
        res += "  ";
        for (int i = 0; i < cells.length; i++)
            res += "--";
        res += "-\n";
        return res;
    }
}
