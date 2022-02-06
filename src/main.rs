use blockchainlib::*;

fn main () {
    let difficulty = 0x00ffffffffffffffffffffffffffffff;

    // Use web API to construct a transactions, and then broadcast to the network?
    // Then, the miner collects the transactions from the network and mine them.
    // Peer-to-peer
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 10,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 10,
                }
            ]
        }
    ], difficulty);
    
    // Miner did this
    genesis_block.mine();
    println!("Genesis block mined block {:?}", &genesis_block);

    let last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();
    
    // Validator did this
    blockchain.update_with_block(genesis_block).expect("Fail to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 20,
                }
            ]
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 3,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 4,
                }
            ]
        },

    ], difficulty);

    block.mine();
    println!("{:?}", &block);

    blockchain.update_with_block(block).expect("Mined second block!");
}
