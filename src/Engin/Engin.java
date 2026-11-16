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
        this.board = Board.classic(false);
        System.out.println(board);
        // System.out.println("king move: " + Arrays.toString(board.getMove(wking).toArray()));
        
        board.move(new Pos("e2"), new Pos("e4"));
        System.out.println(board);
        board.move(new Pos("f1"), new Pos("b5"));
        board.move(new Pos("e1"), new Pos("e3"));
        board.move(new Pos("e3"), new Pos("d3"));
        board.move(new Pos("d3"), new Pos("d7"));
        board.move(new Pos("g1"), new Pos("h3"));
        board.move(new Pos("b1"), new Pos("a3"));
        board.move(new Pos("d2"), new Pos("d4"));
        board.move(new Pos("c1"), new Pos("h6"));
        board.move(new Pos("d1"), new Pos("d3"));
        System.out.println(board);
        System.out.println("white king move: " + Arrays.toString(board.getMove(new Pos("d8")).toArray()));
        System.out.println("white bishop move: " + Arrays.toString(board.getMove(new Pos("c8")).toArray()));
        System.out.println("white king check ? " + board.kingIsInCheck(true));

        // board.move(new Pos("c8"), new Pos("d7"));
        // System.out.println(board.controlByBlack.containsKey(new Pos("d8")));
        System.out.println(board);
//        board.updateControle();
        System.out.println("white king check ? " + board.kingIsInCheck(true));
        System.out.println("echek et mate? " + board.mate(true));
        System.out.println(Arrays.toString(board.getMove(new Pos("e1")).toArray()));
        board.move(new Pos("e1"), new Pos("c1"));
        System.out.println(board);
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
