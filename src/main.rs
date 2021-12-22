// This project is a compressor with the intention of learning the huffman algorithm and understanding
// compression as a whole. It is based other simple compression algorithms that can be used to encode
// files and folders.

use std::collections::HashMap;

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

/// The root node will take the responsibility of holding the sum of the leaf nodes that contains the
/// characters. This node will be made to be easily switched out with another when needed.
///
/// # Note
/// This naming scheme will be subject to change in the future when the implementation is more efficient
/// and more concrete.
struct RootNode {
    pub left: LNode,
    pub right: LNode,
    pub sum: u64,
}

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
struct HuffmanTree {
    pub input: String,
    pub root: Option<Box<RootNode>>,
}

impl HuffmanTree {
    /// Create a new Huffman tree.
    pub fn new(input_file: &str) -> Self {
        Self {
            input: input_file.to_string(),
            root: None,
        }
    }

    /// Gets the frequencies of all characters for the priority queue of the Huffman tree.
    pub fn get_frequencies(&self) -> HashMap<char, usize> {
        let mut frequencies: HashMap<char, usize> = HashMap::new();
        for character in self.input.chars() {
            let entry = frequencies.entry(character).or_insert(0);
            *entry += 1;
        }

        frequencies
    }
}

/// The main entry point for this program.
fn main() {
    // This is the input string that will be used to test the Huffman tree.
    let input = "This is the input string and test for the huffman tree";

    let huffman = HuffmanTree::new(input);
    let freq = huffman.get_frequencies();
    println!("{:#?}", freq);

    println!("{}", 0b010);
}
