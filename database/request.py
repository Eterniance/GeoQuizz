import requests
import json
from pathlib import Path

def filter_json(data):
    filtered = []
    for el in data.get("elements", []):
        tags = el.get("tags", {})
        name_fr = tags.get("name:fr")
        name_nl = tags.get("name:nl")
        name_default = tags.get("name")

        filtered.append({
            "name:default": name_default,
            "name:fr": name_fr,
            "name:nl": name_nl,
            "lat": el["lat"],
            "lon": el["lon"]
        })
    
    return filtered

def main():
    url = "https://overpass-api.de/api/interpreter"
    headers = {"User-Agent": "personal_geoquizz_game"}

    query = """
    [out:json][timeout:180];
    area["ISO3166-1"="BE"][admin_level=2]->.country;
    node["place"~"city|town"](area.country);
    out;
    """

    response = requests.post(url, data={"data": query}, headers=headers)
    response.raise_for_status()
    data = response.json()

    print(f"Found {len(data.get('elements', []))} city nodes in Belgium")

    filtered = filter_json(data)

    print(f"Elements remaining after filtering: {len(filtered)}")

    filepath = script_dir = Path(__file__).parent.resolve() / "belgium_cities.json"
    
    with open(filepath, "w", encoding="utf-8") as f:
        json.dump(filtered, f, ensure_ascii=False, indent=2)

if __name__ == "__main__":
    main()
