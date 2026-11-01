#include "Board.hpp"
#include <iostream>
#include <cstdint>

using namespace std;

void Board::Draw()
{
    // cout << chess_size << ", " << cell_size << ", " << sprite_size << ", " << start_x_draw << ", " << width << ", " << height << ", " << endl;

    if (IsWindowResized())
    {
        width = GetRenderWidth();
        height = GetRenderHeight();
        cell_size = (width > height ? height : width) / chess_size;
        start_x_draw = (width - (cell_size * chess_size)) / 2;
    }

    for (int y = 0; y < chess_size; y++)
    {
        for (int x = 0; x < chess_size; x++)
        {
            DrawTexturePro(grid_texture[(x + y) % 2], {0, 0, (float)sprite_size, (float)sprite_size}, {(float)(start_x_draw + (x * cell_size)), (float)(y * cell_size), (float)cell_size, (float)cell_size}, {0, 0}, 0.0f, WHITE);

            if (selected && x == select_cell.x && y == select_cell.y)
                DrawRectanglePro({(float)(start_x_draw + (x * cell_size)), (float)(y * cell_size), (float)cell_size, (float)cell_size}, {0, 0}, 0.0f, RED);
        }
    }

    for (int y = 0; y < chess_size; y++)
    {
        for (int x = 0; x < chess_size; x++)
        {
            uint8_t value = board[y * chess_size + x];

            if (value == 0)
                continue;

            bool color = (value & 0b10000000) != 0;
            uint8_t piece_id = value & 0b01111111;

            Texture2D piece = color ? white_piece[piece_id] : black_piece[piece_id];
            DrawTexturePro(piece, {0, 0, (float)sprite_size, (float)sprite_size}, {(float)(start_x_draw + (x * cell_size)), (float)(y * cell_size), (float)cell_size, (float)cell_size}, {0, 0}, 0.0f, WHITE);
        }
    }


    //     // white piece eated BY black
    // std::vector<Piece> eated_white;                            
    // // black piece eated BY white
    // std::vector<Piece> eated_black;                            


    // draw bottom left piece eated by white

    // draw top right pieace eated by black


    // by number or by order with little pieces

    if (selected)
    {
        for (const auto pos : GetLegalMove(select_cell))
        {
            DrawCircle(start_x_draw + pos.x * cell_size + cell_size / 2, pos.y * cell_size + cell_size / 2, cell_size * 0.25, Color{130, 130, 130, 125});
        }
    }
}

std::vector<Board::pair> Board::GetLegalMove(Board::pair pos)
{
    vector<Board::pair> l_pos;

    uint8_t pvalue = board[pos.x + pos.y * chess_size];

    Board::Piece p = static_cast<Piece>(pvalue & 0b01111111);
    bool color = (pvalue & 0b10000000) != 0;

    // printf("%d\n", pvalue);
    // cout << "piece: " << (pvalue & 0b01111111) << " color: " << (color ? "white" : "black") << endl;

    switch (p)
    {
    case Board::Piece::Pawn:
        if (color)
        {
            if (board[pos.x + (pos.y - 1) * chess_size] == 0)
            {
                if (pos.y == chess_size - 2)
                {
                    l_pos.push_back(Board::pair{pos.x, pos.y - 2});
                }
                l_pos.push_back(Board::pair{pos.x, pos.y - 1});
            }
            if (pos.x > 0)
            {
                uint8_t value = board[pos.x - 1 + (pos.y - 1) * chess_size];
                if (value != 0 && (!((value & 0b10000000) != 0) && color))
                    l_pos.push_back(Board::pair{pos.x - 1, pos.y - 1});
            }
            if (pos.x < chess_size - 1)
            {
                uint8_t value = board[pos.x + 1 + (pos.y - 1) * chess_size];
                if (value != 0 && (!((value & 0b10000000) != 0) && color))
                    l_pos.push_back(Board::pair{pos.x + 1, pos.y - 1});
            }
        }
        else
        {
            if (board[pos.x + (pos.y + 1) * chess_size] == 0)
            {
                if (pos.y == 1)
                {
                    l_pos.push_back(Board::pair{pos.x, pos.y + 2});
                }
                l_pos.push_back(Board::pair{pos.x, pos.y + 1});
            }
            if (pos.x > 0)
            {
                uint8_t value = board[pos.x - 1 + (pos.y + 1) * chess_size];
                if (value != 0 && (((value & 0b10000000) != 0) && !color))
                    l_pos.push_back(Board::pair{pos.x - 1, pos.y + 1});
            }
            if (pos.x < chess_size - 1)
            {
                uint8_t value = board[pos.x + 1 + (pos.y + 1) * chess_size];
                if (value != 0 && (((value & 0b10000000) != 0) && !color))
                    l_pos.push_back(Board::pair{pos.x + 1, pos.y + 1});
            }
        }
        break;
    }

    return l_pos;
}

void Board::SelectPiece(Vector2 pos)
{

    if (pos.x < start_x_draw || pos.x > start_x_draw + cell_size * chess_size)
    {
        selected = false;
        return;
    }

    pos.x -= start_x_draw;
    select_cell.x = pos.x / cell_size;
    select_cell.y = pos.y / cell_size;

    selected = board[select_cell.x + select_cell.y * chess_size] != 0;
}

void Board::MovePiece(Vector2 pos)
{
    if (pos.x < start_x_draw || pos.x > start_x_draw + cell_size * chess_size)
    {
        selected = false;
        return;
    }
    Board::pair tmp;
    pos.x -= start_x_draw;
    tmp.x = pos.x / cell_size;
    tmp.y = pos.y / cell_size;
    selected = board[tmp.x + tmp.y * chess_size] != 0;

    for (const auto pos : GetLegalMove(select_cell))
    {
        if (pos.x == tmp.x && pos.y == tmp.y)
        {
            Eat(tmp);
            board[tmp.x + tmp.y * chess_size] = board[select_cell.x + select_cell.y * chess_size];
            board[select_cell.x + select_cell.y * chess_size] = 0;
            selected = false;
        }
    }

    select_cell.x = tmp.x;
    select_cell.y = tmp.y;
}

void Board::Eat(pair pos)
{
    uint8_t value = board[pos.x+pos.y*chess_size];
    if(value == 0) return;
    Piece p = static_cast<Piece>(value&0b01111111);
    if((value & 0b10000000) != 0) {
        eated_white.push_back(p);
    } else {
        eated_black.push_back(p);
    }
}
