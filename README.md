# Readme

This project contains a snippet of the network manager component as described in the associated design document.

The snippet itself is implemented in Rust and can be found in `src/network_manager.rs`. It expands on operation of
the outgoing buffer as described in the design document, focusing on response chunking and connection interruption
handling.

This component was chosen to further illustrate how the design might handle some of the complexity arising from
connection interruptions and bandwidth limits, as well as to frame how it might be adaptable to different systems.
