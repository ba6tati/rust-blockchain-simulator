use sha2::Sha256;

pub struct Block {
    index: u64,
    timestamp: u64, // stores unix timestamps in seconds
    transactions: Vec<(u64, String)>,
    last_hash: String,
    hash: String,
    nonce: u64
}

impl Block {
    pub fn new(index: u64, last_hash: String) -> Self {
        let mut block = Block {
            index, // TODO: make it automatically
            transactions: Vec::new(),
            last_hash,
            hash: String::new(),
            nonce: 0
        };

        block.hash = block.generate_hash();
        block.timestamp = block.generate_timestamp();
        block
    }

    fn get_index(&self) -> u64 { self.index }
    fn set_index(&mut self, index: u64) { self.index = index; }

    fn get_timestamp(&self) -> u64 { self.timestamp }
    fn set_timestamp(&mut self, timestamp: u64) { self.timestamp = timestamp; }

    fn get_transactions(&self) -> Vec<(u64, String)> { self.transactions.clone() }
    fn add_transaction(&mut self, sender: u64, recipient: String, amount: u64) {
        self.transactions.push((sender, recipient));
    }

    fn get_last_hash(&self) -> String { self.last_hash.clone() }
    fn set_last_hash(&mut self, last_hash: String) { self.last_hash = last_hash; }

    fn get_hash(&self) -> String { self.hash.clone() }
    fn set_hash(&mut self, hash: String) { self.hash = hash; }

    fn get_nonce(&self) -> u64 { self.nonce }
    fn set_nonce(&mut self, nonce: u64) { self.nonce = nonce; }

    fn generate_hash(&self) -> String {
        // TODO: block_data will be generated from format!("{}{}", id, hash, ...)

        let mut hasher = Sha256::new();
        // TODO: hasher.update(block_data.as_bytes());
    } 
}