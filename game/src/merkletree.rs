use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use ring::digest::Algorithm;

use crate::hashutils::{HashUtils, Hashable};
use crate::tree::{LeavesIntoIterator, LeavesIterator, Tree};

use crate::proof::{Lemma, Proof};

/// A Merkle tree is a binary tree, with values of type `T` at the leafs,
/// and where every internal node holds the hash of the concatenation of the hashes of its children nodes.
#[derive(Clone, Debug)]
pub struct MerkleTree<T> {
    /// The hashing algorithm used by this Merkle tree
    pub algorithm: &'static Algorithm,

    /// The root of the inner binary tree
    root: Tree<T>,

    /// The height of the tree
    height: usize,

    /// The number of leaf nodes in the tree
    count: usize,
}

impl<T: PartialEq> PartialEq for MerkleTree<T> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &MerkleTree<T>) -> bool {
        self.root == other.root
            && self.height == other.height
            && self.count == other.count
            && (self.algorithm as *const Algorithm) == (other.algorithm as *const Algorithm)
    }
}

impl<T: Eq> Eq for MerkleTree<T> {}

impl<T: Ord> PartialOrd for MerkleTree<T> {
    fn partial_cmp(&self, other: &MerkleTree<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for MerkleTree<T> {
    #[allow(trivial_casts)]
    fn cmp(&self, other: &MerkleTree<T>) -> Ordering {
        self.height
            .cmp(&other.height)
            .then(self.count.cmp(&other.count))
            .then((self.algorithm as *const Algorithm).cmp(&(other.algorithm as *const Algorithm)))
            .then_with(|| self.root.cmp(&other.root))
    }
}

impl<T: Hash> Hash for MerkleTree<T> {
    #[allow(trivial_casts)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        <Tree<T> as Hash>::hash(&self.root, state);
        self.height.hash(state);
        self.count.hash(state);
        (self.algorithm as *const Algorithm).hash(state);
    }
}

impl<T> MerkleTree<T> {
    /// Constructs a Merkle Tree from a vector of data blocks.
    /// Returns `None` if `values` is empty.
    pub fn from_vec(algorithm: &'static Algorithm, values: Vec<T>) -> Self
    where
        T: Hashable,
    {
        if values.is_empty() {
            return MerkleTree {
                algorithm,
                root: Tree::empty(algorithm.hash_empty()),
                height: 0,
                count: 0,
            };
        }

        let count = values.len();
        let mut height = 0;
        let mut cur = Vec::with_capacity(count);

        for v in values {
            let leaf = Tree::new_leaf(algorithm, v);
            cur.push(leaf);
        }

        while cur.len() > 1 {
            let mut next = Vec::new();
            while !cur.is_empty() {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                } else {
                    let left = cur.remove(0);
                    let right = cur.remove(0);

                    let combined_hash = algorithm.hash_nodes(left.hash(), right.hash());

                    let node = Tree::Node {
                        hash: combined_hash.as_ref().into(),
                        left: Box::new(left),
                        right: Box::new(right),
                    };

                    next.push(node);
                }
            }

            height += 1;

            cur = next;
        }

        debug_assert!(cur.len() == 1);

        let root = cur.remove(0);

        MerkleTree {
            algorithm,
            root,
            height,
            count,
        }
    }
}

use crate::error::Error;
use byteorder::{BigEndian, ByteOrder};
use near_sdk::borsh::{self, BorshDeserialize, BorshSchema, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use rlp::{self, encode_list};
use safe_transmute::transmute_vec;
use tiny_keccak::{Hasher, Sha3};

#[derive(
    BorshDeserialize,
    BorshSchema,
    BorshSerialize,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Hash,
    Copy,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Hash(pub [u8; 32]);

impl Hash {
/// Create Hash from bytes
pub fn new(data: &[u8]) -> Self {
    Self(Self::hash(data))
}

/// Create Hash for any serializable data
pub fn serialize<S: BorshSerialize + BorshSchema>(d: &S) -> Result<Self, Error> {
    let ser = borsh::try_to_vec_with_schema(d)
    .map_err(|err| Error::HashSerialize(format!("{}", err)))?;
    Ok(Self(Self::hash(&ser[..])))
}

fn hash(input: &[u8]) -> [u8; 32] {
    let mut output = [0; 32];
    let mut sha3 = Sha3::v256();
    sha3.update(input);
    sha3.finalize(&mut output);
    output
}

pub fn sha3_fips_256(input: &[u8]) -> Vec<u32> {
    let mut output = [0; 32];
    let mut sha3 = Sha3::v256();
    sha3.update(input);
    sha3.finalize(&mut output);
    transmute_vec(output.to_vec()).expect("Failed to transform the vector")
}

pub fn ec_recover_public_key(message_hash: &[u8], vote_signature: &[u8]) -> Vec<u8> {
    let r = &vote_signature[0..32];
    let s = &vote_signature[32..64];
    let v = &vote_signature[64..65];
    let prefix: Vec<u8> = transmute_vec([0u8; 31].to_vec()).unwrap();
    let message_hash: Vec<u8> = transmute_vec(message_hash.to_vec()).unwrap();
    let input = encode_list(&[
    BigEndian::read_u128(&message_hash),
    BigEndian::read_u128(&prefix),
    BigEndian::read_u128(v),
    BigEndian::read_u128(r),
    BigEndian::read_u128(s),
    ])
    .to_vec();
    let mut output = [0; 32];
    let mut sha3 = Sha3::v256();
    sha3.update(&input);
    sha3.finalize(&mut output);
    transmute_vec(output.to_vec()).expect("Failed to transform the vector")
    }
}
