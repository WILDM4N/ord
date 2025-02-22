use super::*;

#[doc(hidden)]
#[jsonrpc_derive::rpc(server)]
pub trait Api {
  #[rpc(name = "getblockchaininfo")]
  fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult, jsonrpc_core::Error>;

  #[rpc(name = "getnetworkinfo")]
  fn get_network_info(&self) -> Result<GetNetworkInfoResult, jsonrpc_core::Error>;

  #[rpc(name = "getbalances")]
  fn get_balances(&self) -> Result<GetBalancesResult, jsonrpc_core::Error>;

  #[rpc(name = "getblockhash")]
  fn get_block_hash(&self, height: usize) -> Result<BlockHash, jsonrpc_core::Error>;

  #[rpc(name = "getblockheader")]
  fn get_block_header(
    &self,
    block_hash: BlockHash,
    verbose: bool,
  ) -> Result<Value, jsonrpc_core::Error>;

  #[rpc(name = "getblock")]
  fn get_block(&self, blockhash: BlockHash, verbosity: u64) -> Result<String, jsonrpc_core::Error>;

  #[rpc(name = "getblockcount")]
  fn get_block_count(&self) -> Result<u64, jsonrpc_core::Error>;

  #[rpc(name = "getwalletinfo")]
  fn get_wallet_info(&self) -> Result<GetWalletInfoResult, jsonrpc_core::Error>;

  #[rpc(name = "createrawtransaction")]
  fn create_raw_transaction(
    &self,
    utxos: Vec<CreateRawTransactionInput>,
    outs: HashMap<String, f64>,
    locktime: Option<i64>,
    replaceable: Option<bool>,
  ) -> Result<String, jsonrpc_core::Error>;

  #[rpc(name = "signrawtransactionwithwallet")]
  fn sign_raw_transaction_with_wallet(
    &self,
    tx: String,
    utxos: Option<()>,
    sighash_type: Option<()>,
  ) -> Result<Value, jsonrpc_core::Error>;

  #[rpc(name = "sendrawtransaction")]
  fn send_raw_transaction(&self, tx: String) -> Result<String, jsonrpc_core::Error>;

  #[rpc(name = "gettransaction")]
  fn get_transaction(
    &self,
    txid: Txid,
    include_watchonly: Option<bool>,
  ) -> Result<Value, jsonrpc_core::Error>;

  #[rpc(name = "getrawtransaction")]
  fn get_raw_transaction(
    &self,
    txid: Txid,
    verbose: bool,
    blockhash: Option<BlockHash>,
  ) -> Result<Value, jsonrpc_core::Error>;

  #[rpc(name = "listunspent")]
  fn list_unspent(
    &self,
    minconf: Option<usize>,
    maxconf: Option<usize>,
    address: Option<bitcoin::Address>,
    include_unsafe: Option<bool>,
    query_options: Option<String>,
  ) -> Result<Vec<ListUnspentResultEntry>, jsonrpc_core::Error>;

  #[rpc(name = "getrawchangeaddress")]
  fn get_raw_change_address(&self) -> Result<bitcoin::Address, jsonrpc_core::Error>;
}
