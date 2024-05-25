# Compress

![image](https://github.com/lucatsf/compress/assets/18267941/d006f553-3c95-4c0b-917c-dd8921fbf60f)


A command-line utility for compressing files using gzip or zip formats.

## Requirements

- Rust (to compile the source code)
- Cargo (Rust's package manager)

## Installation

### Downloading and Installing

1. Clone the repository:

    ```sh
    git clone https://github.com/username/compress.git
    cd compress
    ```

2. Compile the project:

    ```sh
    cargo build --release
    ```

3. You can run the project execute .install.sh or 
    1. Copy the compiled binary to the `/opt/compress` directory:

        ```sh
        sudo mkdir -p /opt/compress
        sudo cp target/release/compress /opt/compress/
        sudo chmod +x /opt/compress/compress
        ```

    2. Add an alias to your `.bashrc`:

        ```sh
        echo "alias compress='/opt/compress/compress'" >> ~/.bashrc
        source ~/.bashrc
        ```

5. Verify the installation:

    ```sh
    compress --help
    ```

## Usage

The program supports two compression types: gzip and zip.

### Example Usage:

- Gzip compression:

    ```sh
    compress gzip source.txt source.txt.gz
    ```

- Zip compression:

    ```sh
    compress zip source.txt source.zip
    ```

### Help

To see the usage options:

```sh
compress --help
```

## Contribution

If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License
`This project is licensed under the terms of the MIT license.