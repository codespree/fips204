use fips204::traits::{KeyGen, Signer, Verifier};

#[cfg(feature = "ml-dsa-44")]
use fips204:: ml_dsa_44;

#[cfg(feature = "ml-dsa-44")]
use fips204::traits::SerDes;

#[cfg(all(feature = "ml-dsa-44", feature = "default-rng"))]
use rand_core::RngCore;

#[cfg(feature = "ml-dsa-65")]
use fips204:: ml_dsa_65;

#[cfg(feature = "ml-dsa-87")]
use fips204:: ml_dsa_87;

use rand_chacha::rand_core::SeedableRng;

// cargo flamegraph --test integration

// $ cargo test --package fips204 --test integration forever -- --ignored --nocapture
#[cfg(all(feature = "ml-dsa-44", feature = "default-rng"))]
#[ignore]
#[test]
fn forever() {
    let mut msg = [0u8; 32];
    let mut i = 0u64;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123);
    let mut flip = [0u8; 5];
    loop {
        rng.fill_bytes(&mut msg);
        let (pk, sk) = ml_dsa_44::KG::try_keygen_with_rng(&mut rng).unwrap();
        let sig = sk.try_sign(&msg).unwrap();
        let ver = pk.try_verify(&msg, &sig);
        assert!(ver.unwrap());
        // now, confirm that we do not accept an invalid signature
        rng.fill_bytes(&mut flip);
        let index = u32::from_le_bytes(flip[0..4].try_into().unwrap()); // index of byte to flip
        let mut sig2 = core::array::from_fn(|i| sig[i]);
        sig2[index as usize % (sig2.len() - 2)] ^= if flip[4] != 0 { flip[4] } else { 0x55 }; // investigate sig[last]
        let ver = pk.try_verify(&msg, &sig2);
        if ver.is_ok() && ver.unwrap() {
            eprintln!("Msg is      {}\n", hex::encode(msg));
            eprintln!("Index is {}\nflip[4] is{}\n", index, flip[4]);
            eprintln!("sk is       {}\n", hex::encode(sk.into_bytes()));
            eprintln!("pk is       {}\n", hex::encode(pk.into_bytes()));
            eprintln!("good sig is {}\n", hex::encode(sig));
            eprintln!("bad sig is  {}\n", hex::encode(sig2));
            assert!(!ver.unwrap()); // fail and stop if 'verified'
        }
        if i % 10000 == 0 {
            println!("So far i: {}", i)
        };
        i += 1;
    }
}

