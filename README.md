# anchor-upgradeable-loader
Example of reading your own upgradeable loader state in anchor

To run yourself:

1. Change the wallet keypair in Anchor.toml so it refers to yours, not mine :p
2. anchor build --> generate a program id keypair for yourself
3. fix declare_id! + Anchor.toml
4. *rerun* anchor build
5. Start your local validator
6. `anchor deploy`
7. `anchor test`
8. `cat .anchor/program-logs/<your-prog>.log
