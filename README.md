# uniswap-alike-in-polkadot
For studying purpose. Not for usage or distribution

# Local Run Guide

### 1. Install Substrate Node  
Download the latest Substrate Contracts Node from:  
[https://github.com/paritytech/substrate-contracts-node/releases/tag/v0.42.0](https://github.com/paritytech/substrate-contracts-node/releases/tag/v0.42.0)  

---

### 2. Run the Node  
```bash
./substrate-node-template --dev
```
---

### 3. Compile and Build the contract (note to run this in the contracts dir):
```bash
cargo contract build
```
---

### 4. Contract UI 
Navigate to Contract UI in the browser (Brave somehow block contract UI, if you are using Brave, switch to different Browser)

### 5. Upload Contract
- Click upload new contract
- Upload the <name>.contract file in the ~/target/ink
- Instantiate and run


