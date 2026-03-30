import requests
import pprint
import json

# Gets the last thousand game ids.
def get_ids():
    ids = []

    for page in range(1, 11):
        url = f"https://hexo.did.science/api/finished-games?page={page}&pageSize=100"
        response = requests.get(url)
        for game in games["games"]:
            ids.append(game["id"])


    with open(f'ids.json', 'w') as f:
        json.dump(ids, f, indent=4)


def get_game(id):
    url = f"https://hexo.did.science/api/finished-games/{id}"
    response = requests.get(url)
    return response.json()


def get_all_games():
    games = []

    with open('ids.json', 'r') as f:
        ids = json.load(f)

    i = 0
    for id in ids:
        game = get_game(id)
        games.append(game)
        print(f"Got game {i}")
        i += 1
        
    with open(f'games.json', 'w') as f:
        json.dump(games, f, indent=4)


# The API endpoint
main_url = "https://hexo.did.science/api/finished-games?page=1&pageSize=100"

# url = "https://hexo.did.science/api/finished-games/b20d4ee8-0dd3-414a-8725-9b8d4db6fd8b"

# A GET request to the API
# response = requests.get(main_url)

# Print the response
# pprint.pp(response.json())

# Put response into file
# with open('response.json', 'w') as f:
#     json.dump(response.json(), f, indent=4)

# Read the JSON file from the file response.json:
# with open('response.json', 'r') as f:
#     games = json.load(f)

# Get a string from the user and test what functionality they want to use:
user_input = input("Enter command: ")

if user_input == "get_ids":
    get_ids()
elif user_input == "get_all_games":
    get_all_games()
elif user_input == "test":
    id = "6a725ce6-5853-4ec6-82a6-1f82c86a7341"
    game = get_game(id)
    pprint.pp(game)
else:    
    print("Invalid command.")