const anchor = require("@coral-xyz/anchor");
const { SystemProgram, Keypair, PublicKey } = anchor.web3;
const assert = require("assert");

describe("nft_sales", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.NftSales;
    const wallet = provider.wallet;
    let salePda, saleBump;
    let buyer = Keypair.generate();
    let tokenId = 1;
    let nftPda, nftBump;

    before(async () => {
        // 分配测试 SOL
        await provider.connection.requestAirdrop(buyer.publicKey, 2_000_000_000);
        await new Promise((resolve) => setTimeout(resolve, 1000));
        // 计算 NFT PDA
        [nftPda, nftBump] = await PublicKey.findProgramAddress(
            [Buffer.from("nft"), new anchor.BN(tokenId).toArrayLike(Buffer, "le", 8)],
            program.programId
        );
    });

    it("Mints an NFT", async () => {
        const metadataUri = "https://example.com/nft/1";
        await program.methods
            .mintNft(new anchor.BN(tokenId), metadataUri)
            .accounts({
                nft: nftPda,
                minter: wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        const nftAccount = await program.account.nftState.fetch(nftPda);
        assert.equal(nftAccount.tokenId.toNumber(), tokenId);
        assert.equal(nftAccount.owner.toString(), wallet.publicKey.toString());
        assert.equal(nftAccount.metadataUri, metadataUri);
    });

    it("Lists NFT for sale", async () => {
        const price = new anchor.BN(1_000_000_000);
        [salePda, saleBump] = await PublicKey.findProgramAddress(
            [Buffer.from("sale"), nftPda.toBuffer()],
            program.programId
        );
        await program.methods
            .listNft(price)
            .accounts({
                nft: nftPda,
                sale: salePda,
                seller: wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        const saleAccount = await program.account.saleState.fetch(salePda);
        assert.equal(saleAccount.nftAccount.toString(), nftPda.toString());
        assert.equal(saleAccount.seller.toString(), wallet.publicKey.toString());
        assert.equal(saleAccount.price.toNumber(), price.toNumber());
        assert.equal(saleAccount.active, true);
    });

   

    it("Buys NFT", async () => {
        const buyerBalanceBefore = await provider.connection.getBalance(buyer.publicKey);
        const sellerBalanceBefore = await provider.connection.getBalance(wallet.publicKey);
        await program.methods
            .buyNft()
            .accounts({
                sale: salePda,
                nft: nftPda,
                buyer: buyer.publicKey,
                seller: wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .signers([buyer])
            .rpc();

        const nftAccount = await program.account.nftState.fetch(nftPda);
        assert.equal(nftAccount.owner.toString(), buyer.publicKey.toString());

        const saleAccount = await program.account.saleState.fetch(salePda);
        assert.equal(saleAccount.active, false);

        const buyerBalanceAfter = await provider.connection.getBalance(buyer.publicKey);
        const sellerBalanceAfter = await provider.connection.getBalance(wallet.publicKey);
        assert(buyerBalanceBefore - buyerBalanceAfter >= 1_000_000_000, "Buyer should have paid at least 1 SOL");
        assert(sellerBalanceAfter - sellerBalanceBefore > 900_000_000, "Seller should have received more than 0.9 SOL");
    });
   
});