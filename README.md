# Compress

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
