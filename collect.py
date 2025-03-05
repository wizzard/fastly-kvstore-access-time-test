# python collect.py https://fastly-service/ 100

import sys
import requests
import json
import time

TEST_HDR_KEY = "x-test-enabled"
TEST_HDR_VALUE = "1"
TEST_PATH = "/test_kv"
CSV_FILE = "kv_access_times.csv"

def main():
    if len(sys.argv) != 3:
        print("Usage: python collect.py [SERVICE URL] [requests to send]")
        sys.exit(1)

    url = sys.argv[1]
    req_to_send = int(sys.argv[2])
    times = []

    for i in range(req_to_send):
        print(f"Sending request {i+1} out of {req_to_send} ...")
        response = requests.get(url + TEST_PATH, headers={TEST_HDR_KEY: TEST_HDR_VALUE})
        if response.status_code == 200:
            try:
                data = response.json()
                open_time = data.get("open_time")
                read_time = data.get("read_time")
                open_time = int(open_time) / 1000 # convert to ms
                read_time = int(read_time) / 1000 # convert to ms

                times.append([open_time, read_time])
            except json.JSONDecodeError:
                print(f"Request {i+1}: Failed to decode JSON")
        else:
            print(f"Request {i+1}: Failed with status code {response.status_code}")
        time.sleep(0.1)

    if times:
        with open(CSV_FILE, "w") as csv_file:
            csv_file.write("KVStore::open() time(ms),KVStore::lookup() time(ms)\n")
            for t in times:
                csv_file.write(f"{t[0]},{t[1]}\n")
        print("KV Store access times saved to:  " + CSV_FILE)


if __name__ == "__main__":
    main()