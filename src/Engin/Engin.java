package Engin;

import javax.swing.*;
import java.awt.*;
import java.util.Arrays;

import Engin.Game.*;

public class Engin {

    public static Integer width = 8, height = 8;

    public Board board;

    public static void start() {
        new Engin().run();
    }

    public void run() {
        this.board = Board.empty(false);
        Pos wking = new Pos("a3");
        Pos wbishop = new Pos("b4");
        Pos wrook = new Pos("c5");
        Pos bBishop = new Pos("d6");
        board.placePiece(wking, Piece.wKing);
        board.placePiece(wbishop, Piece.wBishop);
        board.placePiece(wrook, Piece.wRook);
        board.placePiece(bBishop, Piece.bBishop);
        board.updateControle();
        System.out.println(board);
        // System.out.println("king move: " + Arrays.toString(board.getMove(wking).toArray()));
        System.out.println("rook move: " + Arrays.toString(board.getMove(wrook).toArray()));
        
        // board.move(setupRookWhite, new Pos("h3"));
        // System.out.println(board);
        // System.out.println(Arrays.toString(board.getMove(king).toArray()));

        // JFrame frame = new JFrame("Chess");

        // frame.setSize(width * cellSize, (height * cellSize) + 40);
        // frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        // JPanel board = new JPanel() {
        // @Override
        // protected void paintComponent(Graphics g) {
        // super.paintComponent(g);

        // for (int row = 0; row < height; row++) {
        // for (int col = 0; col < width; col++) {

        // if ((row + col) % 2 == 0) {
        // g.setColor(Color.WHITE);
        // } else {
        // g.setColor(Color.BLACK);
        // }

        // g.fillRect(
        // col * cellSize,
        // row * cellSize,
        // cellSize,
        // cellSize);
        // }
        // }
        // }
        // };

        // frame.add(board);
        // frame.setVisible(true);
    }
}
