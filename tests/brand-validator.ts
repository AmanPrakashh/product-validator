import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BrandValidatorOnChain } from "../target/types/brand_validator_on_chain";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("brand-validator-on-chain", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BrandValidatorOnChain as Program<BrandValidatorOnChain>;
  
  // Create test accounts
  const company = provider.wallet.publicKey;
  const unauthorizedCompany = anchor.web3.Keypair.generate();
  const buyer = anchor.web3.Keypair.generate();
  
  // Helper function to find PDA
  const findProductPda = async (owner: PublicKey) => {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("product"), owner.toBuffer()],
      program.programId
    );
    return pda;
  };

  // Fund test accounts
  before(async () => {
    const airdropAmount = 2 * anchor.web3.LAMPORTS_PER_SOL;
    await provider.connection.requestAirdrop(unauthorizedCompany.publicKey, airdropAmount);
    await provider.connection.requestAirdrop(buyer.publicKey, airdropAmount);
  });

  it("Adds a product (authorized company)", async () => {
    const productId = "PROD123";
    const productPda = await findProductPda(company);

    await program.methods
      .addProduct(productId)
      .accounts({
        productAccount: productPda,
        owner: company,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.productAccount.fetch(productPda);
    expect(account.productId).to.equal(productId);
    expect(account.isBuyed).to.be.false;
    expect(account.owner.toString()).to.equal(company.toString());
  });

  it("Fails to add product (unauthorized company)", async () => {
    const productId = "PROD456";
    const productPda = await findProductPda(unauthorizedCompany.publicKey);

    try {
      await program.methods
        .addProduct(productId)
        .accounts({
          productAccount: productPda,
          owner: unauthorizedCompany.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      assert.fail("Should have failed with unauthorized company");
    } catch (error) {
      expect(error).to.be.instanceOf(Error);
    }
  });

  it("Validates a product", async () => {
    const productPda = await findProductPda(company);
    const [productId, isBuyed] = await program.methods
      .validateProduct()
      .accounts({
        productAccount: productPda,
      })
      .view();

    expect(productId).to.equal("PROD123");
    expect(isBuyed).to.be.false;
  });

  it("Buys a product", async () => {
    const productPda = await findProductPda(company);
    
    await program.methods
      .buyProduct("PROD123")
      .accounts({
        productAccount: productPda,
        buyer: buyer.publicKey,
      })
      .signers([buyer])
      .rpc();

    const account = await program.account.productAccount.fetch(productPda);
    expect(account.isBuyed).to.be.true;
  });

  it("Fails to buy already bought product", async () => {
    const productPda = await findProductPda(company);
    
    try {
      await program.methods
        .buyProduct("PROD123")
        .accounts({
          productAccount: productPda,
          buyer: buyer.publicKey,
        })
        .signers([buyer])
        .rpc();
      assert.fail("Should have failed with already bought product");
    } catch (error) {
      expect(error).to.be.instanceOf(Error);
    }
  });

  it("Deletes a product (authorized company)", async () => {
    const productPda = await findProductPda(company);
    
    await program.methods
      .deleteProduct()
      .accounts({
        productAccount: productPda,
        owner: company,
      })
      .rpc();

    const account = await program.account.productAccount.fetchNullable(productPda);
    expect(account).to.be.null;
  });

  it("Fails to delete product (unauthorized company)", async () => {
    const productPda = await findProductPda(company);
    
    try {
      await program.methods
        .deleteProduct()
        .accounts({
          productAccount: productPda,
          owner: unauthorizedCompany.publicKey,
        })
        .signers([unauthorizedCompany])
        .rpc();
      assert.fail("Should have failed with unauthorized company");
    } catch (error) {
      expect(error).to.be.instanceOf(Error);
    }
  });
}); 