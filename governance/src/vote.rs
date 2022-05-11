elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub enum VoteType {
    Upvote = 1,
    DownVote = 2,
}