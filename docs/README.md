# ftpbrutal

<strong>ftpbrutal</strong> is a brute-force tool for FTP servers. It attempts to connect by testing passwords from a provided wordlist until a successful connection is established.


## How to use

### Setup

1. Clone the repository to your local machine:

    ```bash
    $ git clone https://github.com/voidex1/ftpbrtual.git
    ```


2. Compile the code:

    ```bash
    $ cd ftpbrtual
    $ cargo build --release
    ```

3. Spin up the container with the FTP server using Docker:

    ```bash
    $ docker-compose up -d
    ```

### Usage Examples
Here are a few examples to demonstrate how to use the tool:

- Basic usage:

    ```bash
   $ cargo run -- --u john --w rockyouu.txt --p 21 --ip 127.0.0.3
    ```
- Or

   ``` bash
   $ ./target/release/ftp-brutal --u john --w rockyouu.txt --p 21 --ip 127.0.0.3
   ```

## System Requirements

- Rust: Make sure you have Rust installed on your system. You can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).