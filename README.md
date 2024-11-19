# hxr

A simple HTTP request CLI tool

---

## Installation

```zsh
cargo install hxr
```

##### Ensure `~/.cargo/bin` is in your `PATH`
```bash
echo $PATH
```

- If needed, add it manually to your shell configuration:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

- Run the `hxr` command directly from the terminal:
```bash
hxr --help
```

---

## Usage

- Make a `GET` request to [api.ipify.org](https://api.ipify.org/)
    - Returns your IP address
```zsh
hxr https://api.ipify.org/
```

- Make a `POST` request
```zsh
hxr -m POST https://api.example.org/ -b bodyToSend
```

### Adding headers

##### With Headers

Run the command with headers:
- Adding JSON as the content type will prettify the JSON in the terminal
```bash
hxr --method GET https://httpbin.org/headers -h "content-type:application/json" -h "Authorization:Bearer token123"
```

##### Without Headers

Run a simple GET request:
```bash
hxr -m GET https://api.ipify.org/
```

---