use blockchainlib::*;

fn main () {
    let difficulty = 0x00ffffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 8,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 50,
                }
             ]
        }
    ], difficulty);

    genesis_block.mine();
    println!("Genesis block mined block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Fail to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 120,
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
