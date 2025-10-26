import json
import math
from pathlib import Path
X0 = math.radians(4.5)
PATH = Path(__file__).parent.resolve() / "belgium_cities.json"

def average_xy():
    with open(PATH, 'r') as file:
        data = json.load(file)
    lat0 = 0
    lon0 = 0
    for el in data:
        lat0 += el["lat"]
        lon0 += el["lon"]
    lat0 /= len(data)
    lon0 /= len(data)

    print(f"lat0 = {lat0}")
    print(f"lon0 = {lon0}")

def mercator(long,lat):
    x = math.radians(long)
    y = math.radians(lat)
    new_x = (x-X0)
    new_y = math.log(math.tan(0.25*math.pi+0.5*y))
    return (new_x,new_y)

def compute_scale(s1,s2,s_target2, s_target1):
    return (s_target2-s_target1)/(s2-s1)

def compute_offset(s1, s_target1, scale):
    return s_target1 - (scale*s1)


def main():
    with open(PATH, 'r') as file:
        data = json.load(file)

    target = "Soignies"
    for el in data:
        if el["name:fr"] == "Arlon":
            lat = float(el["lat"])
            lon = float(el["lon"])
            xA,yA = mercator(lon,lat)
            
        if el["name:fr"] == "Ostende":
            lat = float(el["lat"])
            lon = float(el["lon"])
            xO,yO = mercator(lon,lat)

        if el["name:fr"] == target:
            lat = float(el["lat"])
            lon = float(el["lon"])
            xB,yB = mercator(lon,lat)
    target_Ax = 258.0
    target_Ay = -248.0
    target_Ox = -307.0
    target_Oy = 207.0

    a_x = compute_scale(xA, xO, target_Ox, target_Ax)
    b_x = compute_offset(xA, target_Ax, a_x)
    print(f"x scaling: a = {a_x}, b={b_x}")

    a_y = compute_scale(yA, yO, target_Oy, target_Ay)
    b_y = compute_offset(yA, target_Ay, a_y)
    print(f"y scaling: a = {a_y}, b={b_y}")

    new_xB = a_x*xB+b_x
    new_yB = a_y*yB+b_y
    print(f"{target} coordinate = {new_xB},{new_yB}")

if __name__ == "__main__":
    main()