#include "raylib.h"
#include "Board.hpp"
#include <iostream>

const unsigned int width = 1920, height = 1080, chess_size = 8, sprite_size = 32;

int main()
{

    InitWindow(width, height, "Chess");

    Board *chess = new Board();

    for (const auto p : chess->GetLegalMove({3, 3}))
    {
        std::cout << p.x << ", " << p.y << std::endl;
    }

    SetTargetFPS(144);

    while (!WindowShouldClose() && !IsKeyPressed(KEY_ESCAPE))
    {

        if (chess->selected)
        {
            if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT))
                chess->MovePiece(GetMousePosition());
        }
        else if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT))
            chess->SelectPiece(GetMousePosition());

        BeginDrawing();
        ClearBackground(GRAY);
        chess->Draw();
        EndDrawing();
    }

    delete chess;
    CloseWindow();
    return 0;
}
