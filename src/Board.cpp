#include "Board.hpp"
#include <iostream>
#include <cstdint>
#include <array>

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
    bool pcolor = (pvalue & 0b10000000) != 0;

    // printf("%d\n", pvalue);
    // cout << "piece: " << (pvalue & 0b01111111) << " color: " << (color ? "white" : "black") << endl;

    switch (p)
    {
    case Board::Piece::Pawn:
        if (pcolor)
        {
            if (board[pos.x + (pos.y - 1) * chess_size] == 0)
            {
                if (pos.y == chess_size - 2)
                    if (board[pos.x + (pos.y - 2) * chess_size] == 0)
                        l_pos.push_back(Board::pair{pos.x, pos.y - 2});

                l_pos.push_back(Board::pair{pos.x, pos.y - 1});
            }
            if (pos.x > 0)
            {
                uint8_t value = board[pos.x - 1 + (pos.y - 1) * chess_size];
                if (value != 0 && (!((value & 0b10000000) != 0) && pcolor))
                    l_pos.push_back(Board::pair{pos.x - 1, pos.y - 1});
            }
            if (pos.x < chess_size - 1)
            {
                uint8_t value = board[pos.x + 1 + (pos.y - 1) * chess_size];
                if (value != 0 && (!((value & 0b10000000) != 0) && pcolor))
                    l_pos.push_back(Board::pair{pos.x + 1, pos.y - 1});
            }
        }
        else
        {
            if (board[pos.x + (pos.y + 1) * chess_size] == 0)
            {
                if (pos.y == 1)
                    if (board[pos.x + (pos.y + 2) * chess_size] == 0)
                        l_pos.push_back(Board::pair{pos.x, pos.y + 2});

                l_pos.push_back(Board::pair{pos.x, pos.y + 1});
            }
            if (pos.x > 0)
            {
                uint8_t value = board[pos.x - 1 + (pos.y + 1) * chess_size];
                if (value != 0 && (((value & 0b10000000) != 0) && !pcolor))
                    l_pos.push_back(Board::pair{pos.x - 1, pos.y + 1});
            }
            if (pos.x < chess_size - 1)
            {
                uint8_t value = board[pos.x + 1 + (pos.y + 1) * chess_size];
                if (value != 0 && (((value & 0b10000000) != 0) && !pcolor))
                    l_pos.push_back(Board::pair{pos.x + 1, pos.y + 1});
            }
        }
        break;

    case Board::Piece::Bishop:
    {
        vector<Board::pair> bdirections = {
            {1, 1},
            {1, -1},
            {-1, 1},
            {-1, -1}};

        multidirection(l_pos, bdirections, pos, pcolor);

        break;
    }

    case Board::Piece::Rook:
    {
        vector<Board::pair> rdirections = {
            {1, 0},
            {-1, 0},
            {0, 1},
            {0, -1}};

        multidirection(l_pos, rdirections, pos, pcolor);

        break;
    }

    case Board::Piece::Queen:
    {
        vector<Board::pair> qdirections = {
            {1, 1},
            {1, -1},
            {-1, 1},
            {-1, -1},
            {1, 0},
            {-1, 0},
            {0, 1},
            {0, -1}};

        multidirection(l_pos, qdirections, pos, pcolor);

        break;
    }

    case Board::Piece::Knight:
    {
        vector<pair> poss = {
            {-1, 2},
            {1, 2},
            {-1, -2},
            {1, -2},
            {-2, 1},
            {-2, -1},
            {2, -1},
            {2, 1},
        };
        for (auto current : poss)
        {
            current.x += pos.x;
            current.y += pos.y;
            if (!(current.x >= 0 && current.x < chess_size && current.y >= 0 && current.y < chess_size))
                continue;
            uint8_t value = board[current.x + current.y * chess_size];
            if (value != 0)
            {
                if (((value & 0b10000000) != 0) ^ pcolor)
                {
                    l_pos.push_back(current);
                }
            }
            else
                l_pos.push_back(current);
        }
        break;
    }
    case Board::Piece::King:
    {
        for (int y = pos.y - 1; y < pos.y + 1; y++)
        {
            for (int x = pos.x - 1; x < pos.x + 1; x++)
            {
                if (!(x >= 0 && x < chess_size && y >= 0 && y < chess_size))
                    continue;
                uint8_t value = board[x + y * chess_size];
                if (value != 0)
                    if (!(((value & 0b10000000) != 0) ^ pcolor))
                        if (!IsAttackBy(pair{x, y}, !pcolor))
                            l_pos.push_back(pair{x, y});
            }
        }
        break;
    }
    }

    return l_pos; // return specific value for check the king, if moving bishop do check return a thing like {-12, -12}
}

