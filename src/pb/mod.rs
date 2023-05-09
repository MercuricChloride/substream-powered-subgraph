// @generated
pub mod google {
    // @@protoc_insertion_point(attribute:google.protobuf)
    pub mod protobuf {
        include!("google.protobuf.rs");
        // @@protoc_insertion_point(google.protobuf)
    }
}
pub mod sf {
    pub mod ethereum {
        pub mod block_meta {
            // @@protoc_insertion_point(attribute:sf.ethereum.block_meta.v1)
            pub mod v1 {
                include!("sf.ethereum.block_meta.v1.rs");
                // @@protoc_insertion_point(sf.ethereum.block_meta.v1)
            }
        }
    }
    pub mod substreams {
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}
// @@protoc_insertion_point(attribute:test)
pub mod test {
    include!("test.rs");
    // @@protoc_insertion_point(test)
}
