# My_Rust_compressor

My_Rust_compressor is a small CLI archiving program made to learn as many functionalities of Rust as possible. It is meant to be fast rather than space-optimized.

Currently, only the Huffman Encoding method is implemented, but this project will grow to have several encoding methods.

## Usage:

To compress files into an archive :
`./my_compressor -c <file_to_compress>+ archive_name.zip`

To decompress archives :
`./my_compressor -d <arhives.zip>+`
*note that decompressing an archive will create one subfolder for each archive*
