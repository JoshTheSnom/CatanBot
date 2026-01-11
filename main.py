import math
from dataclasses import dataclass
from typing import Optional, Tuple

import matplotlib.pyplot as plt
from matplotlib.patches import Polygon


@dataclass(frozen=True)
class Hex:
    id: int
    res: str
    num: Optional[int]
    # axial coordinates
    q: int
    r: int


data = [
    (1, "ore", 10, -2, 2),
    (2, "sheep", 2, -1, 2),
    (3, "wood", 9, 0, 2),
    (4, "wheat", 12, -2, 1),
    (5, "brick", 6, -1, 1),
    (6, "sheep", 4, 0, 1),
    (7, "brick", 10, 1, 1),
    (8, "wheat", 9, -2, 0),
    (9, "wood", 11, -1, 0),
    (10, "desert", None, 0, 0),
    (11, "wood", 3, 1, 0),
    (12, "ore", 8, 2, 0),
    (13, "wood", 8, -1, -1),
    (14, "ore", 3, 0, -1),
    (15, "wheat", 4, 1, -1),
    (16, "sheep", 5, 2, -1),
    (17, "brick", 5, 0, -2),
    (18, "wheat", 6, 1, -2),
    (19, "sheep", 11, 2, -2),
]

HEXES = [Hex(id=d[0], res=d[1], num=d[2], q=d[3], r=d[4]) for d in data]

COLORS = {
    "wood": "#2D8C24",
    "brick": "#D9532B",
    "sheep": "#78B800",
    "wheat": "#F2CB30",
    "ore": "#A9ADAE",
    "desert": "#DFD8B1",
}

hex_size = 1.0


def axial_to_pixel(q: int, r: int) -> Tuple[float, float]:
    x = hex_size * (math.sqrt(3) * (q + r / 2))
    y = hex_size * (3 / 2 * r)
    return x, y


def hex_corners(x: float, y: float):
    corners = []
    for i in range(6):
        angle = math.radians(60 * i + 30)
        corners.append((x + hex_size * math.cos(angle), y + hex_size * math.sin(angle)))
    return corners


def draw_board(hex_list):
    fig, ax = plt.subplots(figsize=(8, 8))

    all_x, all_y = [], []

    for h in hex_list:
        x, y = axial_to_pixel(h.q, h.r)
        corners = hex_corners(x, y)

        all_x.append(x)
        all_y.append(y)

        poly = Polygon(corners, facecolor=COLORS[h.res], edgecolor="black", lw=2)
        ax.add_patch(poly)

        

        if h.num is not None:

            font_relative_size = 28-int(abs((h.num)-7.0))*2.5
            color = "#000000"

            if h.num == 8 or h.num == 6:
                color = "#ffffff"
            
            
            ax.text(
                x,
                y,
                str(h.num),
                ha="center",
                va="center",
                fontsize=font_relative_size,
                fontweight="extra bold",
                color=color
            )

    limit = 4.5
    ax.set_xlim(-limit, limit)
    ax.set_ylim(-limit, limit)

    ax.set_aspect("equal")
    ax.axis("off")
    plt.title("Catan Board")
    plt.show()


if __name__ == "__main__":
    draw_board(HEXES)