// This test demonstrates that two different signatures can be verified for the same pk/sig
// Spec error, see https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/TQo-qFbBO1A/m/YcYKjMblAAAJ
// Ref https://github.com/pq-crystals/dilithium/blob/master/ref/packing.c
#[cfg(feature = "ml-dsa-44")]
#[ignore]
#[test]
fn bad_sig() {
    let msg: [u8; 32] = <[u8; 32]>::try_from(
        hex::decode("2e77de619a8963d1ec5ff9e7db269e3ed9076a35badf49960571c8a98055d8eb").unwrap(),
    )
    .unwrap();
    let _sk = ml_dsa_44::PrivateKey::try_from_bytes(<[u8; 2560]>::try_from(hex::decode("6ef73b20d211607012d7697a8277141e905239801f7bd539cbb8ea8e3d9146a515bbd2e28754acdf3ca5a9a17feaa3c0b336a365aa3e02f02c438845823b6be5d63f8b5e3deec3a297b3906f3b5f094da7bb165acaa37ab4dbdb2a1f63198910ee9212a21feaa334a02b3875a03fd1b78ed68a03595303d0c3e8b9a7808512af0c037193884812396153401289b410102548db1065d2c66959b8301411910c2522c3b44908250940c285a1c004d2360522c100cc12299848412105881a49700449302202104c822c23a96820a185dc006a0bc68c21354cd8801020416c8316660b310ae2b66c121024d2c6718b246401c60c8b266401895110996d22a00d042920202069518404ca2682d9128c1b026084c85158b051831086123328484622ca841184264e21c6600a070d58306419016d21380c1b266d99b4855c0651d0187142286e24a4842182459aa80c50b685a310615ba001c096008ac071c0a04050a200124246513030d4384ac9286110265023a240502682c0900902114602a92889b210db324190342813c78dd2204d19a52521b97188902123362212178d6414690033481ac481d294405a300dc914650cb129c1422d43c4615396299c302109c26c09426021968920b490219485da3681c3b80943142149086543848d93088c00954413076a40304ada806580343223968161c644e0285120058892048522158858068a40241042b86508166c89c884a01430ca9051d04289449688e1b061181000503282484648e1280e8a12491a3845a23880e238529c048dd3362112060114a71110094519a72c0128681ba92d218625a41809c4b62493105158126150c86d9306425cb685db982819306e23b10da42204d1b800da4866ca124489460ec8144963462d912864990686093810134000128865144628042429dac844a320284c9691e204224290114aa664c0482d0129500aa96c60a2044a124251c20402276d43b211000680e44012cb268a03c30851042249326991308e61064a0c346621900450b84119430609362e01344c1a386d0b898c11a2600208091b3128193811db1032d1840d4b4286d4b271dc066c00326008412888981109b180ca206824016510324a21936589b26d82a8440c893144b610128321a31285d2422a89321018442804a82959842941c201094782d0a02562226ccb18245208121a130008a040a10602e4a66514134094008a8c360ec9a089610640210241d3028013316dd206029b51781ac3a5960cb8833902e9f8b9e3ee16565ea0f7ff99bf78e1615a6c13e255c493ff1e0f4470ffb3726cab9209301832c2f499c158cd5bf40ddcd3c9ce7c659e660ee05740b169199ff1c866f7019ab53379481a8d3fe35356600e650de1dbfd9a42a1362af0b45d0d483fe2f427f303ba6296d05dd779babfb8d00d3acb24a2de06382cfad42b58fc1ff641d37cd2a43a3eace737e3e84732ee0d1109482d2cc4b22f38ca7f32ee16828ad732523edb39f0446ac568cbf166e9b31147dbd5db72f0b8c0e4d2a81606e9857c81bde135c431bcf489c8342d8c586493c52cb98a845f765dee285a1c48fb610312ebdb731464cca484e733b3cc333c208d9538416ea9dafe8d997016e3a724bfd6b00680e541613f5d3efc23b7e94a7ce71b1918ee78cb0d856c804b107ecc55541b2696958de0d06e030a6002eaff42b0e8ab7b64fd6592e42c582480ec539e286d3298e429f33e40fc8924ed70140a931f60afa9f81084578ccda3f6edf568f48cc8d6ef01d8875f9530793fd8bd099be129c14abdd7d5930a16aa317a4d40d2c22b6ff4fbb4e6d261206d4333c321911df41892109d412c5aed2299e4688113000eaab6a86e07c5979a251f44dbd2d45dc313adcc632d5f59da19dba7c4bcd17b40f7f96bfc49c6959155400d64fa53c2268c1a6b1ee5f630e9146e75e2ce1045af964127b2d29ba232cbffb7067f10b18880ce635e3d0e4c6f27430a34875a206623a08761d48519c53d9d68464f4687322020fa06b5d63d87e97a556271b1af1d7e7ae2db32d80bcf5fd09d9c283e3d28e29602dacb99ae56db9f7bc4b3b7247328d4b70497470c58e8b6f9154c2e87b2d92800bd17ea2b71c5239d55aa15d0a6e6efb67890ed272ebf69edcba8574fd788f57fe62141ea584219ac3cc926121f7fcbe46589f7e3f76c02f499b2c271ec3af1e43470243c52cdaac42e123900a832b175dda043295ff13af0239da5c724d30442a17d89ef07f8eaf9c16e6506f63eaf0995a9e103635e6052126ecb2774bc745619396e0eecb9cc00a4b35b094316653fa86995724350b90440e077c1523acdf980c2b29d4e54cbca6948c065e36eb08517be32a0f591e0d4887aba99097d75a97b2e14686b9e725752b75a7c312af0a3dff4222dee6ce015b56799829dcbc43b1e12ec97b354515558b22066afd8e1a6316f75598cc9f347eb120fc47cec2d750c411eddfd0df24fa1fe5e05e65eacab1536455540c5353a6fccb4ae6f86e3ca967d181942e8213684a8aac9d528de027fd984e7ded3a84458b4270abc847be85f9a3fd736f3c3cdb3b0f18cf0e793f331227989ff0ed0415e22feeee0b9800b29af84ff06e920b6e7fcb758f8733fe444c5a6f9b55c9e8b5eb0185da1bd08c25dfdd4e427a29eacfbc3a5da364759fd8f9c71afeff42e5dbe1432aefcffb727d55b67d7f61d9b8f52db5e9caf946d594d7cf2a1a55422a9e5700074219b0c3045ef572e60318ef3e7ae8dd7b821d07d7df54a563458c6dec05dfff6fcd462ae26caca3323bdc1929bcb4723c8a78d24a5b135167b4602b97902968bd5aa58fb798d8b92f31bbdf3ac3fab6f87926989acc9443fe5c7ea4f8f54bdad5f138eb0b75826837836e33915d95347bb69090cb774a26ef15cda81ca98cfe0cec911d85e70745877e563f17f43322b0b8995840491f197a58a3dc76f95695ff10d48286bc2755337eb1dad68b7371c52b8f491bc6e751a6347f8fbe429c65e33668467ae0dac3f4dd28ebeb5c5aecb1eb478e7a70a1a14ef1190d2843c0b992f5d8b0134d34016e153167e731e85280a36338b8df0477d46b747000d2b168be6dbcf2f138063cc15014357173c6e1674bae16be57ea7043aa6bae0c67d8caf801d7ce0055bb8c4bd52c5010be6dd6c28287c1f88391faeb19917af085f724c48c23b120d70bc9ae4ce292ef97f099cfae69eef756ca50e0dbf8486ac04c400b3188756c9d7ed3422fd3170dce3f66d6b90858159ae64f8b30b73fad255d0214f550b337306f02b16e96c18d1426e31bbc46180893908b0d3773c0699eb60fcf236b480fb665c573e0abd59e113e3ba295a94cf95879aa8373af97ae9a90298c218b95807a8e799b194b68aed731146bab58fa5494b42ef9acb7181e82bc3225b1f00dd79ee40897402cbeac266954a0dfa24406c686a922e65defc7e0f131e58c892df216c6b96a088199984bb4410f9ef1eef8c1cc2c05501072c2a674ff8fb7bffdfd0ccaeb66f77d897e3c5b1adfe5f79ec5e94c12ef097da5c4393d165e84a9b69f53661670bde2663d7b99d3713fbfef511710141798837dab0d153fc").unwrap()).unwrap()).unwrap();
    let pk = ml_dsa_44::PublicKey::try_from_bytes(<[u8; 1312]>::try_from(hex::decode("6ef73b20d211607012d7697a8277141e905239801f7bd539cbb8ea8e3d9146a5035af8ef22935f07f254a03558b8aa3b7baff5d1d0c63890beebfa28b13c67954696229413467a676b4611a7d9f0edefe78b7d70db79dbfb31df0dda9d81906af3295a285ed27288e00badf22724be26a3a6e3f00bb1f8671777b5eeca11fb6432969d38a8e8022548ee39b26bafc2071f26a1f0f602c8451b37e1c170494fba123b6f5a789e71f005fc0b1c4209b99cc9780b6b4ce4cfcf26ab3c8cb3991b0e516ae6ece41c1ad81432cb50112e59bc66e67d3f6e88dd80f91a319a5237064ec463ea5023780a480760a1c110816b1348e15ac2e8bc10ce0a2d8b47da53d8b3ab8ffa4fde0fb861d05a582492998f0d60d45a95885d1d4e5b3bfca1c7d9fba9b11f99d7f3087ae8891c6a1f5cfd3a02d4fe8e1c510ef023574c0dfab9f86a7e8491b405c659ba84eaf6b2b5d1eb4db575d0d25bd6e92f43a80dad5725b8847dfea0ee18e2222000d738210accfc75d3718f78eaeb9f2964688276c8269fd8803c5854eda05efed36cf4e57dbf4b54bd18db022e4494812644353818eb2f5509e14f8a3c682dea883deec5bf0ac75b7c893ab14c01c3336987b718b92dc6ec34a8278dc5a0981e8162fb822cfb86ce283cf22145f701623d1aaa36c653463b373a07fcecadfdf24287bc2213b43108550c505160b14f68ec36bd94bfcc372e82c3bb5044126b76541e0776e874de03c4eade5cc42bed862f7b30ced6da5cdb09d761a848bce989cd7219929fac382266d49268a542a6484b8d29b5423ff28d18d5d052a751cef7d4f550b68b01bdb510aea11cfd314489f522d8f6273f3612e9f49b459819c1ff19c9f38f17cc0a8934612a52df3ce09d807caa2414c449efe8c9a90697fcc48607d344bc1f4140c4a159a8e1ecb13b9396e74f379f7f4da69a2d20cde7c3f360f4c2afe140a4ffed050dd0ca6851b4c6e8c959e8ace271afd3ee880367a1c862a47f5cfc95f07e970766c00cc45b7d6060b48110fe8dec28793d1a3cb784d16ea82ab39ea01a7cc86028c98589586a11d3b0ab468cb160ddc7a70cd9b8214b9fe54c6d7d69dbc8e1efeae24a8985fae533737d2156aa5c3d78f50209d2039566d98aa1ee442a738a53fd004f4e464734793d4a24d6bb3a76f0dac5cff75417c5f89d9abaad89994e420add0d6c0f7ec7910b46bf758e71e1fbe598a1dfd746ba15f6ef8e5135b6f3cf03f9c36981ed6b6cbba00f881dc1f929101ca774af1c89d2fd96e713609740b74592758e70b23c8f7ee774564d9654974c783fadb0589858f1f70a9294e84fd19db1cd43d7034831c6eee7f6fdc2a0d4ff97ae2c6e97c44906f9ad18a393ecf346fdeb0c4aeb1e705d3b699bedb5ae435f28ae4ef1e35220bf90c58afeb1aa26fa215b35ad0ee32ff8422f8943feb2887a9f094acf13df4bb2666e7f5463f9ed7129279e31d73ae000208aded4fd233eb837ee0b70dae358965651df6ddb9181b1b820eb86e78cfca93d682d47e78eeb09782dae582b359a78e7746c3f81a15ab8a325b848c64336790baafe0fb6045312e1168b62b58dcc89f54972426d3ef542040de0dcfd67534356cef46ab464e580d89c73ceebf3bd79229d7a669eb96dc76808924693a948d60dc6c377aaaa7f5769aa051777082cd0821e95c5647e53eb889830bb6c15dc7374da7726e14d14e82985122295c6dfc1ba7266b0d108561f58eb183ef8f7c7c247c11a229ac8f7ed989c78855b8bd949de7f94762e1f692f373880115b8faa9558dca7abbd36ff1a64f35853a79f447d9381a7b93b4c43fb8b4152d884a7d319508bccc3a21b2d").unwrap()).unwrap()).unwrap();
    let good_sig =<[u8; 2420]>::try_from(hex::decode("bcb502b1c16c3bf5c40450fd32ec0d15f5d31e454716984b76cef27ff5bd3e4d11f80cf857d677a984bcb70f29915840ecce898914a65d33a425417308afc2fb247428a5ed843d0603469973172102ecf997fdfeaeab39b41ee3be5afd4157e1dc34ce9b782aef95edacdfcecab116315a925bdb0c17fec880052804efcad89a0cea15e76e9ad8de73002bb6e6d6bb182dbb16c6b4714e07f5c130656cd253d5711c446871bf02a3a28f90a3b1f26e8d6116afce733986563fc6c0c1f4ff4c8a86f5f49d7a949b38ba2351fd38ac6a33996c6ea818ba0830dd004f90ceeeb1f4bcadd3a28d9baa8c3ea7c3b12c478553ff9d323bc2e480528941714ca59f08da5e870ea30bda4ffa9ac194bdba21cb9e08502d400b810d12534b8a4476ae3a48a0e8cde465f6bd0bbc25b9668e6753464aef17d5cc84f9107856f379ec42cf762224090d91f2d0b26721b56182a2150d24b766f5735d37cc594965c9a02462231891ea1f37849313dfc3af65d2b24b510caa689e912be1bc49011aea6276459d52a4b2a7ad00cf27f73e830ae02c051ab868533fb0f08b189123b60acd5bfd4719dc15a4d07a118b25f53cc20b10d26af2ade3e4593e47ce4729f20a1762f5baeebe8de435f237c248bd9530ade80627c6414bcd80edb35bb23efafd82c10ddc4733e76a1e5a5173ef4ee1988060994c50b3aceff0740a319d7b9e1c9241e6a49cf25109015de1cdaa7a5f89dd7be7174a153e906648ec3e567d104d352d34cebed293cef06ef551b46a1f1516ffa54827829ecfeba79853ea850ba67c74e694f1d2f7195a7568b61f576997cf3e409f51a67020f446981dc8bace88424739938a40c21165e0a2b94f51ffb39bb82bfb62533f62d0ceec50aef1ad079478c3c45ed6ee7d608e1d0f29b605da0c55cfea0ba40d4e8499872fa445da3cee7ae8b8ec62ec20ff158b70243a350d59de0c2e69254fe7a1ca825686c0c81a9c3ea1129d9f97395c9fd91ba5c996a510bf3a87dd2ed57cbc717aeb3e218dc1f28558709d9a88d4f6dea89ab69a61eb84515a9ef1577c2c57673bdd462d26ab30b306ac4101e3e05af0915b0cbbf4743ba0107149fc8576738e42005eb7978f2d753e2adb4839a8335fb48a8bdeef60bbb5cb44f54b8624d2365d5b92d159c7227116a87100bb1a2e3394124149a32c859c0a9f30371f80d048c02f154d14bf1b37b1db32c701c51397faa880483c90b01aa61d50b5f148c3326d569e2fcde732f8d6bd1f439add3fff31ab158aad4bd13bcecce77ff7bd46c277766a8404582b55cf9e67088ba7040e459844e821e60bc9db11c65083baf60d7bcea45e9e121b52f4f72ec142ec691ba61ade30255d1379c61182da55416e2d8e60f5cb1e9801bf7b1805a6542213da0311fa7aa078d94c86909f5358209ecb9d07129cedad0031012df076fcb03120a6276563711fea05964ddbbf09a1badb0a93b1ec60be62159b669c0623d9242c3d03bf29047de26e92d3ba300ca2fab5859e59e37542454141589ebf8e823fd8a08470951bc2db82de8b72fa839a483b7f7326cdea03246588ee10993a6962f8e07f4765b063d89d48d49ad5de307df0de6b5b8574d40a190adf709776aac04dd3c6f531f96d07c2186eb9d0e7418f6777a42323e144202392a664f938601bfa0aa028d7f5835e6811d7efea967bbcd9db824365eda0988ea1002d28939d0e3f66ebb41daa20ab33b728771ab422a8beb8571828947601fe833feea78c393b36c75880b1f0d4f9efeb268ed03cdb0e81e8139f578495e6ea6e55dbe0b8c661f1c013b14540bff775f6b095c03dffacb8534161aa8e10362d6eb3f5622ab5ef1203f03e9a141f5f54fe8e8dbdb403fcf399558baf8154a9d74e2f2033eed030b38980066b45f71854565f239df1bfbc773e991d033862b59f2ac5ccb932f0178a08669f56c07593bd11c4612a6ef4aeaa4f7532cbdb6604c659764d5a574602ba5c344c470eb29303b89955df5bcce7f7ab7b7ba8da332f30d5444111731e988d597bfb1928f4eda9ad1ca3c9c56d70bbaa4e47be0a386b4de060ba4538ec7db8a617a35252c6b121cf9e1910ea6385943263b1b18224746e421687648fb598f9a9e9aba59bea2a208d6b98c33d8268c5d2370d2eb354a66ba7ff6c35633fcd5614f71c68b56859e37983316fd99794684365d0ce167f891c1393c3f016c3293f16879d00c65ed86ad7684c825d87a853767149c8c7aead44f63e5a425134f6f6823a1f7825143d51b89bc8e077b174e5d1ae41815ad3f80c0dd27d4c6358ed4441eb89b8027b0b2cd3f700c1934b4410ef714ee8c54aaae4562d14f4bbe9f3b60bf07d547e25a8145b302e3e32731a68a106d4289cb1298fae0e6709ffb855d9673bd41e3459bc5c2df3b218d44cce81759a5a5cee8ba755a1dcdfd3e264c38bdf475ef6221b6a684987aace346e3b4d70ba4c5591682032fe0b20df05f54ea3c83289860cc73be8c8296fc1addd0195b9be04f3f531f035429eb38b8f58d9ad1f715a78702224e711d3b36d863721e5799d79e56785d0220e77cb3fa21062b68eeccd6abb9cea936fc517b153789d7a1fecbaaaf7ed116f032558aa813716574f550edfe3687424f67596a04916a591bfbb05b24d778de7d880ad711ca3162128f65d71ba8609bb931c19dfd38ea341fd25aca1f0169816258361e8b49b053fa203b8fb5c86c1d5bb0190d59040cf145b0ddcd27c5361a28850bd2c8181b4e72063832a38e200f98fdd54bb77ad685daffc31d27d8fc2fc374d437510f077cc278e7305b10f076e8a01dca35d3d1d54966a74c49a4c6ea168d6ebb8f07c3b8b6122edc4a1b642e6fb7b6e4b529ec743d63da129a889347e7c4b1109c4419bf13fb1cab50b6229015466175cb57b577b399bca25a0b784f99f90317082949b01e518a6d4348dc00ed76d57253ee6959b01c6fd89d1f006d9cb1e08b70fcb5ddc4b5b531989e3a2d1544b6041eaba1375fbcf8b756020b10ebeeb24e349f75ccfdd2c2eec5183e12a9a4daa677ca7b138185e3fa5a54f30df049ca4065ca928d07bc337f6383852291e12273800a3e9e492b5219cc25e021d91b5be483c8c1066c254ca3a28aef1de15dac4adcf425bebae41ea47788e8aa9d461b35a5d157073b68a155a609077ed9cabf1a1683a26f789afa7c104f321676225c438901aa0dd7719d43d89faf4b92785d0d63ebb4a1f91c66868dc5ccea9c9fffa80c5c111d77000f33865b17f12ce08c64f35708756f28b1025ca0f0ea2b29228393f4144546264717e8589c2c3d7ddedf4040b0d6a829ea5d2ddfd16376f70a0b0b1b3b8bccccdcee1fb0031438ca3a9cb000000000000000000000000000000000000000000000000000000000000121c2b32").unwrap()).unwrap();
    let bad_sig = <[u8; 2420]>::try_from(hex::decode("bcb502b1c16c3bf5c40450fd32ec0d15f5d31e454716984b76cef27ff5bd3e4d11f80cf857d677a984bcb70f29915840ecce898914a65d33a425417308afc2fb247428a5ed843d0603469973172102ecf997fdfeaeab39b41ee3be5afd4157e1dc34ce9b782aef95edacdfcecab116315a925bdb0c17fec880052804efcad89a0cea15e76e9ad8de73002bb6e6d6bb182dbb16c6b4714e07f5c130656cd253d5711c446871bf02a3a28f90a3b1f26e8d6116afce733986563fc6c0c1f4ff4c8a86f5f49d7a949b38ba2351fd38ac6a33996c6ea818ba0830dd004f90ceeeb1f4bcadd3a28d9baa8c3ea7c3b12c478553ff9d323bc2e480528941714ca59f08da5e870ea30bda4ffa9ac194bdba21cb9e08502d400b810d12534b8a4476ae3a48a0e8cde465f6bd0bbc25b9668e6753464aef17d5cc84f9107856f379ec42cf762224090d91f2d0b26721b56182a2150d24b766f5735d37cc594965c9a02462231891ea1f37849313dfc3af65d2b24b510caa689e912be1bc49011aea6276459d52a4b2a7ad00cf27f73e830ae02c051ab868533fb0f08b189123b60acd5bfd4719dc15a4d07a118b25f53cc20b10d26af2ade3e4593e47ce4729f20a1762f5baeebe8de435f237c248bd9530ade80627c6414bcd80edb35bb23efafd82c10ddc4733e76a1e5a5173ef4ee1988060994c50b3aceff0740a319d7b9e1c9241e6a49cf25109015de1cdaa7a5f89dd7be7174a153e906648ec3e567d104d352d34cebed293cef06ef551b46a1f1516ffa54827829ecfeba79853ea850ba67c74e694f1d2f7195a7568b61f576997cf3e409f51a67020f446981dc8bace88424739938a40c21165e0a2b94f51ffb39bb82bfb62533f62d0ceec50aef1ad079478c3c45ed6ee7d608e1d0f29b605da0c55cfea0ba40d4e8499872fa445da3cee7ae8b8ec62ec20ff158b70243a350d59de0c2e69254fe7a1ca825686c0c81a9c3ea1129d9f97395c9fd91ba5c996a510bf3a87dd2ed57cbc717aeb3e218dc1f28558709d9a88d4f6dea89ab69a61eb84515a9ef1577c2c57673bdd462d26ab30b306ac4101e3e05af0915b0cbbf4743ba0107149fc8576738e42005eb7978f2d753e2adb4839a8335fb48a8bdeef60bbb5cb44f54b8624d2365d5b92d159c7227116a87100bb1a2e3394124149a32c859c0a9f30371f80d048c02f154d14bf1b37b1db32c701c51397faa880483c90b01aa61d50b5f148c3326d569e2fcde732f8d6bd1f439add3fff31ab158aad4bd13bcecce77ff7bd46c277766a8404582b55cf9e67088ba7040e459844e821e60bc9db11c65083baf60d7bcea45e9e121b52f4f72ec142ec691ba61ade30255d1379c61182da55416e2d8e60f5cb1e9801bf7b1805a6542213da0311fa7aa078d94c86909f5358209ecb9d07129cedad0031012df076fcb03120a6276563711fea05964ddbbf09a1badb0a93b1ec60be62159b669c0623d9242c3d03bf29047de26e92d3ba300ca2fab5859e59e37542454141589ebf8e823fd8a08470951bc2db82de8b72fa839a483b7f7326cdea03246588ee10993a6962f8e07f4765b063d89d48d49ad5de307df0de6b5b8574d40a190adf709776aac04dd3c6f531f96d07c2186eb9d0e7418f6777a42323e144202392a664f938601bfa0aa028d7f5835e6811d7efea967bbcd9db824365eda0988ea1002d28939d0e3f66ebb41daa20ab33b728771ab422a8beb8571828947601fe833feea78c393b36c75880b1f0d4f9efeb268ed03cdb0e81e8139f578495e6ea6e55dbe0b8c661f1c013b14540bff775f6b095c03dffacb8534161aa8e10362d6eb3f5622ab5ef1203f03e9a141f5f54fe8e8dbdb403fcf399558baf8154a9d74e2f2033eed030b38980066b45f71854565f239df1bfbc773e991d033862b59f2ac5ccb932f0178a08669f56c07593bd11c4612a6ef4aeaa4f7532cbdb6604c659764d5a574602ba5c344c470eb29303b89955df5bcce7f7ab7b7ba8da332f30d5444111731e988d597bfb1928f4eda9ad1ca3c9c56d70bbaa4e47be0a386b4de060ba4538ec7db8a617a35252c6b121cf9e1910ea6385943263b1b18224746e421687648fb598f9a9e9aba59bea2a208d6b98c33d8268c5d2370d2eb354a66ba7ff6c35633fcd5614f71c68b56859e37983316fd99794684365d0ce167f891c1393c3f016c3293f16879d00c65ed86ad7684c825d87a853767149c8c7aead44f63e5a425134f6f6823a1f7825143d51b89bc8e077b174e5d1ae41815ad3f80c0dd27d4c6358ed4441eb89b8027b0b2cd3f700c1934b4410ef714ee8c54aaae4562d14f4bbe9f3b60bf07d547e25a8145b302e3e32731a68a106d4289cb1298fae0e6709ffb855d9673bd41e3459bc5c2df3b218d44cce81759a5a5cee8ba755a1dcdfd3e264c38bdf475ef6221b6a684987aace346e3b4d70ba4c5591682032fe0b20df05f54ea3c83289860cc73be8c8296fc1addd0195b9be04f3f531f035429eb38b8f58d9ad1f715a78702224e711d3b36d863721e5799d79e56785d0220e77cb3fa21062b68eeccd6abb9cea936fc517b153789d7a1fecbaaaf7ed116f032558aa813716574f550edfe3687424f67596a04916a591bfbb05b24d778de7d880ad711ca3162128f65d71ba8609bb931c19dfd38ea341fd25aca1f0169816258361e8b49b053fa203b8fb5c86c1d5bb0190d59040cf145b0ddcd27c5361a28850bd2c8181b4e72063832a38e200f98fdd54bb77ad685daffc31d27d8fc2fc374d437510f077cc278e7305b10f076e8a01dca35d3d1d54966a74c49a4c6ea168d6ebb8f07c3b8b6122edc4a1b642e6fb7b6e4b529ec743d63da129a889347e7c4b1109c4419bf13fb1cab50b6229015466175cb57b577b399bca25a0b784f99f90317082949b01e518a6d4348dc00ed76d57253ee6959b01c6fd89d1f006d9cb1e08b70fcb5ddc4b5b531989e3a2d1544b6041eaba1375fbcf8b756020b10ebeeb24e349f75ccfdd2c2eec5183e12a9a4daa677ca7b138185e3fa5a54f30df049ca4065ca928d07bc337f6383852291e12273800a3e9e492b5219cc25e021d91b5be483c8c1066c254ca3a28aef1de15dac4adcf425bebae41ea47788e8aa9d461b35a5d157073b68a155a609077ed9cabf1a1683a26f789afa7c104f321676225c438901aa0dd7719d43d89faf4b92785d0d63ebb4a1f91c66868dc5ccea9c9fffa80c5c111d77000f33865b17f12ce08c64f35708756f28b1025ca0f0ea2b29228393f4144546264717e8589c2c3d7ddedf4040b0d6a829ea5d2ddfd16376f70a0b0b1b3b8bccccdcee1fb0031438ca3a9cb000000000000000000000000000000000000000000000000000000000000121c2b48").unwrap()).unwrap();
    assert!(pk.try_verify(&msg, &good_sig).unwrap());
    assert!(pk.try_verify(&msg, &bad_sig).unwrap());
}

