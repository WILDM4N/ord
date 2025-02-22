Ordinal Block Explorer
======================

The `ord` binary includes a block explorer. We host a instance of the block
explorer on mainnet at [ordinals.com](https://ordinals.com), and on signet at
[signet.ordinals.com](htps://signet.ordinals.com).

Search
------

The search box accepts a variety of object representations.

### Blocks

Blocks can be searched by hash, for example, the genesis block:

[000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f](https://ordinals.com/search/000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f)

### Transactions

Transactions can be searched by hash, for example, the genesis block coinbase
transaction:

[4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b](https://ordinals.com/search/4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b)

### Outputs

Transaction outputs can searched by outpoint, for example, the only output of
the genesis block coinbase transaction:

[4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:0](https://ordinals.com/search/4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:0)

### Ordinal

Ordinals can be searched by integer, their position within the entire bitcoin
supply:

[2099994106992659](https://ordinals.com/search/2099994106992659)

By decimal, their block and offset within that block:

[481824.0](https://ordinals.com/search/481824.0)

By degree, their cycle, blocks since the last halving, blocks since the last
difficulty adjustment, and offset within their block:

[1°0′0″0‴](https://ordinals.com/search/1°0′0″0‴)

By name, their base 26 representation using the letters "a" through "z":

[ahistorical](https://ordinals.com/search/ahistorical)

Or by percentile, the percentage of bitcoin's supply that has been or will have
been issued when they are mined:

[100%](https://ordinals.com/search/100%)
