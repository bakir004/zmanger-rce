# Remote C++ Code Execution Tool

Zmanger RCE is a simple, minimal remote code execution tool written in Rust. For now, it can only run C and C++ code, and adding more languages is not planned. It leverages Podman to run user-submitted code in isolation.

## Setup

Zmanger RCE itself is meant to be run as a Docker/Podman container using the following instructions.

### Using Docker/Podman

**Note: this Docker image is built for the arm64 architecture, and has not been tested for x86_64. Users on x86_64 may need to build the image locally or run under an emulation layer.**

*These commands work with `docker` and `podman`, just swap out the keyword*

```bash
podman pull docker.io/bakeroni1/cpp-runner:latest

podman run --rm -p 3001:3001 -d bakeroni1/cpp-runner
```

### Using Cargo

Zmanger RCE can also be compiled and run as a Rust app, only after installing the rust toolchain `rustc` and package manager `cargo`.

## Usage

After spinning up an instance of the RCE, you can send C and C++ code to the endpoints.

#### `POST /submissions`

### Request Body

For future reference, call this object format `Submission`.

| Field             | Type            | Required | Description                                           |
|------------------|-----------------|----------|-------------------------------------------------------|
| `code`           | `string`        | Yes      | The source code to run.                               |
| `stdin`          | `string`        | No       | Optional standard input to pass to the program.      |
| `expected_output`| `array[string]` | Yes       | Expected output lines for validation (empty array if no specific output is expected).     |
| `language_id`    | `integer`       | Yes      | 1 for C++, 2 for C.      |

### Response Body

| Field               | Type     | Description                                          |
|--------------------|----------|------------------------------------------------------|
| `compile_output`   | `string` | Compiler output (errors/warnings).                   |
| `stdout`           | `string` | Program standard output.                              |
| `stderr`           | `string` | Program standard error output.                        |
| `time`             | `integer`| Execution time in milliseconds.                       |
| `runtime_status`   | `integer`| Status code of program execution (0 = success).      |
| `submission_status`| `integer`| Status of submission. |
| `description`      | `string` | Human-readable description of result.                |

### Submission Status Codes

| Value | Description                |
|-------|---------------------------------|
| 0     | Accepted              | Accepted                        |
| 1     | Core Accepted                   |
| 2     | Wrong Answer                     |
| 3     | Compilation Error                |
| 4     | Time Limit Exceeded              |
| 5     | Memory Limit Exceeded            |
| 6     | Memory Leak Detected             |
| 7     | Memory Error                     |
| 8     | Runtime Error                    |

#### Example

Example body:
```json
{
  "code": "#include<iostream>\nint main() {\n std::cout << \"selamun aleykum dunya\"; return 0;}",
  "stdin": "",
  "expected_output": ["selamun aleykum dunya"],
  "language_id": 1
}
```
Example response:
```json
{
  "compile_output": "",
  "stdout": "selamun aleykum dunyaa",
  "stderr": "",
  "time": 13557,
  "runtime_status": 0,
  "submission_status": 0,
  "description": "Accepted"
}
```

#### `POST /submissions/batch`

### Request Body

Request body for batch execution consists of an array of objects with properties `id` and `submission`, where submission is the aforementioned `Submission` object used in the `/submissions` endpoint request body.

### Response body

Response body consists of an array `results` of objects with properties `id` and `result`, where `result` is the same object returned by the previous endpoint.

Example body:
```json
[
  {
    "id": 0,
    "submission": {
      "code": "#include<iostream>\nint main() {\n std::cout << \"hello one\"; return 0;}",
      "stdin": "",
      "expected_output": ["hello one"],
      "language_id": 1
    }
  },
  {
    "id": 1,
    "submission": {
      "code": "#include<iostream>\nint main() {\n std::cout << \"hello two\"; return 0;}",
      "stdin": "",
      "expected_output": ["one"],
      "language_id": 1
    }
  }
]
```
Example response:
```json
{
  "results": [
    {
      "id": 0,
      "result": {
        "compile_output": "",
        "stdout": "hello one",
        "stderr": "",
        "time": 14300,
        "runtime_status": 0,
        "submission_status": 0,
        "description": "Accepted"
      }
    },
    {
      "id": 1,
      "result": {
        "compile_output": "",
        "stdout": "hello two",
        "stderr": "",
        "time": 13526,
        "runtime_status": 0,
        "submission_status": 2,
        "description": "Wrong Answer"
      }
    }
  ]
}
```

It is the responsibility of the user to assign id's to each submission and the rearrange them after the API's response.