#[cfg(all(feature = "ml-dsa-44", feature = "default-rng"))]
#[test]
fn test_44_rounds() {
    let mut msg = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123);
    for i in 0..128 {
        msg[0] = i as u8;
        let (pk, sk) = ml_dsa_44::KG::try_keygen_with_rng(&mut rng).unwrap();
        let sig = sk.try_sign(&msg).unwrap();
        let ver = pk.try_verify(&msg, &sig);
        assert!(ver.unwrap())
    }
}

#[cfg(feature = "ml-dsa-65")]
#[test]
fn test_65_rounds() {
    let mut msg = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(456);
    for i in 0..128 {
        msg[0] = i as u8;
        let (pk, sk) = ml_dsa_65::KG::try_keygen_with_rng(&mut rng).unwrap();
        let sig = sk.try_sign_with_rng(&mut rng, &msg).unwrap();
        let ver = pk.try_verify(&msg, &sig);
        assert!(ver.unwrap())
    }
}

#[cfg(feature = "ml-dsa-87")]
#[test]
fn test_87_rounds() {
    let mut msg = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(789);
    for i in 0..128 {
        msg[0] = i as u8;
        let (pk, sk) = ml_dsa_87::KG::try_keygen_with_rng(&mut rng).unwrap();
        let sig = sk.try_sign_with_rng(&mut rng, &msg).unwrap();
        let ver = pk.try_verify(&msg, &sig);
        assert!(ver.unwrap())
    }
}

