mod consensus_integration {
    use snarkos_dpc::{
        address::AddressPublicKey,
        dpc::base_dpc::instantiated::{Components, Tx},
    };

    use snarkos_dpc_consensus::{miner::Miner, test_data::*};
    use snarkos_objects::{dpc::DPCTransactions, BlockHeader, BlockHeaderHash, MerkleRootHash};
    use snarkos_storage::genesis::*;
    use snarkos_utilities::bytes::FromBytes;

    fn test_find_block(
        transactions: &DPCTransactions<Tx>,
        parent_header: &BlockHeader,
        expected_previous_block_hash: &BlockHeaderHash,
        expected_merkle_root_hash: &MerkleRootHash,
    ) {
        let consensus = TEST_CONSENSUS;
        let miner_address: AddressPublicKey<Components> = FromBytes::read(&GENESIS_ADDRESS_PAIR[..]).unwrap();
        let miner = Miner::new(miner_address, consensus);

        let header = miner.find_block(transactions, parent_header).unwrap();
        assert_eq!(header.previous_block_hash, *expected_previous_block_hash);
        assert_eq!(header.merkle_root_hash, *expected_merkle_root_hash);
    }

    fn test_verify_header(parent_header: &BlockHeader, child_header: &BlockHeader, merkle_root_hash: &MerkleRootHash) {
        let consensus = TEST_CONSENSUS;
        consensus
            .verify_header(child_header, parent_header, merkle_root_hash)
            .unwrap();
    }

