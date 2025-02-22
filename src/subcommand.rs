use super::*;

mod epochs;
mod find;
mod index;
mod info;
mod list;
mod parse;
mod range;
mod server;
mod supply;
mod traits;
mod wallet;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  Epochs,
  Find(find::Find),
  Index,
  Info,
  List(list::List),
  Parse(parse::Parse),
  Range(range::Range),
  Server(server::Server),
  Supply,
  Traits(traits::Traits),
  #[clap(subcommand)]
  Wallet(wallet::Wallet),
}

impl Subcommand {
  pub(crate) fn run(self, options: Options) -> Result {
    match self {
      Self::Epochs => epochs::run(),
      Self::Find(find) => find.run(options),
      Self::Index => index::run(options),
      Self::Info => info::run(options),
      Self::List(list) => list.run(options),
      Self::Parse(parse) => parse.run(),
      Self::Range(range) => range.run(),
      Self::Server(server) => {
        let index = Arc::new(Index::open(&options)?);
        let handle = axum_server::Handle::new();
        LISTENERS.lock().unwrap().push(handle.clone());
        server.run(options, index, handle)
      }
      Self::Supply => supply::run(),
      Self::Traits(traits) => traits.run(),
      Self::Wallet(wallet) => wallet.run(options),
    }
  }
}
