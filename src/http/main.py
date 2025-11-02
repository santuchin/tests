import requests

domain = "santuchin.xyz"

api_key = \
"pk1_0c86013e238290a460f3e16dbcd4bba68f7cc7fee6ad60b385d620fbf805e45a"
secret_api_key = \
"sk1_9d70d0a4ccda110feed0fff56ba320d33b533e30e392d85835f6bfe617b3952e"

endpoint = "https://api.porkbun.com/api/json/v3/" + f"ping" 

data = {
	"secretapikey": secret_api_key,
	"apikey": api_key,
}


resp = requests.post(
	endpoint,
	data=data,
)

print(resp.content)