    // CONSENSUS_PARAMS format: [ transaction_bytes, parent_header, child_header, parent_header_hash, merkle_root_hash ]
    const CONSENSUS_PARAMS: [([u8; 1889], BlockHeader, BlockHeader, BlockHeaderHash, MerkleRootHash); 2] = [
        (
            [
                2, 53, 73, 216, 106, 236, 166, 153, 163, 202, 124, 0, 89, 58, 229, 199, 253, 175, 255, 70, 92, 220,
                200, 55, 171, 33, 134, 113, 13, 225, 36, 123, 17, 129, 208, 39, 241, 110, 211, 176, 11, 204, 49, 28,
                158, 60, 90, 255, 35, 252, 31, 218, 15, 41, 72, 32, 121, 68, 252, 79, 209, 133, 142, 217, 2, 164, 215,
                166, 18, 129, 136, 198, 217, 10, 24, 67, 226, 246, 42, 242, 228, 131, 243, 221, 148, 120, 244, 13, 252,
                8, 232, 83, 114, 189, 48, 20, 8, 54, 80, 68, 170, 227, 37, 187, 148, 76, 186, 231, 215, 102, 44, 141,
                81, 242, 3, 131, 117, 243, 152, 94, 159, 10, 95, 63, 84, 204, 37, 147, 0, 2, 211, 87, 215, 140, 108,
                241, 224, 25, 119, 88, 22, 202, 124, 50, 62, 165, 24, 221, 160, 192, 184, 99, 208, 103, 98, 24, 68,
                153, 108, 89, 22, 0, 119, 138, 134, 37, 83, 207, 121, 84, 74, 14, 237, 252, 220, 191, 170, 222, 227,
                64, 249, 122, 63, 171, 21, 96, 38, 135, 5, 99, 254, 24, 10, 6, 153, 191, 132, 12, 230, 67, 44, 115, 71,
                218, 116, 253, 69, 245, 214, 159, 164, 168, 118, 133, 127, 167, 142, 238, 199, 64, 124, 79, 211, 123,
                147, 139, 210, 245, 70, 87, 19, 6, 228, 194, 85, 227, 225, 206, 84, 61, 124, 91, 185, 10, 67, 93, 110,
                143, 196, 132, 149, 171, 244, 201, 12, 97, 190, 1, 17, 212, 43, 132, 111, 175, 103, 232, 72, 146, 19,
                25, 68, 72, 134, 247, 77, 95, 180, 185, 181, 213, 58, 253, 92, 151, 235, 43, 236, 90, 0, 247, 23, 218,
                152, 92, 193, 85, 26, 236, 246, 230, 200, 84, 10, 120, 232, 0, 18, 22, 20, 165, 66, 97, 136, 221, 69,
                153, 61, 13, 36, 29, 223, 33, 77, 28, 118, 166, 36, 214, 21, 68, 131, 119, 191, 172, 144, 140, 68, 218,
                233, 210, 242, 251, 217, 174, 40, 48, 150, 209, 88, 200, 178, 123, 165, 1, 0, 28, 103, 117, 126, 35,
                109, 214, 115, 210, 220, 255, 115, 201, 9, 92, 3, 63, 228, 11, 10, 89, 17, 71, 127, 80, 206, 93, 34,
                110, 238, 174, 29, 203, 157, 78, 65, 111, 50, 59, 61, 9, 254, 218, 185, 205, 180, 227, 0, 88, 71, 42,
                234, 128, 16, 120, 151, 242, 109, 147, 4, 227, 108, 193, 35, 177, 237, 103, 14, 194, 43, 19, 77, 104,
                154, 157, 57, 103, 19, 10, 221, 120, 1, 81, 165, 92, 81, 9, 244, 42, 239, 123, 127, 62, 14, 163, 1,
                247, 4, 32, 6, 59, 129, 220, 194, 163, 151, 227, 19, 212, 199, 116, 33, 120, 113, 157, 164, 136, 1,
                246, 106, 75, 229, 206, 120, 132, 37, 188, 119, 207, 177, 188, 196, 181, 30, 129, 1, 255, 190, 218,
                137, 140, 4, 76, 0, 75, 141, 2, 70, 188, 248, 240, 169, 232, 2, 119, 16, 137, 47, 53, 70, 251, 18, 47,
                178, 54, 223, 47, 152, 199, 133, 220, 124, 219, 204, 244, 67, 240, 37, 121, 111, 188, 219, 193, 147,
                24, 139, 31, 107, 162, 5, 86, 0, 0, 109, 141, 21, 5, 205, 141, 96, 137, 196, 154, 183, 18, 250, 26, 78,
                155, 8, 125, 241, 106, 105, 137, 75, 193, 124, 128, 223, 75, 192, 212, 94, 12, 201, 238, 9, 227, 210,
                181, 230, 165, 151, 36, 46, 144, 237, 204, 116, 0, 255, 214, 204, 71, 243, 197, 172, 137, 134, 56, 246,
                91, 130, 32, 220, 192, 53, 192, 236, 176, 68, 30, 246, 12, 168, 242, 191, 80, 230, 160, 102, 134, 99,
                0, 106, 208, 184, 181, 222, 6, 24, 167, 212, 145, 103, 203, 101, 1, 0, 64, 85, 23, 124, 121, 167, 152,
                57, 95, 68, 4, 219, 155, 148, 97, 32, 67, 125, 133, 205, 217, 132, 29, 69, 166, 63, 219, 92, 121, 248,
                151, 199, 199, 45, 66, 34, 211, 64, 249, 18, 218, 130, 184, 104, 220, 216, 215, 223, 124, 184, 22, 188,
                157, 17, 213, 8, 91, 189, 168, 131, 146, 195, 144, 18, 162, 161, 35, 136, 16, 142, 229, 149, 68, 221,
                172, 146, 198, 233, 228, 200, 190, 31, 209, 70, 241, 47, 169, 246, 7, 203, 10, 99, 144, 182, 214, 142,
                42, 39, 0, 0, 0, 0, 0, 0, 2, 5, 125, 106, 34, 50, 214, 78, 21, 102, 123, 42, 230, 149, 139, 137, 178,
                128, 252, 139, 47, 250, 22, 49, 3, 32, 47, 151, 163, 106, 114, 127, 23, 92, 134, 164, 74, 203, 134, 13,
                77, 63, 7, 129, 161, 201, 35, 162, 64, 194, 95, 94, 235, 61, 108, 2, 238, 192, 164, 67, 72, 120, 100,
                29, 170, 141, 52, 3, 165, 64, 225, 232, 217, 149, 175, 192, 0, 81, 35, 9, 75, 156, 147, 232, 139, 139,
                28, 137, 197, 237, 223, 142, 183, 190, 63, 53, 24, 11, 0, 0, 0, 0, 0, 0, 0, 224, 227, 178, 230, 91,
                175, 247, 102, 221, 242, 131, 247, 81, 201, 68, 100, 77, 112, 67, 13, 44, 165, 227, 93, 120, 222, 83,
                39, 234, 101, 35, 140, 71, 219, 229, 241, 155, 130, 112, 196, 198, 45, 221, 241, 84, 194, 162, 201,
                188, 124, 144, 73, 187, 98, 137, 28, 85, 208, 113, 212, 9, 9, 189, 195, 105, 31, 174, 103, 209, 132,
                252, 19, 193, 217, 135, 23, 206, 209, 37, 98, 82, 228, 182, 0, 205, 86, 94, 243, 217, 148, 73, 165, 29,
                209, 138, 58, 189, 52, 0, 0, 0, 0, 0, 0, 224, 103, 65, 27, 11, 146, 253, 230, 32, 77, 157, 70, 221, 25,
                52, 129, 171, 195, 15, 200, 129, 164, 53, 2, 44, 114, 79, 114, 230, 111, 161, 179, 129, 96, 63, 128,
                184, 214, 239, 146, 251, 131, 214, 154, 21, 241, 225, 254, 238, 94, 97, 145, 149, 42, 45, 245, 137,
                153, 41, 251, 160, 13, 137, 243, 163, 226, 88, 174, 19, 37, 130, 105, 128, 45, 87, 37, 2, 87, 181, 85,
                61, 233, 75, 153, 10, 37, 254, 55, 211, 36, 27, 116, 28, 192, 195, 44, 110, 11, 0, 0, 0, 0, 0, 0, 148,
                201, 112, 22, 48, 140, 79, 103, 254, 108, 199, 105, 183, 191, 146, 86, 252, 199, 180, 0, 219, 162, 210,
                231, 40, 173, 143, 227, 208, 44, 199, 59, 3, 45, 85, 34, 115, 48, 102, 5, 165, 20, 139, 147, 186, 233,
                154, 61, 23, 59, 203, 19, 192, 36, 134, 158, 144, 194, 206, 64, 143, 247, 211, 183, 176, 236, 141, 146,
                103, 152, 188, 7, 7, 15, 136, 92, 20, 153, 176, 251, 188, 228, 243, 230, 232, 70, 98, 44, 71, 254, 30,
                236, 17, 79, 124, 16, 18, 52, 0, 0, 0, 0, 0, 0, 168, 88, 14, 156, 126, 90, 25, 145, 138, 195, 250, 59,
                237, 238, 222, 238, 149, 229, 201, 83, 207, 95, 89, 152, 70, 246, 155, 84, 228, 88, 235, 0, 9, 97, 102,
                123, 128, 148, 175, 52, 194, 60, 72, 132, 144, 109, 214, 162, 107, 149, 236, 73, 236, 221, 145, 137, 9,
                214, 50, 120, 133, 170, 66, 64, 215, 150, 31, 155, 153, 61, 232, 86, 176, 33, 75, 198, 50, 131, 189,
                47, 4, 164, 26, 167, 234, 93, 232, 100, 199, 164, 188, 58, 116, 158, 152, 96, 129, 4, 0, 0, 0, 0, 0, 0,
                100, 23, 197, 146, 228, 154, 237, 234, 33, 202, 95, 114, 87, 18, 231, 143, 10, 170, 197, 92, 85, 253,
                30, 5, 129, 239, 188, 227, 70, 19, 251, 170, 22, 111, 3, 23, 38, 43, 225, 219, 107, 2, 172, 159, 124,
                58, 185, 161, 160, 79, 99, 49, 233, 40, 30, 213, 174, 225, 182, 162, 212, 164, 81, 125, 207, 41, 64,
                14, 53, 52, 82, 29, 91, 226, 54, 170, 18, 190, 109, 110, 18, 251, 166, 101, 119, 88, 159, 128, 247,
                201, 208, 177, 140, 218, 96, 167, 29, 45, 0, 0, 0, 0, 0, 0, 95, 178, 152, 216, 55, 212, 178, 67, 14,
                77, 221, 210, 225, 95, 148, 112, 163, 243, 62, 214, 67, 200, 11, 255, 241, 180, 118, 20, 173, 239, 31,
                203, 52, 84, 3, 33, 135, 175, 200, 72, 68, 224, 40, 221, 167, 123, 31, 122, 123, 245, 85, 237, 174, 93,
                122, 209, 246, 15, 168, 153, 53, 75, 240, 92, 250, 131, 145, 63, 2, 7, 160, 121, 6, 139, 85, 56, 137,
                213, 221, 187, 76, 51, 186, 147, 125, 173, 94, 9, 119, 210, 88, 204, 153, 23, 255, 171, 200, 0, 0, 0,
                0, 0, 0, 0, 0, 72, 174, 245, 187, 39, 93, 138, 40, 109, 133, 36, 195, 48, 100, 216, 5, 51, 175, 161,
                213, 11, 127, 98, 54, 84, 93, 238, 3, 143, 100, 172, 224, 69, 12, 225, 153, 110, 212, 86, 103, 176,
                155, 236, 51, 31, 216, 199, 25, 54, 173, 153, 76, 10, 162, 108, 79, 101, 168, 24, 98, 40, 79, 227, 67,
                107, 104, 184, 108, 148, 226, 185, 8, 132, 204, 31, 237, 132, 255, 169, 129, 233, 127, 113, 220, 254,
                204, 239, 182, 58, 11, 203, 102, 115, 87, 0, 201, 35, 29, 0, 0, 0, 0, 0, 0, 58, 224, 123, 253, 199, 17,
                100, 154, 11, 24, 216, 54, 40, 157, 50, 92, 241, 156, 21, 192, 247, 43, 114, 33, 46, 230, 188, 48, 70,
                164, 15, 22, 100, 82, 125, 189, 164, 60, 112, 89, 254, 221, 211, 69, 214, 218, 159, 231, 130, 140, 82,
                233, 7, 62, 87, 185, 202, 103, 15, 76, 107, 8, 255, 89, 207, 202, 74, 212, 250, 125, 110, 116, 113,
                239, 106, 7, 229, 79, 242, 94, 5, 251, 89, 194, 208, 68, 113, 28, 147, 163, 238, 54, 139, 234, 120,
                194, 87, 2, 0, 0, 0, 0, 0, 0, 0, 194, 97, 245, 172, 16, 185, 133, 62, 154, 74, 130, 237, 100, 34, 60,
                124, 249, 226, 68, 63, 9, 68, 33, 169, 77, 18, 141, 178, 82, 228, 43, 20, 64, 130, 197, 174, 81, 175,
                99, 121, 21, 111, 128, 186, 153, 67, 160, 175, 86, 42, 230, 46, 162, 141, 92, 0, 241, 107, 125, 194,
                106, 206, 252, 1, 0, 31, 10, 250, 255, 255, 255, 255, 2, 3, 210, 62, 111, 44, 17, 223, 162, 178, 86,
                81, 164, 192, 76, 250, 38, 37, 68, 37, 28, 83, 125, 203, 206, 243, 208, 54, 203, 149, 195, 170, 1, 152,
                109, 174, 186, 81, 196, 169, 141, 185, 169, 120, 144, 100, 9, 144, 51, 88, 111, 166, 92, 112, 10, 42,
                11, 29, 200, 84, 101, 138, 213, 41, 3, 164, 255, 170, 24, 15, 215, 181, 216, 76, 80, 192, 217, 180,
                211, 28, 16, 139, 145, 52, 95, 149, 155, 150, 84, 79, 23, 248, 121, 45, 56, 194, 2, 224, 104, 52, 80,
                27, 70, 89, 118, 83, 108, 93, 176, 112, 185, 63, 64, 199, 248, 90, 196, 18, 36, 130, 18, 114, 97, 217,
                96, 216, 209, 175, 3,
            ],
            BlockHeader {
                previous_block_hash: BlockHeaderHash([0u8; 32]),
                merkle_root_hash: MerkleRootHash([0u8; 32]),
                time: 0i64,
                difficulty_target: 0x07FF_FFFF_FFFF_FFFF_u64,
                nonce: 0u32,
            },
            BlockHeader {
                previous_block_hash: BlockHeaderHash([
                    34, 41, 138, 133, 212, 97, 218, 105, 103, 149, 244, 65, 29, 63, 202, 157, 79, 184, 117, 83, 54,
                    165, 78, 178, 91, 245, 248, 4, 235, 112, 78, 121,
                ]),
                merkle_root_hash: MerkleRootHash([
                    57, 234, 200, 188, 96, 77, 30, 155, 49, 208, 100, 5, 50, 64, 76, 66, 82, 237, 186, 233, 186, 237,
                    244, 88, 119, 149, 21, 59, 227, 100, 99, 138,
                ]),
                time: 0i64,
                difficulty_target: 17_762_688_379_536_359_781_u64,
                nonce: 0u32,
            },
            BlockHeaderHash([
                34, 41, 138, 133, 212, 97, 218, 105, 103, 149, 244, 65, 29, 63, 202, 157, 79, 184, 117, 83, 54, 165,
                78, 178, 91, 245, 248, 4, 235, 112, 78, 121,
            ]),
            MerkleRootHash([
                57, 234, 200, 188, 96, 77, 30, 155, 49, 208, 100, 5, 50, 64, 76, 66, 82, 237, 186, 233, 186, 237, 244,
                88, 119, 149, 21, 59, 227, 100, 99, 138,
            ]),
        ),
        (
            [
                2, 53, 73, 216, 106, 236, 166, 153, 163, 202, 124, 0, 89, 58, 229, 199, 253, 175, 255, 70, 92, 220,
                200, 55, 171, 33, 134, 113, 13, 225, 36, 123, 17, 129, 208, 39, 241, 110, 211, 176, 11, 204, 49, 28,
                158, 60, 90, 255, 35, 252, 31, 218, 15, 41, 72, 32, 121, 68, 252, 79, 209, 133, 142, 217, 2, 164, 215,
                166, 18, 129, 136, 198, 217, 10, 24, 67, 226, 246, 42, 242, 228, 131, 243, 221, 148, 120, 244, 13, 252,
                8, 232, 83, 114, 189, 48, 20, 8, 54, 80, 68, 170, 227, 37, 187, 148, 76, 186, 231, 215, 102, 44, 141,
                81, 242, 3, 131, 117, 243, 152, 94, 159, 10, 95, 63, 84, 204, 37, 147, 0, 2, 211, 87, 215, 140, 108,
                241, 224, 25, 119, 88, 22, 202, 124, 50, 62, 165, 24, 221, 160, 192, 184, 99, 208, 103, 98, 24, 68,
                153, 108, 89, 22, 0, 119, 138, 134, 37, 83, 207, 121, 84, 74, 14, 237, 252, 220, 191, 170, 222, 227,
                64, 249, 122, 63, 171, 21, 96, 38, 135, 5, 99, 254, 24, 10, 6, 153, 191, 132, 12, 230, 67, 44, 115, 71,
                218, 116, 253, 69, 245, 214, 159, 164, 168, 118, 133, 127, 167, 142, 238, 199, 64, 124, 79, 211, 123,
                147, 139, 210, 245, 70, 87, 19, 6, 228, 194, 85, 227, 225, 206, 84, 61, 124, 91, 185, 10, 67, 93, 110,
                143, 196, 132, 149, 171, 244, 201, 12, 97, 190, 1, 17, 212, 43, 132, 111, 175, 103, 232, 72, 146, 19,
                25, 68, 72, 134, 247, 77, 95, 180, 185, 181, 213, 58, 253, 92, 151, 235, 43, 236, 90, 0, 247, 23, 218,
                152, 92, 193, 85, 26, 236, 246, 230, 200, 84, 10, 120, 232, 0, 18, 22, 20, 165, 66, 97, 136, 221, 69,
                153, 61, 13, 36, 29, 223, 33, 77, 28, 118, 166, 36, 214, 21, 68, 131, 119, 191, 172, 144, 140, 68, 218,
                233, 210, 242, 251, 217, 174, 40, 48, 150, 209, 88, 200, 178, 123, 165, 1, 0, 28, 103, 117, 126, 35,
                109, 214, 115, 210, 220, 255, 115, 201, 9, 92, 3, 63, 228, 11, 10, 89, 17, 71, 127, 80, 206, 93, 34,
                110, 238, 174, 29, 203, 157, 78, 65, 111, 50, 59, 61, 9, 254, 218, 185, 205, 180, 227, 0, 88, 71, 42,
                234, 128, 16, 120, 151, 242, 109, 147, 4, 227, 108, 193, 35, 177, 237, 103, 14, 194, 43, 19, 77, 104,
                154, 157, 57, 103, 19, 10, 221, 120, 1, 81, 165, 92, 81, 9, 244, 42, 239, 123, 127, 62, 14, 163, 1,
                247, 4, 32, 6, 59, 129, 220, 194, 163, 151, 227, 19, 212, 199, 116, 33, 120, 113, 157, 164, 136, 1,
                246, 106, 75, 229, 206, 120, 132, 37, 188, 119, 207, 177, 188, 196, 181, 30, 129, 1, 255, 190, 218,
                137, 140, 4, 76, 0, 75, 141, 2, 70, 188, 248, 240, 169, 232, 2, 119, 16, 137, 47, 53, 70, 251, 18, 47,
                178, 54, 223, 47, 152, 199, 133, 220, 124, 219, 204, 244, 67, 240, 37, 121, 111, 188, 219, 193, 147,
                24, 139, 31, 107, 162, 5, 86, 0, 0, 109, 141, 21, 5, 205, 141, 96, 137, 196, 154, 183, 18, 250, 26, 78,
                155, 8, 125, 241, 106, 105, 137, 75, 193, 124, 128, 223, 75, 192, 212, 94, 12, 201, 238, 9, 227, 210,
                181, 230, 165, 151, 36, 46, 144, 237, 204, 116, 0, 255, 214, 204, 71, 243, 197, 172, 137, 134, 56, 246,
                91, 130, 32, 220, 192, 53, 192, 236, 176, 68, 30, 246, 12, 168, 242, 191, 80, 230, 160, 102, 134, 99,
                0, 106, 208, 184, 181, 222, 6, 24, 167, 212, 145, 103, 203, 101, 1, 0, 64, 85, 23, 124, 121, 167, 152,
                57, 95, 68, 4, 219, 155, 148, 97, 32, 67, 125, 133, 205, 217, 132, 29, 69, 166, 63, 219, 92, 121, 248,
                151, 199, 199, 45, 66, 34, 211, 64, 249, 18, 218, 130, 184, 104, 220, 216, 215, 223, 124, 184, 22, 188,
                157, 17, 213, 8, 91, 189, 168, 131, 146, 195, 144, 18, 162, 161, 35, 136, 16, 142, 229, 149, 68, 221,
                172, 146, 198, 233, 228, 200, 190, 31, 209, 70, 241, 47, 169, 246, 7, 203, 10, 99, 144, 182, 214, 142,
                42, 39, 0, 0, 0, 0, 0, 0, 2, 5, 125, 106, 34, 50, 214, 78, 21, 102, 123, 42, 230, 149, 139, 137, 178,
                128, 252, 139, 47, 250, 22, 49, 3, 32, 47, 151, 163, 106, 114, 127, 23, 92, 134, 164, 74, 203, 134, 13,
                77, 63, 7, 129, 161, 201, 35, 162, 64, 194, 95, 94, 235, 61, 108, 2, 238, 192, 164, 67, 72, 120, 100,
                29, 170, 141, 52, 3, 165, 64, 225, 232, 217, 149, 175, 192, 0, 81, 35, 9, 75, 156, 147, 232, 139, 139,
                28, 137, 197, 237, 223, 142, 183, 190, 63, 53, 24, 11, 0, 0, 0, 0, 0, 0, 0, 224, 227, 178, 230, 91,
                175, 247, 102, 221, 242, 131, 247, 81, 201, 68, 100, 77, 112, 67, 13, 44, 165, 227, 93, 120, 222, 83,
                39, 234, 101, 35, 140, 71, 219, 229, 241, 155, 130, 112, 196, 198, 45, 221, 241, 84, 194, 162, 201,
                188, 124, 144, 73, 187, 98, 137, 28, 85, 208, 113, 212, 9, 9, 189, 195, 105, 31, 174, 103, 209, 132,
                252, 19, 193, 217, 135, 23, 206, 209, 37, 98, 82, 228, 182, 0, 205, 86, 94, 243, 217, 148, 73, 165, 29,
                209, 138, 58, 189, 52, 0, 0, 0, 0, 0, 0, 224, 103, 65, 27, 11, 146, 253, 230, 32, 77, 157, 70, 221, 25,
                52, 129, 171, 195, 15, 200, 129, 164, 53, 2, 44, 114, 79, 114, 230, 111, 161, 179, 129, 96, 63, 128,
                184, 214, 239, 146, 251, 131, 214, 154, 21, 241, 225, 254, 238, 94, 97, 145, 149, 42, 45, 245, 137,
                153, 41, 251, 160, 13, 137, 243, 163, 226, 88, 174, 19, 37, 130, 105, 128, 45, 87, 37, 2, 87, 181, 85,
                61, 233, 75, 153, 10, 37, 254, 55, 211, 36, 27, 116, 28, 192, 195, 44, 110, 11, 0, 0, 0, 0, 0, 0, 148,
                201, 112, 22, 48, 140, 79, 103, 254, 108, 199, 105, 183, 191, 146, 86, 252, 199, 180, 0, 219, 162, 210,
                231, 40, 173, 143, 227, 208, 44, 199, 59, 3, 45, 85, 34, 115, 48, 102, 5, 165, 20, 139, 147, 186, 233,
                154, 61, 23, 59, 203, 19, 192, 36, 134, 158, 144, 194, 206, 64, 143, 247, 211, 183, 176, 236, 141, 146,
                103, 152, 188, 7, 7, 15, 136, 92, 20, 153, 176, 251, 188, 228, 243, 230, 232, 70, 98, 44, 71, 254, 30,
                236, 17, 79, 124, 16, 18, 52, 0, 0, 0, 0, 0, 0, 168, 88, 14, 156, 126, 90, 25, 145, 138, 195, 250, 59,
                237, 238, 222, 238, 149, 229, 201, 83, 207, 95, 89, 152, 70, 246, 155, 84, 228, 88, 235, 0, 9, 97, 102,
                123, 128, 148, 175, 52, 194, 60, 72, 132, 144, 109, 214, 162, 107, 149, 236, 73, 236, 221, 145, 137, 9,
                214, 50, 120, 133, 170, 66, 64, 215, 150, 31, 155, 153, 61, 232, 86, 176, 33, 75, 198, 50, 131, 189,
                47, 4, 164, 26, 167, 234, 93, 232, 100, 199, 164, 188, 58, 116, 158, 152, 96, 129, 4, 0, 0, 0, 0, 0, 0,
                100, 23, 197, 146, 228, 154, 237, 234, 33, 202, 95, 114, 87, 18, 231, 143, 10, 170, 197, 92, 85, 253,
                30, 5, 129, 239, 188, 227, 70, 19, 251, 170, 22, 111, 3, 23, 38, 43, 225, 219, 107, 2, 172, 159, 124,
                58, 185, 161, 160, 79, 99, 49, 233, 40, 30, 213, 174, 225, 182, 162, 212, 164, 81, 125, 207, 41, 64,
                14, 53, 52, 82, 29, 91, 226, 54, 170, 18, 190, 109, 110, 18, 251, 166, 101, 119, 88, 159, 128, 247,
                201, 208, 177, 140, 218, 96, 167, 29, 45, 0, 0, 0, 0, 0, 0, 95, 178, 152, 216, 55, 212, 178, 67, 14,
                77, 221, 210, 225, 95, 148, 112, 163, 243, 62, 214, 67, 200, 11, 255, 241, 180, 118, 20, 173, 239, 31,
                203, 52, 84, 3, 33, 135, 175, 200, 72, 68, 224, 40, 221, 167, 123, 31, 122, 123, 245, 85, 237, 174, 93,
                122, 209, 246, 15, 168, 153, 53, 75, 240, 92, 250, 131, 145, 63, 2, 7, 160, 121, 6, 139, 85, 56, 137,
                213, 221, 187, 76, 51, 186, 147, 125, 173, 94, 9, 119, 210, 88, 204, 153, 23, 255, 171, 200, 0, 0, 0,
                0, 0, 0, 0, 0, 72, 174, 245, 187, 39, 93, 138, 40, 109, 133, 36, 195, 48, 100, 216, 5, 51, 175, 161,
                213, 11, 127, 98, 54, 84, 93, 238, 3, 143, 100, 172, 224, 69, 12, 225, 153, 110, 212, 86, 103, 176,
                155, 236, 51, 31, 216, 199, 25, 54, 173, 153, 76, 10, 162, 108, 79, 101, 168, 24, 98, 40, 79, 227, 67,
                107, 104, 184, 108, 148, 226, 185, 8, 132, 204, 31, 237, 132, 255, 169, 129, 233, 127, 113, 220, 254,
                204, 239, 182, 58, 11, 203, 102, 115, 87, 0, 201, 35, 29, 0, 0, 0, 0, 0, 0, 58, 224, 123, 253, 199, 17,
                100, 154, 11, 24, 216, 54, 40, 157, 50, 92, 241, 156, 21, 192, 247, 43, 114, 33, 46, 230, 188, 48, 70,
                164, 15, 22, 100, 82, 125, 189, 164, 60, 112, 89, 254, 221, 211, 69, 214, 218, 159, 231, 130, 140, 82,
                233, 7, 62, 87, 185, 202, 103, 15, 76, 107, 8, 255, 89, 207, 202, 74, 212, 250, 125, 110, 116, 113,
                239, 106, 7, 229, 79, 242, 94, 5, 251, 89, 194, 208, 68, 113, 28, 147, 163, 238, 54, 139, 234, 120,
                194, 87, 2, 0, 0, 0, 0, 0, 0, 0, 194, 97, 245, 172, 16, 185, 133, 62, 154, 74, 130, 237, 100, 34, 60,
                124, 249, 226, 68, 63, 9, 68, 33, 169, 77, 18, 141, 178, 82, 228, 43, 20, 64, 130, 197, 174, 81, 175,
                99, 121, 21, 111, 128, 186, 153, 67, 160, 175, 86, 42, 230, 46, 162, 141, 92, 0, 241, 107, 125, 194,
                106, 206, 252, 1, 0, 31, 10, 250, 255, 255, 255, 255, 2, 3, 210, 62, 111, 44, 17, 223, 162, 178, 86,
                81, 164, 192, 76, 250, 38, 37, 68, 37, 28, 83, 125, 203, 206, 243, 208, 54, 203, 149, 195, 170, 1, 152,
                109, 174, 186, 81, 196, 169, 141, 185, 169, 120, 144, 100, 9, 144, 51, 88, 111, 166, 92, 112, 10, 42,
                11, 29, 200, 84, 101, 138, 213, 41, 3, 164, 255, 170, 24, 15, 215, 181, 216, 76, 80, 192, 217, 180,
                211, 28, 16, 139, 145, 52, 95, 149, 155, 150, 84, 79, 23, 248, 121, 45, 56, 194, 2, 224, 104, 52, 80,
                27, 70, 89, 118, 83, 108, 93, 176, 112, 185, 63, 64, 199, 248, 90, 196, 18, 36, 130, 18, 114, 97, 217,
                96, 216, 209, 175, 3,
            ],
            BlockHeader {
                previous_block_hash: BlockHeaderHash([0u8; 32]),
                merkle_root_hash: MerkleRootHash([0u8; 32]),
                time: 0i64,
                difficulty_target: 0x0000_7FFF_FFFF_FFFF_u64,
                nonce: 69950u32,
            },
            BlockHeader {
                previous_block_hash: BlockHeaderHash([
                    71, 96, 136, 138, 156, 60, 0, 0, 172, 219, 151, 28, 109, 226, 132, 171, 235, 109, 113, 92, 207, 54,
                    69, 213, 19, 158, 217, 13, 154, 191, 146, 241,
                ]),
                merkle_root_hash: MerkleRootHash([
                    57, 234, 200, 188, 96, 77, 30, 155, 49, 208, 100, 5, 50, 64, 76, 66, 82, 237, 186, 233, 186, 237,
                    244, 88, 119, 149, 21, 59, 227, 100, 99, 138,
                ]),
                time: 0i64,
                difficulty_target: 17_730_728_272_220_445_192_u64,
                nonce: 55793u32,
            },
            BlockHeaderHash([
                71, 96, 136, 138, 156, 60, 0, 0, 172, 219, 151, 28, 109, 226, 132, 171, 235, 109, 113, 92, 207, 54, 69,
                213, 19, 158, 217, 13, 154, 191, 146, 241,
            ]),
            MerkleRootHash([
                57, 234, 200, 188, 96, 77, 30, 155, 49, 208, 100, 5, 50, 64, 76, 66, 82, 237, 186, 233, 186, 237, 244,
                88, 119, 149, 21, 59, 227, 100, 99, 138,
            ]),
        ),
    ];

    #[test]
    fn find_valid_block() {
        CONSENSUS_PARAMS.iter().for_each(
            |(transaction_bytes, parent_header, _, expected_previous_block_hash, expected_merkle_root_hash)| {
                let transaction = Tx::read(&transaction_bytes[..]).unwrap();
                let transactions = DPCTransactions(vec![transaction]);

                test_find_block(
                    &transactions,
                    parent_header,
                    expected_previous_block_hash,
                    expected_merkle_root_hash,
                );
            },
        );
    }

    #[test]
    fn verify_header() {
        CONSENSUS_PARAMS
            .iter()
            .for_each(|(_, parent_header, child_header, _, expected_merkle_root_hash)| {
                test_verify_header(parent_header, child_header, expected_merkle_root_hash);
            });
    }
}