#[cfg(feature = "ml-dsa-44")]
#[test]
fn test_44_no_verif() {
    let msg = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123);
    let (pk, sk) = ml_dsa_44::KG::try_keygen_with_rng(&mut rng).unwrap();
    let sig = sk.try_sign_with_rng(&mut rng, &msg).unwrap();

    // Bad messages
    for i in 0..8 {
        let mut msg_bad = msg;
        msg_bad[i] ^= 0x08;
        let ver = pk.try_verify(&msg_bad, &sig).unwrap();
        assert!(!ver)
    }

    // Bad secret key  (intriguing, byte 40 is 'k' which is allowed variance)
    for i in 0..8 {
        let mut sk_bad = sk.clone().into_bytes();
        sk_bad[70 + i * 10] ^= 0x08;
        let sk_bad = ml_dsa_44::PrivateKey::try_from_bytes(sk_bad).unwrap();
        let sig = sk_bad.try_sign_with_rng(&mut rng, &msg).unwrap();
        let ver = pk.try_verify(&msg, &sig).unwrap();
        assert!(!ver)
    }

    // Bad public key
    for i in 0..8 {
        let mut pk_bad = pk.clone().into_bytes();
        pk_bad[i * 10] ^= 0x08;
        let pk_bad = ml_dsa_44::PublicKey::try_from_bytes(pk_bad).unwrap();
        let ver = pk_bad.try_verify(&msg, &sig).unwrap();
        assert!(!ver)
    }

    // Bad signature
    for i in 0..8 {
        let mut sig_bad = sig;
        sig_bad[i * 10] ^= 0x08;
        let ver = pk.try_verify(&msg, &sig_bad).unwrap();
        assert!(!ver)
    }
}
