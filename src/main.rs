// This project is a compressor with the intention of learning the huffman algorithm and understanding
// compression as a whole. It is based other simple compression algorithms that can be used to encode
// files and folders.

/// This is a short hand name since the usage of `Option<Box<Node>>` is used quite often.
type LNode = Option<Box<Node>>;

/// The huffman algorithm is comprised of a tree of nodes, each of which is represented as a leaf.
/// Each node contains a left and a right child node, respectively. It also contains the character it
/// represents and the frequency of which that character appears in the input file.
///
/// The node additionally contains a code represented in bits. The bits corresponds to the frequency of the
/// character and how much it appears in the input file. The more times the character appears in the
/// input file, the smaller the code of that character is.
struct Node {
    pub left: LNode,
    pub right: LNode,
    pub frequency: u64,
    pub value: u8,
}

impl Node {}

/// The huffman tree is going to be responsible not only for the encoding and decoding of characters
/// but also count as the root of the tree. The root node will change as the input file is read and
/// the root will count as the total bits of the input file. Each child node is the sum of all children's
/// bits, with the last child being the character itself.
///
/// # Example
/// ```rust
/// let mut huffman = Huffman::new();
/// let encoded_string = huffman.encode(&input_file);
/// println!("Encoded");
/// println!("String: {}", encoded_string);
///
/// // the huffman tree has to have already encodes the string before you can get the codes.
/// let codes = huffman.get_codes();
/// for code in codes {
///     println!("{}: {}", code.0, code.1);
/// }
///
/// // Additionally, you can't decode a string without the huffman tree already having encoded the string
/// // the reason behind it is that the huffman tree contains the key codes for each character.
/// let original_input = huffman.decode();
/// println!("Decoded: {}", original_input);
///
/// ```
struct HuffmanTree {}

/// The main entry point for this program.
fn main() {
    // TODO: Take in commands from the user and run them accordingly.
    // Implement a form of a binary tree if one doesn't exist.
}
