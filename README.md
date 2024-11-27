# Chunker CLI

`Chunker CLI` is a fast and efficient tool to split large files into smaller chunks and merge them back together. It leverages concurrency for the splitting operation, allowing for faster processing of large files.

## 🚀 Features

- **Split Files**: Divide large files into smaller, manageable chunks.
- **Merge Chunks**: Reassemble previously split files back into their original form.
- **Concurrency**: Split files using multiple concurrent tasks to speed up the process.
- **Custom Chunk Size**: Control the size of the chunks in bytes.

## 📦 Installation

To install and use `chunker-cli`, follow these simple steps:

1. Go to the [releases](https://github.com/pyyupsk/chunker-cli/releases) page and download the `chunker-cli` executable.

2. Run the CLI tool in your terminal. For example:

    ```bash
    chunker-cli split <SOURCE_FILE_PATH> -o <OUTPUT_DIRECTORY>
    ```

3. That's it! The tool will handle the rest, splitting the file into smaller chunks and merging them back together.

## 🧩 Usage

### Split a File into Chunks

Use the `split` command to divide a large file into smaller chunks.

```bash
chunker-cli split <SOURCE_FILE_PATH> [--output <OUTPUT_DIRECTORY>] [--concurrent <NUM_CONCURRENT_TASKS>] [--chunk-size <CHUNK_SIZE>]
```

#### Options

- `--output <OUTPUT_DIRECTORY>`: Directory where the chunk files will be saved. If not provided, the default is `<SOURCE_FILE_NAME>_chunks`.
- `--concurrent <NUM_CONCURRENT_TASKS>`: Number of concurrent tasks to run. Default is `5`.
- `--chunk-size <CHUNK_SIZE>`: Size of each chunk in bytes (in binary format). Default is `24.5MB` (25690112 bytes).

#### Example

```bash
chunker-cli split large_file.txt --output ./chunks --concurrent 5 --chunk-size 10485760
```

This will split the `large_file.txt` into chunks of 10 MB and save them in the `./chunks` directory using 5 concurrent tasks.

### Merge File Chunks

Use the `merge` command to combine chunks back into a single file.

```bash
chunker-cli merge <CHUNKS_DIRECTORY> <OUTPUT_FILE_PATH>
```

#### Example

```bash
chunker-cli merge ./chunks merged_file.txt
```

This will merge the chunks in the `./chunks` directory back into a single file called `merged_file.txt`.

## 🎉 Contributing

Contributions are welcome! If you'd like to improve `chunker-cli`, feel free to fork the repository and submit a pull request.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Enjoy splitting and merging your files with `chunker-cli`! 🚀
