import requests 
import json 

PORT: int = 8080
BASE_URL: str = "http://localhost:" + str(PORT)
MESSAGE_URL: str = BASE_URL + "/send-message"



def test_message(message: str) -> object:

    token: str = "" 
    authHeaders = {"Authorization" : f"Bearer {token}", "Content-Type" : "application/json"}   
    messagePayload = {"message" : message} 
    response = requests.post(url=MESSAGE_URL, headers=authHeaders, json=messagePayload)
    if response.status_code == 200:
        data = response.json()
        print(json.dumps(data, indent=4))
    else:
        print(f'Request failed with status code: {response.status_code}')
        print(f'Error: {response.text}')
        
    



if __name__ == "__main__":
    
    test_message("Hello, world!")