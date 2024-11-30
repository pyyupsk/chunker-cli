# Chunker CLI

`Chunker CLI` is a high-performance tool designed to split large files into smaller chunks and merge them back together. It leverages parallel processing for both splitting and merging operations, delivering exceptional performance and resource efficiency.

## 🚀 Features

- **Parallel Split**: Divide large files into smaller chunks using parallel processing
- **Parallel Merge**: Rapidly reassemble chunks back into the original file
- **High Performance**: Efficiently splits and merges large files
- **Resource Efficient**: Uses minimal memory and CPU resources
- **Custom Chunk Size**: Control the size of chunks in bytes

## 📦 Installation

To install and use `chunker-cli`:

1. Go to the [releases](https://github.com/pyyupsk/chunker-cli/releases) page
2. Download the `chunker-cli` executable
3. Run it from your terminal:

```bash
chunker-cli split <SOURCE_FILE_PATH> -o <OUTPUT_DIRECTORY>
```

## 🧩 Usage

### Split a File into Chunks

```bash
chunker-cli split <SOURCE_FILE_PATH> [--output <OUTPUT_DIRECTORY>] [--concurrent <NUM_CONCURRENT_TASKS>] [--chunk-size <CHUNK_SIZE>]
```

#### Options

- `--output <OUTPUT_DIRECTORY>`: Output directory for chunks (default: `<SOURCE_FILE_NAME>_chunks`)
- `--concurrent <NUM_CONCURRENT_TASKS>`: Number of parallel tasks (default: `4`)
- `--chunk-size <CHUNK_SIZE>`: Chunk size in bytes (default: `10MB`)

#### Example

```bash
chunker-cli split large_file.txt --output ./chunks --concurrent 4 --chunk-size 10485760
```

This command splits `large_file.txt` into 10MB chunks using 4 parallel tasks.

### Merge File Chunks

```bash
chunker-cli merge <CHUNKS_DIRECTORY> <OUTPUT_FILE_PATH>
```

#### Example

```bash
chunker-cli merge ./chunks merged_file.txt
```

This command uses parallel processing to rapidly merge chunks back into `merged_file.txt`.

## 🎯 Best Practices

- For optimal performance, set `--concurrent` to match your CPU core count
- Choose a chunk size that balances memory usage with performance (10MB is recommended)
- Ensure adequate free disk space for both chunks and merged files

## 🎉 Contributing

Contributions are welcome! Feel free to fork the repository and submit pull requests.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Split and merge files at lightning speed with `chunker-cli`! ⚡
