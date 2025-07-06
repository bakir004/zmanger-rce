// Save as sendRequests.js
import fetch from "node-fetch";

const url = "http://localhost:3000/queue";

const body = {
  code: `#include <iostream>
#include <vector>

int main() {
    int n;
    std::cin >> n;
    std::cout << n << std::endl;
    std::cout.flush();
    std::vector<int> v(n);
    for (int i = 0; i < n; i++) {
        v[i] = i;
    }
    std::cout << v.size() << std::endl;
    int sum = 0;
    for (int i = 0; i < n; i++) {
        sum += v[i];
    }
    std::cout << sum << std::endl;

 return 0;
}
`,
  stdin: "2000",
  expected_output: ["2000\n2000\n1999000\n"],
  language_id: 1,
};

async function sendRequest(i) {
  try {
    const res = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });

    const data = await res.json();
    console.log(`Response #${i}:`, data);
  } catch (e) {
    console.error(`Error on request #${i}:`, e);
  }
}

async function main() {
  for (let i = 1; i <= 20; i++) {
    sendRequest(i);
    await new Promise((r) => setTimeout(r, 100)); // 100 ms delay
  }
}

main();
