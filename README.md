# apolo_contract

near delete nft.w3music2023.testnet w3music2023.testnet
near create-account nft.w3music2023.testnet --masterAccount w3music2023.testnet --initialBalance 20

cargo build --target wasm32-unknown-unknown --release
near deploy --wasmFile target/wasm32-unknown-unknown/release/w3music_contract.wasm --accountId nft.w3music2023.testnet

near call nft.w3music2023.testnet new_default_meta '{"owner_id": "nft.w3music2023.testnet", "vault_id": "w3music2023.testnet"}' --accountId nft.w3music2023.testnet

near call nft.w3music2023.testnet update_tasa '{"tasa": 2.13}' --accountId nft.w3music2023.testnet

near call nft.w3music2023.testnet nft_sample '{"token_metadata":{"title": "prueba musica", "description": "evento de prueba", "media": "https://bafybeiceqkp7hscb72jzqxsdz3o23rf7u6cwr6k3ngkn7asv4fgyp42uei.ipfs.w3s.link/Dion%20Primo%20Enter%20the%20Feast.jpeg", "reference": "https://www.youtube.com/watch?v=jZGpkLElSu8&ab_channel=KarolGVEVO"}, "price":2.13, "royalty":{"nft3.w3music2023.testnet": 1000}, "royalty_buy": {"w3music2023.testnet": 7000}}' --accountId nft.w3music2023.testnet --depositYocto 10910000000000000000000

near call nft.w3music2023.testnet nft_buy '{"token_event_id": "1|1"}' --accountId nft.w3music2023.testnet --deposit 1.2
