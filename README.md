# C.L.O.B
Callam Limit Order Book

A basic LOB implementation in Rust. The orderbook is represented by an array of 32 items. It is assumed 
that 32 items will be enough to represent at least a few levels of depth; and there is an inbuilt rebalance method 
(similar to a hashmap rebalance). 

Designed mostly with crypto in mind.


# Potential Performance improvements
 - move from Decimal to an Integer representation of currency. This would be easy if given fiat restrictions,
 but more challenging taking into account crypto currency. The book operates on an assumption of min step & min price which allows
 very quick refernece to a particular price / quantity pair. Integer would be more efficient than Decimal for instantiation and most calculations.
 - Investigate  BLAS/LAPACK - Considering the orderbook is relatively light on math this may be more relevant to deducing something
 from the book rather than actual book operations.
 - less assignments / pre-assignment. Rust offers some nice primitives for pre-asignment (?). We may wish to investigate these, though
 my heavy preferance is to keep things on the stack as much as possible.

# TODO
 - impelement order types; this should be a relatively easy task considering that everything is really a subset of a limit order
 - implement more benches. We need to know the performance of rebalances, orders etc.


# A final note

This is not really a production ready orderbook. I intend to use this code as a basis for a real client-side orderbook implementation and will update this repository with any code / improvement which does not affect my edge, which is somewhat speed related.

Conscious thought has been applied to CPU cache levels and accessibility of data. I am not sure as to the effectiveness of the cache hit rate,
further investigation is required on this point.