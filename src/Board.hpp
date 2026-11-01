#ifndef BOARD_CPP
#define BOARD_CPP

#include <cstdint>
#include <stdlib.h>
#include <vector>
#include "raylib.h"

class Board
{
public:
    Board()
    {
        cell_size = (width > height ? height : width) / chess_size;
        start_x_draw = (width - (cell_size * chess_size)) / 2;

        const unsigned int size = chess_size * chess_size;

        for (uint32_t i = 0; i < size; i++)
            board[i] = 0;

        board[0] = Piece::Rook;
        board[1] = Piece::Knight;
        board[2] = Piece::Bishop;
        board[3] = Piece::Queen;
        board[4] = Piece::King;
        board[5] = Piece::Bishop;
        board[6] = Piece::Knight;
        board[7] = Piece::Rook;

        board[8] = Piece::Pawn;
        board[9] = Piece::Pawn;
        board[10] = Piece::Pawn;
        board[11] = Piece::Pawn;
        board[12] = Piece::Pawn;
        board[13] = Piece::Pawn;
        board[14] = Piece::Pawn;
        board[15] = Piece::Pawn;

        board[size - 9] = Piece::Pawn | 0x80;
        board[size - 10] = Piece::Pawn | 0x80;
        board[size - 11] = Piece::Pawn | 0x80;
        board[size - 12] = Piece::Pawn | 0x80;
        board[size - 13] = Piece::Pawn | 0x80;
        board[size - 14] = Piece::Pawn | 0x80;
        board[size - 15] = Piece::Pawn | 0x80;
        board[size - 16] = Piece::Pawn | 0x80;

        board[size - 8] = Piece::Rook | 0x80;
        board[size - 7] = Piece::Knight | 0x80;
        board[size - 6] = Piece::Bishop | 0x80;
        board[size - 5] = Piece::Queen | 0x80;
        board[size - 4] = Piece::King | 0x80;
        board[size - 3] = Piece::Bishop | 0x80;
        board[size - 2] = Piece::Knight | 0x80;
        board[size - 1] = Piece::Rook | 0x80;

        board[2 + 4 * 8] = Piece::Pawn | 0x00;
        board[3 + 3 * 8] = Piece::Pawn | 0x00;
    }

    ~Board()
    {
        UnloadTexture(grid_texture[0]);
        UnloadTexture(grid_texture[1]);
        free(board);
    }

    void Draw();

    enum Piece : uint8_t
    {
        Pawn = 1,
        Bishop = 2,
        Knight = 3,
        Rook = 4,
        King = 5,
        Queen = 6
    };

    typedef struct
    {
        int x, y;
    } pair;

    std::vector<pair> GetLegalMove(Board::pair pos);
    void multidirection(std::vector<Board::pair> &l_pos, std::vector<Board::pair> &directions, Board::pair pos, bool pcolor);
    void SelectPiece(Vector2 pos);
    void MovePiece(Vector2 pos);

    bool selected = false;

private:
    int width = 1920, height = 1080, chess_size = 8, sprite_size = 32, cell_size, start_x_draw;

    pair select_cell;

    uint8_t *board = (uint8_t *)malloc(sizeof(uint8_t) * chess_size * chess_size);
    Texture2D grid_texture[2] = {LoadTexture("assets/white.png"), LoadTexture("assets/black.png")};

    Texture2D white_piece[7] = {{}, // pawn, bishop, knight, rook, king, queen
                                LoadTexture("assets/piece/white/pawn.png"),
                                LoadTexture("assets/piece/white/bishop.png"),
                                LoadTexture("assets/piece/white/knight.png"),
                                LoadTexture("assets/piece/white/rook.png"),
                                LoadTexture("assets/piece/white/king.png"),
                                LoadTexture("assets/piece/white/queen.png")};
    Texture2D black_piece[7] = {{}, //
                                LoadTexture("assets/piece/black/pawn.png"),
                                LoadTexture("assets/piece/black/bishop.png"),
                                LoadTexture("assets/piece/black/knight.png"),
                                LoadTexture("assets/piece/black/rook.png"),
                                LoadTexture("assets/piece/black/king.png"),
                                LoadTexture("assets/piece/black/queen.png")};

    // white piece eated BY black
    std::vector<Piece> eated_white;                            
    // black piece eated BY white
    std::vector<Piece> eated_black;                            

    void Eat(pair pos);
};

#endif