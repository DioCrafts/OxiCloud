pub mod rijndael {
    use std::convert::TryInto;
    
    // Define constants
    pub const MODE_CTR: i32 = -1;
    pub const MODE_ECB: i32 = 1;
    pub const MODE_CBC: i32 = 2;
    pub const MODE_CFB: i32 = 3;
    pub const MODE_OFB: i32 = 4;
    
    const MODE_INTERNAL: i32 = 1;
    const MODE_MCRYPT: i32 = 2;
    
    /// Pure-Rust implementation of Rijndael.
    ///
    /// If `set_block_length()` isn't called, it'll be assumed to be 128 bits.  If 
    /// `set_key_length()` isn't called, it'll be calculated from 
    /// `set_key()`.  ie. if the key is 128-bits, the key length will be 128-bits.  If it's 
    /// 136-bits it'll be null-padded to 160-bits and 160 bits will be the key length until 
    /// `set_key()` is called, again, at which point, it'll be recalculated.
    ///
    /// Not all Rijndael implementations may support 160-bits or 224-bits as the block length / key length.
    /// AES, itself, only supports block lengths of 128 and key lengths of 128, 192, and 256.
    pub struct Rijndael {
        // The Encryption Mode
        mode: i32,
        
        // The Key
        key: Vec<u8>,
        
        // The Initialization Vector
        iv: Vec<u8>,
        
        // A "sliding" Initialization Vector
        encrypt_iv: Vec<u8>,
        
        // A "sliding" Initialization Vector
        decrypt_iv: Vec<u8>,
        
        // Continuous Buffer status
        continuous_buffer: bool,
        
        // Padding status
        padding: bool,
        
        // Does the key schedule need to be (re)calculated?
        changed: bool,
        
        // Has the key length explicitly been set or should it be derived from the key, itself?
        explicit_key_length: bool,
        
        // The Key Schedule
        w: Vec<Vec<u32>>,
        
        // The Inverse Key Schedule
        dw: Vec<Vec<u32>>,
        
        // The Block Length
        block_size: usize,
        
        // The Block Length divided by 32
        nb: usize,
        
        // The Key Length
        key_size: usize,
        
        // The Key Length divided by 32
        nk: usize,
        
        // The Number of Rounds
        nr: usize,
        
        // Shift offsets
        c: Vec<usize>,
        
        // Precomputed mixColumns table
        t0: Vec<u32>,
        t1: Vec<u32>,
        t2: Vec<u32>,
        t3: Vec<u32>,
        
        // Precomputed invMixColumns table
        dt0: Vec<u32>,
        dt1: Vec<u32>,
        dt2: Vec<u32>,
        dt3: Vec<u32>,
        
        // Is the mode one that is paddable?
        paddable: bool,
        
        // Encryption buffer for CTR, OFB and CFB modes
        enbuffer: EnBuffer,
        
        // Decryption buffer for CTR, OFB and CFB modes
        debuffer: DeBuffer,
    }
    
    struct EnBuffer {
        encrypted: Vec<u8>,
        xor: Vec<u8>,
        pos: usize,
    }
    
    struct DeBuffer {
        ciphertext: Vec<u8>,
        xor: Vec<u8>,
        pos: usize,
    }
    
    impl Rijndael {
        /// Creates a new Rijndael cipher instance.
        ///
        /// `mode` should only, at present, be `MODE_ECB` or `MODE_CBC`.
        /// If not explicitly set, `MODE_CBC` will be used.
        pub fn new(mode: i32) -> Self {
            let mode = match mode {
                MODE_ECB | MODE_CBC => mode,
                MODE_CTR | MODE_CFB | MODE_OFB => mode,
                _ => MODE_CBC,
            };
            
            let paddable = matches!(mode, MODE_ECB | MODE_CBC);
            
            // Create tables
            let (t0, t1, t2, t3, dt0, dt1, dt2, dt3) = Self::create_tables();
            
            Rijndael {
                mode,
                key: vec![0; 16],
                iv: Vec::new(),
                encrypt_iv: Vec::new(),
                decrypt_iv: Vec::new(),
                continuous_buffer: false,
                padding: true,
                changed: true,
                explicit_key_length: false,
                w: Vec::new(),
                dw: Vec::new(),
                block_size: 16,
                nb: 4,
                key_size: 16,
                nk: 4,
                nr: 0,  // Will be calculated in setup
                c: Vec::new(),
                t0,
                t1,
                t2,
                t3,
                dt0,
                dt1,
                dt2,
                dt3,
                paddable,
                enbuffer: EnBuffer {
                    encrypted: Vec::new(),
                    xor: Vec::new(),
                    pos: 0,
                },
                debuffer: DeBuffer {
                    ciphertext: Vec::new(),
                    xor: Vec::new(),
                    pos: 0,
                },
            }
        }
        
        // Create tables used for encryption/decryption
        fn create_tables() -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>) {
            // The t3 table
            let t3 = vec![
                0x6363A5C6, 0x7C7C84F8, 0x777799EE, 0x7B7B8DF6, 0xF2F20DFF, 0x6B6BBDD6, 0x6F6FB1DE, 0xC5C55491, 
                0x30305060, 0x01010302, 0x6767A9CE, 0x2B2B7D56, 0xFEFE19E7, 0xD7D762B5, 0xABABE64D, 0x76769AEC, 
                0xCACA458F, 0x82829D1F, 0xC9C94089, 0x7D7D87FA, 0xFAFA15EF, 0x5959EBB2, 0x4747C98E, 0xF0F00BFB, 
                0xADADEC41, 0xD4D467B3, 0xA2A2FD5F, 0xAFAFEA45, 0x9C9CBF23, 0xA4A4F753, 0x727296E4, 0xC0C05B9B, 
                0xB7B7C275, 0xFDFD1CE1, 0x9393AE3D, 0x26266A4C, 0x36365A6C, 0x3F3F417E, 0xF7F702F5, 0xCCCC4F83, 
                0x34345C68, 0xA5A5F451, 0xE5E534D1, 0xF1F108F9, 0x717193E2, 0xD8D873AB, 0x31315362, 0x15153F2A, 
                0x04040C08, 0xC7C75295, 0x23236546, 0xC3C35E9D, 0x18182830, 0x9696A137, 0x05050F0A, 0x9A9AB52F, 
                0x0707090E, 0x12123624, 0x80809B1B, 0xE2E23DDF, 0xEBEB26CD, 0x2727694E, 0xB2B2CD7F, 0x75759FEA, 
                0x09091B12, 0x83839E1D, 0x2C2C7458, 0x1A1A2E34, 0x1B1B2D36, 0x6E6EB2DC, 0x5A5AEEB4, 0xA0A0FB5B, 
                0x5252F6A4, 0x3B3B4D76, 0xD6D661B7, 0xB3B3CE7D, 0x29297B52, 0xE3E33EDD, 0x2F2F715E, 0x84849713, 
                0x5353F5A6, 0xD1D168B9, 0x00000000, 0xEDED2CC1, 0x20206040, 0xFCFC1FE3, 0xB1B1C879, 0x5B5BEDB6, 
                0x6A6ABED4, 0xCBCB468D, 0xBEBED967, 0x39394B72, 0x4A4ADE94, 0x4C4CD498, 0x5858E8B0, 0xCFCF4A85, 
                0xD0D06BBB, 0xEFEF2AC5, 0xAAAAE54F, 0xFBFB16ED, 0x4343C586, 0x4D4DD79A, 0x33335566, 0x85859411, 
                0x4545CF8A, 0xF9F910E9, 0x02020604, 0x7F7F81FE, 0x5050F0A0, 0x3C3C4478, 0x9F9FBA25, 0xA8A8E34B, 
                0x5151F3A2, 0xA3A3FE5D, 0x4040C080, 0x8F8F8A05, 0x9292AD3F, 0x9D9DBC21, 0x38384870, 0xF5F504F1, 
                0xBCBCDF63, 0xB6B6C177, 0xDADA75AF, 0x21216342, 0x10103020, 0xFFFF1AE5, 0xF3F30EFD, 0xD2D26DBF, 
                0xCDCD4C81, 0x0C0C1418, 0x13133526, 0xECEC2FC3, 0x5F5FE1BE, 0x9797A235, 0x4444CC88, 0x1717392E, 
                0xC4C45793, 0xA7A7F255, 0x7E7E82FC, 0x3D3D477A, 0x6464ACC8, 0x5D5DE7BA, 0x19192B32, 0x737395E6, 
                0x6060A0C0, 0x81819819, 0x4F4FD19E, 0xDCDC7FA3, 0x22226644, 0x2A2A7E54, 0x9090AB3B, 0x8888830B, 
                0x4646CA8C, 0xEEEE29C7, 0xB8B8D36B, 0x14143C28, 0xDEDE79A7, 0x5E5EE2BC, 0x0B0B1D16, 0xDBDB76AD, 
                0xE0E03BDB, 0x32325664, 0x3A3A4E74, 0x0A0A1E14, 0x4949DB92, 0x06060A0C, 0x24246C48, 0x5C5CE4B8, 
                0xC2C25D9F, 0xD3D36EBD, 0xACACEF43, 0x6262A6C4, 0x9191A839, 0x9595A431, 0xE4E437D3, 0x79798BF2, 
                0xE7E732D5, 0xC8C8438B, 0x3737596E, 0x6D6DB7DA, 0x8D8D8C01, 0xD5D564B1, 0x4E4ED29C, 0xA9A9E049, 
                0x6C6CB4D8, 0x5656FAAC, 0xF4F407F3, 0xEAEA25CF, 0x6565AFCA, 0x7A7A8EF4, 0xAEAEE947, 0x08081810, 
                0xBABAD56F, 0x787888F0, 0x25256F4A, 0x2E2E725C, 0x1C1C2438, 0xA6A6F157, 0xB4B4C773, 0xC6C65197, 
                0xE8E823CB, 0xDDDD7CA1, 0x74749CE8, 0x1F1F213E, 0x4B4BDD96, 0xBDBDDC61, 0x8B8B860D, 0x8A8A850F, 
                0x707090E0, 0x3E3E427C, 0xB5B5C471, 0x6666AACC, 0x4848D890, 0x03030506, 0xF6F601F7, 0x0E0E121C, 
                0x6161A3C2, 0x35355F6A, 0x5757F9AE, 0xB9B9D069, 0x86869117, 0xC1C15899, 0x1D1D273A, 0x9E9EB927, 
                0xE1E138D9, 0xF8F813EB, 0x9898B32B, 0x11113322, 0x6969BBD2, 0xD9D970A9, 0x8E8E8907, 0x9494A733, 
                0x9B9BB62D, 0x1E1E223C, 0x87879215, 0xE9E920C9, 0xCECE4987, 0x5555FFAA, 0x28287850, 0xDFDF7AA5, 
                0x8C8C8F03, 0xA1A1F859, 0x89898009, 0x0D0D171A, 0xBFBFDA65, 0xE6E631D7, 0x4242C684, 0x6868B8D0, 
                0x4141C382, 0x9999B029, 0x2D2D775A, 0x0F0F111E, 0xB0B0CB7B, 0x5454FCA8, 0xBBBBD66D, 0x16163A2C
            ];
            
            // The dt3 table
            let dt3 = vec![
                0xF4A75051, 0x4165537E, 0x17A4C31A, 0x275E963A, 0xAB6BCB3B, 0x9D45F11F, 0xFA58ABAC, 0xE303934B, 
                0x30FA5520, 0x766DF6AD, 0xCC769188, 0x024C25F5, 0xE5D7FC4F, 0x2ACBD7C5, 0x35448026, 0x62A38FB5, 
                0xB15A49DE, 0xBA1B6725, 0xEA0E9845, 0xFEC0E15D, 0x2F7502C3, 0x4CF01281, 0x4697A38D, 0xD3F9C66B, 
                0x8F5FE703, 0x929C9515, 0x6D7AEBBF, 0x5259DA95, 0xBE832DD4, 0x7421D358, 0xE0692949, 0xC9C8448E, 
                0xC2896A75, 0x8E7978F4, 0x583E6B99, 0xB971DD27, 0xE14FB6BE, 0x88AD17F0, 0x20AC66C9, 0xCE3AB47D, 
                0xDF4A1863, 0x1A3182E5, 0x51336097, 0x537F4562, 0x6477E0B1, 0x6BAE84BB, 0x81A01CFE, 0x082B94F9, 
                0x48685870, 0x45FD198F, 0xDE6C8794, 0x7BF8B752, 0x73D323AB, 0x4B02E272, 0x1F8F57E3, 0x55AB2A66, 
                0xEB2807B2, 0xB5C2032F, 0xC57B9A86, 0x3708A5D3, 0x2887F230, 0xBFA5B223, 0x036ABA02, 0x16825CED, 
                0xCF1C2B8A, 0x79B492A7, 0x07F2F0F3, 0x69E2A14E, 0xDAF4CD65, 0x05BED506, 0x34621FD1, 0xA6FE8AC4, 
                0x2E539D34, 0xF355A0A2, 0x8AE13205, 0xF6EB75A4, 0x83EC390B, 0x60EFAA40, 0x719F065E, 0x6E1051BD, 
                0x218AF93E, 0xDD063D96, 0x3E05AEDD, 0xE6BD464D, 0x548DB591, 0xC45D0571, 0x06D46F04, 0x5015FF60, 
                0x98FB2419, 0xBDE997D6, 0x4043CC89, 0xD99E7767, 0xE842BDB0, 0x898B8807, 0x195B38E7, 0xC8EEDB79, 
                0x7C0A47A1, 0x420FE97C, 0x841EC9F8, 0x00000000, 0x80868309, 0x2BED4832, 0x1170AC1E, 0x5A724E6C, 
                0x0EFFFBFD, 0x8538560F, 0xAED51E3D, 0x2D392736, 0x0FD9640A, 0x5CA62168, 0x5B54D19B, 0x362E3A24, 
                0x0A67B10C, 0x57E70F93, 0xEE96D2B4, 0x9B919E1B, 0xC0C54F80, 0xDC20A261, 0x774B695A, 0x121A161C, 
                0x93BA0AE2, 0xA02AE5C0, 0x22E0433C, 0x1B171D12, 0x090D0B0E, 0x8BC7ADF2, 0xB6A8B92D, 0x1EA9C814, 
                0xF1198557, 0x75074CAF, 0x99DDBBEE, 0x7F60FDA3, 0x01269FF7, 0x72F5BC5C, 0x663BC544, 0xFB7E345B, 
                0x4329768B, 0x23C6DCCB, 0xEDFC68B6, 0xE4F163B8, 0x31DCCAD7, 0x63851042, 0x97224013, 0xC6112084, 
                0x4A247D85, 0xBB3DF8D2, 0xF93211AE, 0x29A16DC7, 0x9E2F4B1D, 0xB230F3DC, 0x8652EC0D, 0xC1E3D077, 
                0xB3166C2B, 0x70B999A9, 0x9448FA11, 0xE9642247, 0xFC8CC4A8, 0xF03F1AA0, 0x7D2CD856, 0x3390EF22, 
                0x494EC787, 0x38D1C1D9, 0xCAA2FE8C, 0xD40B3698, 0xF581CFA6, 0x7ADE28A5, 0xB78E26DA, 0xADBFA43F, 
                0x3A9DE42C, 0x78920D50, 0x5FCC9B6A, 0x7E466254, 0x8D13C2F6, 0xD8B8E890, 0x39F75E2E, 0xC3AFF582, 
                0x5D80BE9F, 0xD0937C69, 0xD52DA96F, 0x2512B3CF, 0xAC993BC8, 0x187DA710, 0x9C636EE8, 0x3BBB7BDB, 
                0x267809CD, 0x5918F46E, 0x9AB701EC, 0x4F9AA883, 0x956E65E6, 0xFFE67EAA, 0xBCCF0821, 0x15E8E6EF, 
                0xE79BD9BA, 0x6F36CE4A, 0x9F09D4EA, 0xB07CD629, 0xA4B2AF31, 0x3F23312A, 0xA59430C6, 0xA266C035, 
                0x4EBC3774, 0x82CAA6FC, 0x90D0B0E0, 0xA7D81533, 0x04984AF1, 0xECDAF741, 0xCD500E7F, 0x91F62F17, 
                0x4DD68D76, 0xEFB04D43, 0xAA4D54CC, 0x9604DFE4, 0xD1B5E39E, 0x6A881B4C, 0x2C1FB8C1, 0x65517F46, 
                0x5EEA049D, 0x8C355D01, 0x877473FA, 0x0B412EFB, 0x671D5AB3, 0xDBD25292, 0x105633E9, 0xD647136D, 
                0xD7618C9A, 0xA10C7A37, 0xF8148E59, 0x133C89EB, 0xA927EECE, 0x61C935B7, 0x1CE5EDE1, 0x47B13C7A, 
                0xD2DF599C, 0xF2733F55, 0x14CE7918, 0xC737BF73, 0xF7CDEA53, 0xFDAA5B5F, 0x3D6F14DF, 0x44DB8678, 
                0xAFF381CA, 0x68C43EB9, 0x24342C38, 0xA3405FC2, 0x1DC37216, 0xE2250CBC, 0x3C498B28, 0x0D9541FF, 
                0xA8017139, 0x0CB3DE08, 0xB4E49CD8, 0x56C19064, 0xCB84617B, 0x32B670D5, 0x6C5C7448, 0xB85742D0
            ];
            
            // Precompute tables
            let mut t2 = Vec::with_capacity(256);
            let mut t1 = Vec::with_capacity(256);
            let mut t0 = Vec::with_capacity(256);
            let mut dt2 = Vec::with_capacity(256);
            let mut dt1 = Vec::with_capacity(256);
            let mut dt0 = Vec::with_capacity(256);
            
            for i in 0..256 {
                t2.push(((t3[i] << 8) & 0xFFFFFF00) | ((t3[i] >> 24) & 0x000000FF));
                t1.push(((t3[i] << 16) & 0xFFFF0000) | ((t3[i] >> 16) & 0x0000FFFF));
                t0.push(((t3[i] << 24) & 0xFF000000) | ((t3[i] >> 8) & 0x00FFFFFF));
                
                dt2.push(((dt3[i] << 8) & 0xFFFFFF00) | ((dt3[i] >> 24) & 0x000000FF));
                dt1.push(((dt3[i] << 16) & 0xFFFF0000) | ((dt3[i] >> 16) & 0x0000FFFF));
                dt0.push(((dt3[i] << 24) & 0xFF000000) | ((dt3[i] >> 8) & 0x00FFFFFF));
            }
            
            (t0, t1, t2, t3, dt0, dt1, dt2, dt3)
        }
        
        /// Sets the key.
        ///
        /// Keys can be of any length. Rijndael, itself, requires the use of a key that's between 128-bits and 256-bits long and
        /// whose length is a multiple of 32. If the key is less than 256-bits and the key length isn't set, we round the length
        /// up to the closest valid key length, padding `key` with null bytes. If the key is more than 256-bits, we trim the
        /// excess bits.
        ///
        /// If the key is not explicitly set, it'll be assumed to be all null bytes.
        pub fn set_key(&mut self, key: &[u8]) {
            self.key = key.to_vec();
            self.changed = true;
        }
        
        /// Sets the initialization vector. (optional)
        ///
        /// SetIV is not required when MODE_ECB is being used. If not explicitly set, it'll be assumed
        /// to be all zero's.
        pub fn set_iv(&mut self, iv: &[u8]) {
            let iv_vec = iv.to_vec();
            let padded_iv = if iv_vec.len() >= self.block_size {
                iv_vec[..self.block_size].to_vec()
            } else {
                let mut padded = iv_vec;
                padded.resize(self.block_size, 0);
                padded
            };
            
            self.iv = padded_iv.clone();
            self.encrypt_iv = padded_iv.clone();
            self.decrypt_iv = padded_iv;
        }
        
        /// Sets the key length.
        ///
        /// Valid key lengths are 128, 160, 192, 224, and 256. If the length is less than 128, it will be rounded up to
        /// 128. If the length is greater then 128 and invalid, it will be rounded down to the closest valid amount.
        pub fn set_key_length(&mut self, length: usize) {
            let mut length = length >> 5;
            if length > 8 {
                length = 8;
            } else if length < 4 {
                length = 4;
            }
            self.nk = length;
            self.key_size = length << 2;
            
            self.explicit_key_length = true;
            self.changed = true;
        }
        
        /// Sets the block length.
        ///
        /// Valid block lengths are 128, 160, 192, 224, and 256. If the length is less than 128, it will be rounded up to
        /// 128. If the length is greater then 128 and invalid, it will be rounded down to the closest valid amount.
        pub fn set_block_length(&mut self, length: usize) {
            let mut length = length >> 5;
            if length > 8 {
                length = 8;
            } else if length < 4 {
                length = 4;
            }
            self.nb = length;
            self.block_size = length << 2;
            self.changed = true;
        }
        
        /// Generate CTR XOR encryption key.
        ///
        /// Encrypt the output of this and XOR it against the ciphertext / plaintext to get the
        /// plaintext / ciphertext in CTR mode.
        fn generate_xor(&self, length: usize, iv: &mut Vec<u8>) -> Vec<u8> {
            let mut xor = Vec::new();
            let block_size = self.block_size;
            let num_blocks = (length + (block_size - 1)) / block_size;
            
            for _ in 0..num_blocks {
                xor.extend_from_slice(iv);
                
                let mut j = block_size;
                while j >= 4 {
                    let temp_slice = &iv[iv.len() - j..iv.len() - j + 4];
                    
                    if temp_slice == [0xFF, 0xFF, 0xFF, 0xFF] {
                        let start_idx = iv.len() - j;
                        iv[start_idx..start_idx + 4].copy_from_slice(&[0, 0, 0, 0]);
                    } else if temp_slice == [0x7F, 0xFF, 0xFF, 0xFF] {
                        let start_idx = iv.len() - j;
                        iv[start_idx..start_idx + 4].copy_from_slice