bool Board::IsAttackBy(pair pos, bool by)
{
    vector<pair> directions = {
        {1, 1},
        {1, -1},
        {-1, 1},
        {-1, -1},
        {1, 0},
        {-1, 0},
        {0, 1},
        {0, -1},
    };

    for (const auto dir : directions)
    {
        Board::pair current = pos;

        current.x += dir.x;
        current.y += dir.y;
        while (current.x < chess_size && current.y < chess_size && current.x >= 0 && current.y >= 0)
        {
            uint8_t value = board[current.x + current.y * chess_size];
            if (value != 0)
                return ((value & 0b10000000) != 0) ^ by;
            current.x += dir.x;
            current.y += dir.y;
        }
    }

    vector<pair> poss = {
        {-1, 2},
        {1, 2},
        {-1, -2},
        {1, -2},
        {-2, 1},
        {-2, -1},
        {2, -1},
        {2, 1},
    };
    for (auto current : poss)
    {
        current.x += pos.x;
        current.y += pos.y;
        if (!(current.x >= 0 && current.x < chess_size && current.y >= 0 && current.y < chess_size))
            continue;
        uint8_t value = board[current.x + current.y * chess_size];
        if (value != 0)
            return (((value & 0b10000000) != 0) ^ by);
    }
    return false;
}

void Board::multidirection(std::vector<Board::pair> &l_pos, std::vector<Board::pair> &directions, Board::pair pos, bool pcolor)
{
    for (const auto dir : directions)
    {
        Board::pair current = pos;

        current.x += dir.x;
        current.y += dir.y;
        while (current.x < chess_size && current.y < chess_size && current.x >= 0 && current.y >= 0)
        {
            uint8_t value = board[current.x + current.y * chess_size];
            if (value != 0)
            {
                bool color = (value & 0b10000000) != 0;
                if (pcolor ^ color)
                {
                    l_pos.push_back(current);
                }
                break;
            }
            l_pos.push_back(current);

            current.x += dir.x;
            current.y += dir.y;
        }
    }
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

    for (const auto _pos : GetLegalMove(select_cell))
    {
        if (_pos.x == tmp.x && _pos.y == tmp.y)
        {
            Eat(tmp);
            uint8_t value = board[select_cell.x + select_cell.y * chess_size];
            uint8_t toeated = board[tmp.x + tmp.y * chess_size];
            bool color = (value & 0b10000000) != 0;
            board[tmp.x + tmp.y * chess_size] = value;
            board[select_cell.x + select_cell.y * chess_size] = 0;
            pair kp = getKingPos(color);
            if (IsAttackBy(kp, !color))
                UndoMovePiece(Vector2{(float)tmp.x, (float)tmp.y}, toeated);
            selected = false;
            pair ekp = getKingPos(!color);
            white_check = IsAttackBy(ekp, color);
            break;
        }
    }

    select_cell.x = tmp.x;
    select_cell.y = tmp.y;
}

void Board::UndoMovePiece(Vector2 pos, uint8_t undoeat)
{

//    board[tmp.x + tmp.y * chess_size] = value;
  //  board[select_cell.x + select_cell.y * chess_size] = 0;
}

void Board::Eat(pair pos)
{
    uint8_t value = board[pos.x + pos.y * chess_size];
    if (value == 0)
        return;
    Piece p = static_cast<Piece>(value & 0b01111111);
    if ((value & 0b10000000) != 0)
    {
        eated_white.push_back(p);
    }
    else
    {
        eated_black.push_back(p);
    }
}

Board::pair Board::getKingPos(bool color)
{
    for (int y = 0; y < chess_size; y++)
    {
        for (int x = 0; x < chess_size; x++)
        {
            uint8_t v = board[x + y * chess_size];
            if (v != 0 && Piece::King == static_cast<Piece>(v & 0b01111111) && !(((v & 0b10000000) != 0) ^ color))
                return pair{x, y};
        }
    }
}
