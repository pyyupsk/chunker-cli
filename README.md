# Chunker CLI

`Chunker CLI` is a high-performance tool designed to split large files into smaller chunks and merge them back together. It leverages parallel processing for both splitting and merging operations, delivering exceptional performance and resource efficiency.

## 🚀 Features

- **Parallel Split**: Divide large files into smaller chunks using parallel processing
- **Parallel Merge**: Rapidly reassemble chunks back into the original file
- **High Performance**: Efficiently splits and merges large files
- **Resource Efficient**: Uses minimal memory and CPU resources
- **Custom Chunk Size**: Control the size of chunks in bytes
- **Progress Tracking**: Real-time progress bars for both split and merge operations
- **Automatic Cleanup**: Optional cleanup of chunk files after successful merge

## 📦 Installation

### From Releases

1. Go to the [releases](https://github.com/pyyupsk/chunker-cli/releases) page
2. Download the `chunker-cli` executable for your platform
3. Add the executable to your system PATH (optional)

### From Source

```bash
git clone https://github.com/pyyupsk/chunker-cli.git
cd chunker-cli
cargo build --release
```

The compiled binary will be available at `target/release/chunker-cli`

## 🧩 Usage

### Split a File into Chunks

```bash
chunker-cli split <SOURCE_FILE_PATH> [OPTIONS]
```

#### Options

- `-o, --output <OUTPUT_DIRECTORY>`: Output directory for chunks (default: `<SOURCE_FILE_NAME>_chunks`)
- `-c, --concurrent <NUM_CONCURRENT_TASKS>`: Number of parallel tasks (default: `4`)
- `-s, --chunk-size <CHUNK_SIZE>`: Chunk size in bytes (default: `10MB`)
  - Supports human-readable formats: `10MB`, `1GB`, etc.

#### Example

```bash
chunker-cli split large_file.txt -o ./chunks -c 8 -s 20MB
```

This command splits `large_file.txt` into 20MB chunks using 8 parallel tasks.

### Merge File Chunks

```bash
chunker-cli merge <CHUNKS_DIRECTORY> <OUTPUT_FILE_PATH> [OPTIONS]
```

#### Options

- `-c, --concurrent <NUM_CONCURRENT_TASKS>`: Number of parallel tasks (default: `4`)
- `-b, --buffer-size <BUFFER_SIZE>`: Buffer size in bytes (default: `8MB`)
- `--cleanup`: Remove chunk files after successful merge

#### Example

```bash
chunker-cli merge ./chunks merged_file.txt -c 8 --cleanup
```

This command merges chunks from `./chunks` into `merged_file.txt` using 8 parallel tasks and removes the chunks afterward.

## 🎯 Best Practices

- For optimal performance, set `--concurrent` to match your CPU core count
- Choose a chunk size that balances memory usage with performance
  - Recommended: 10MB-100MB for most use cases
  - Larger chunks may improve performance but require more memory
- Ensure adequate free disk space for both chunks and merged files
- Use the `--cleanup` flag when merging to automatically remove chunk files

## 🔍 Error Handling

The tool includes robust error handling for common scenarios:

- Invalid input file paths
- Insufficient disk space
- Missing or corrupted chunks during merge
- Permission issues

## 🎉 Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Split and merge files at lightning speed with `chunker-cli`! ⚡
