import requests
import json

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

with open("database/belgium_cities.json", "w", encoding="utf-8") as f:
    json.dump(data, f, ensure_ascii=False, indent=2)
