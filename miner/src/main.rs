use blockchainlib::*;

fn main() {
    println!("This is miner node!!");

    // TODO: Listen to the web to see if there're any transactions that need to be processed
    // let transactions_from_network = get_transactions_from_network();
    let transactions_from_network: Vec<Transaction> = vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 30,
                }
            ]
        }
    ];

    // TODO: Implement get_block_from_network() to get the blockchain from the network
    // let block = get_block_from_network();
    let pre_block_index = 1; 
    let pre_block_hash = vec![0; 32];
    let generated_difficulty = 0x00ffffffffffffffffffffffffffffff;


    // Prepare block to be mined
    let mut block = Block::new(pre_block_index, now(), pre_block_hash, transactions_from_network, generated_difficulty);

    block.mine();
    println!("block mined block {:?}", &block);

    // TODO: Broadcast the mined block to the network to be validated
    // broadcast_network(&block);
}
